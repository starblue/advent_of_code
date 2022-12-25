use core::cell::Cell;
use core::cell::RefCell;
use core::fmt;
use core::str::FromStr;

use std::collections::HashMap;
use std::io;
use std::rc::Rc;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::none_of;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::multi::many0;
use nom::multi::many1;
use nom::IResult;

use util::runtime_error;

#[derive(Clone, Debug)]
enum Path {
    Down { dir_name: String },
    Up,
    Root,
}
impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Path::Down { dir_name } => write!(f, "{}", dir_name),
            Path::Up => write!(f, ".."),
            Path::Root => write!(f, "/"),
        }
    }
}

#[derive(Clone, Debug)]
struct FileEntry {
    name: String,
    size: usize,
}
impl fmt::Display for FileEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} {}", self.size, self.name)
    }
}

#[derive(Clone, Debug)]
struct DirEntry {
    name: String,
}
impl fmt::Display for DirEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "dir {}", self.name)
    }
}

#[derive(Clone, Debug)]
enum Entry {
    File(FileEntry),
    Dir(DirEntry),
}
impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Entry::File(file_entry) => write!(f, "{}", file_entry),
            Entry::Dir(dir_entry) => write!(f, "{}", dir_entry),
        }
    }
}

#[derive(Clone, Debug)]
struct DirListing {
    entries: Vec<Entry>,
}

impl fmt::Display for DirListing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for entry in &self.entries {
            write!(f, "{}", entry)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
enum Command {
    Cd(Path),
    Ls(DirListing),
}
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Cd(path) => writeln!(f, "$ cd {}", path),
            Command::Ls(dir_listing) => write!(f, "$ ls\n{}", dir_listing),
        }
    }
}

fn int(i: &str) -> IResult<&str, usize> {
    map_res(digit1, FromStr::from_str)(i)
}

fn name(i: &str) -> IResult<&str, String> {
    map(recognize(many1(none_of("/ \t\r\n"))), String::from)(i)
}

fn path_down(i: &str) -> IResult<&str, Path> {
    let (i, dir_name) = name(i)?;
    Ok((i, Path::Down { dir_name }))
}
fn path_up(i: &str) -> IResult<&str, Path> {
    value(Path::Up, tag(".."))(i)
}
fn path_root(i: &str) -> IResult<&str, Path> {
    value(Path::Root, tag("/"))(i)
}
fn path(i: &str) -> IResult<&str, Path> {
    // Put path_down last because of ambiguity with ".."
    alt((path_up, path_root, path_down))(i)
}

fn file_entry(i: &str) -> IResult<&str, FileEntry> {
    let (i, size) = int(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, name) = name(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, FileEntry { size, name }))
}
fn dir_entry(i: &str) -> IResult<&str, DirEntry> {
    let (i, _) = tag("dir ")(i)?;
    let (i, name) = name(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, DirEntry { name }))
}

fn entry_file(i: &str) -> IResult<&str, Entry> {
    let (i, e) = file_entry(i)?;
    Ok((i, Entry::File(e)))
}
fn entry_dir(i: &str) -> IResult<&str, Entry> {
    let (i, e) = dir_entry(i)?;
    Ok((i, Entry::Dir(e)))
}
fn entry(i: &str) -> IResult<&str, Entry> {
    alt((entry_file, entry_dir))(i)
}

fn dir_listing(i: &str) -> IResult<&str, DirListing> {
    let (i, entries) = many0(entry)(i)?;
    Ok((i, DirListing { entries }))
}

fn command_cd(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ cd ")(i)?;
    let (i, path) = path(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Command::Cd(path)))
}
fn command_ls(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ls")(i)?;
    let (i, _) = line_ending(i)?;
    let (i, listing) = dir_listing(i)?;
    Ok((i, Command::Ls(listing)))
}
fn command(i: &str) -> IResult<&str, Command> {
    alt((command_cd, command_ls))(i)
}

fn input(i: &str) -> IResult<&str, Vec<Command>> {
    many0(command)(i)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct File {
    /// The size of the file.
    size: usize,
}
impl File {
    fn fmt_indented(&self, f: &mut fmt::Formatter, name: &str, indent: &str) -> fmt::Result {
        writeln!(f, "{}- {} (file, size={})", indent, name, self.size)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Dir {
    /// The items contained in the directory, or `None` if they are unknown.
    entries: RefCell<Option<HashMap<String, Item>>>,
    /// The parent directory, or `None` for the root.
    parent: Option<Rc<Dir>>,
    /// The total size of the directory, or `None` if it is unknown.
    size: Cell<Option<usize>>,
}
impl Dir {
    fn root() -> Rc<Dir> {
        Rc::new(Dir {
            entries: RefCell::new(None),
            parent: None,
            size: Cell::new(None),
        })
    }
    fn new(parent: &Rc<Dir>) -> Rc<Dir> {
        Rc::new(Dir {
            entries: RefCell::new(None),
            parent: Some(parent.clone()),
            size: Cell::new(None),
        })
    }
    fn lookup(&self, name: &str) -> Option<Item> {
        self.entries
            .borrow()
            .as_ref()
            .and_then(|entries| entries.get(name).cloned())
    }
    fn subdirs(&self) -> util::Result<Vec<Rc<Dir>>> {
        if let Some(entries) = &mut *self.entries.borrow_mut() {
            Ok(entries
                .iter()
                .flat_map(|(_name, item)| {
                    if let Item::Dir(dir) = item {
                        Some(dir)
                    } else {
                        None
                    }
                })
                .cloned()
                .collect::<Vec<_>>())
        } else {
            Err(runtime_error!("unknown directory contents"))
        }
    }
    fn total_size(&self) -> util::Result<usize> {
        if let Some(result) = self.size.get() {
            Ok(result)
        } else {
            let result = match &mut *self.entries.borrow_mut() {
                None => return Err(runtime_error!("unknown directory contents")),
                Some(entries) => entries
                    .iter_mut()
                    .map(|(_name, item)| item.total_size())
                    .try_fold(0_usize, |a, r| r.map(|s| a + s))?,
            };
            self.size.set(Some(result));
            Ok(result)
        }
    }
    fn fmt_indented(&self, f: &mut fmt::Formatter, name: &str, indent: &str) -> fmt::Result {
        write!(f, "{}- {} (dir", indent, name)?;
        if let Some(size) = self.size.get() {
            write!(f, ", size={}", size)?;
        }
        writeln!(f, ")")?;
        if let Some(entries) = self.entries.borrow().as_ref() {
            let mut new_indent = indent.to_string();
            new_indent.push_str("  ");
            for (name, item) in entries {
                item.fmt_indented(f, name, &new_indent)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Item {
    File(File),
    Dir(Rc<Dir>),
}
impl Item {
    fn total_size(&mut self) -> util::Result<usize> {
        match self {
            Item::File(file) => Ok(file.size),
            Item::Dir(dir) => dir.total_size(),
        }
    }
    fn fmt_indented(&self, f: &mut fmt::Formatter, name: &str, indent: &str) -> fmt::Result {
        match self {
            Item::File(file) => file.fmt_indented(f, name, indent),
            Item::Dir(dir) => dir.fmt_indented(f, name, indent),
        }
    }
}

#[derive(Clone, Debug)]
struct FileSystem {
    root: Rc<Dir>,
    working_dir: Rc<Dir>,
}
impl FileSystem {
    fn new() -> FileSystem {
        let root = Dir::root();
        let working_dir = root.clone();
        FileSystem { root, working_dir }
    }
    fn dirs(&self) -> util::Result<Vec<Rc<Dir>>> {
        let mut result = Vec::new();
        let mut stack = vec![self.root.clone()];
        while let Some(dir) = stack.pop() {
            result.push(dir.clone());
            let subdirs = &mut dir.subdirs()?;
            stack.append(subdirs);
        }
        Ok(result)
    }
    fn execute(&mut self, command: &Command) -> util::Result<()> {
        match command {
            Command::Cd(path) => self.execute_cd(path),
            Command::Ls(listing) => self.execute_ls(listing),
        }
    }
    fn execute_cd(&mut self, path: &Path) -> util::Result<()> {
        match path {
            Path::Down { dir_name } => {
                if let Item::Dir(dir) =
                    self.working_dir.lookup(dir_name).ok_or(runtime_error!(
                        "directory {} not found",
                        dir_name
                    ))?
                {
                    self.working_dir = dir;
                } else {
                    return Err(runtime_error!("not a directory"));
                }
                Ok(())
            }
            Path::Up => {
                self.working_dir = self
                    .working_dir
                    .parent
                    .clone()
                    .ok_or_else(|| runtime_error!("root has no parent directory"))?;
                Ok(())
            }
            Path::Root => {
                self.working_dir = self.root.clone();
                Ok(())
            }
        }
    }
    fn execute_ls(&mut self, listing: &DirListing) -> util::Result<()> {
        let mut entries = HashMap::new();
        for entry in &listing.entries {
            match entry {
                Entry::File(file_entry) => {
                    entries.insert(
                        file_entry.name.clone(),
                        Item::File(File {
                            size: file_entry.size,
                        }),
                    );
                }
                Entry::Dir(dir_entry) => {
                    entries.insert(
                        dir_entry.name.clone(),
                        Item::Dir(Dir::new(&self.working_dir)),
                    );
                }
            }
        }
        *self.working_dir.entries.borrow_mut() = Some(entries);
        Ok(())
    }
}
impl fmt::Display for FileSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.root.fmt_indented(f, "/", "")
    }
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for command in &input {
    //     print!("{}", command);
    // }

    let mut fs = FileSystem::new();
    for command in &input {
        fs.execute(command)?;
    }
    let used_space = fs.root.total_size()?;
    let mut sum = 0;
    for dir in fs.dirs()? {
        let size = dir.total_size()?;
        if size <= 100_000 {
            sum += size
        }
    }
    let result1 = sum;

    let total_space = 70_000_000;
    let needed_space = 30_000_000;
    let available_space = total_space - used_space;
    let space_to_free = needed_space - available_space;
    let mut min_dir_size = usize::MAX;
    for dir in fs.dirs()? {
        let size = dir.total_size()?;
        if size >= space_to_free && size < min_dir_size {
            min_dir_size = size
        }
    }
    let result2 = min_dir_size;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
