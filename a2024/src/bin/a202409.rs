use core::cmp;
use core::fmt;

use std::collections::BinaryHeap;
use std::io;

use nom::character::complete::satisfy;
use nom::multi::many1;
use nom::IResult;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug)]
struct Item {
    digit: usize,
}
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.digit)
    }
}

fn item(i: &str) -> IResult<&str, Item> {
    let (i, c) = satisfy(|c| c.is_ascii_digit())(i)?;
    let digit = c.to_digit(10).unwrap() as usize;
    Ok((i, Item { digit }))
}

fn input(i: &str) -> IResult<&str, Vec<Item>> {
    many1(item)(i)
}

#[derive(Clone, Copy, Debug)]
struct UsedSpace {
    id: usize,
    index: usize,
    len: usize,
}
impl UsedSpace {
    fn checksum(&self) -> usize {
        self.id * (2 * self.index + self.len - 1) * self.len / 2
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FreeSpace {
    index: usize,
    len: usize,
}
impl PartialOrd for FreeSpace {
    fn partial_cmp(&self, other: &FreeSpace) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for FreeSpace {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.index.cmp(&self.index).then(self.len.cmp(&other.len))
    }
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for item in &input {
    //     print!("{}", item);
    // }
    // println!();

    // Build the map.
    let mut map = Vec::new();
    let mut next_id = 0;
    let mut is_file = true;
    for item in &input {
        let block = {
            if is_file {
                let id = next_id;
                next_id += 1;
                Some(id)
            } else {
                None
            }
        };
        for _ in 0..item.digit {
            map.push(block);
        }
        is_file = !is_file;
    }

    // Compact the files.
    let mut map1 = map.clone();
    let mut low = 0;
    let mut high = map1.len() - 1;
    loop {
        while low < high && map1[low].is_some() {
            low += 1;
        }
        while high > low && map1[high].is_none() {
            high -= 1;
        }
        if low >= high {
            break;
        }
        map1.swap(low, high);
    }

    // Compute the checksum.
    let result1 = map1
        .iter()
        .enumerate()
        .map(|(i, block)| if let Some(id) = block { i * id } else { 0 })
        .sum::<usize>();

    // Build an index of used and free spaces.
    let mut id = 0;
    let mut used_spaces = Vec::new();
    let mut free_spaces = (0..=9).map(|_| BinaryHeap::new()).collect::<Vec<_>>();
    let mut index = 0;
    let mut is_file = true;
    for item in &input {
        let len = item.digit;
        if is_file {
            used_spaces.push(UsedSpace { id, index, len });
            id += 1;
        } else {
            free_spaces[len].push(FreeSpace { index, len });
        }
        index += len;
        is_file = !is_file;
    }

    // Compact the files.
    for used_space in used_spaces.iter_mut().rev() {
        let mut leftmost_free_space: Option<FreeSpace> = None;
        for i in used_space.len..=9 {
            if let Some(&free_space) = free_spaces[i].peek() {
                if free_space.index < used_space.index {
                    if leftmost_free_space.is_none()
                        || free_space.index < leftmost_free_space.unwrap().index
                    {
                        leftmost_free_space = Some(free_space);
                    }
                }
            }
        }
        if let Some(free_space) = leftmost_free_space {
            // Remove free space.
            let i = free_space.len;
            free_spaces[i].pop();

            // Move used space into free space.
            used_space.index = free_space.index;

            // Add back any remaining free space.
            let len = free_space.len - used_space.len;
            if len > 0 {
                let index = free_space.index + used_space.len;
                free_spaces[len].push(FreeSpace { index, len });
            }
        }
    }

    // Compute the checksum.
    let result2 = used_spaces
        .iter()
        .map(|used_space| used_space.checksum())
        .sum::<usize>();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
