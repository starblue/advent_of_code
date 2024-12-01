use core::cmp;
use core::fmt;
use core::ops;
use core::ops::Range;
use core::str::FromStr;

use std::collections::HashMap;
use std::io;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::IResult;

use util::runtime_error;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Label(String);
impl Label {
    fn start() -> Label {
        Label("in".to_string())
    }
}
impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
fn label(i: &str) -> IResult<&str, Label> {
    let (i, s) = alpha1(i)?;
    Ok((i, Label(s.to_string())))
}

#[derive(Clone, Copy, Debug)]
enum Category {
    X,
    M,
    A,
    S,
}
impl Category {
    fn to_char(self) -> char {
        match self {
            Category::X => 'x',
            Category::M => 'm',
            Category::A => 'a',
            Category::S => 's',
        }
    }
}
impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
fn category(i: &str) -> IResult<&str, Category> {
    alt((
        value(Category::X, tag("x")),
        value(Category::M, tag("m")),
        value(Category::A, tag("a")),
        value(Category::S, tag("s")),
    ))(i)
}

#[derive(Clone, Copy, Debug)]
struct Condition {
    category: Category,
    ordering: cmp::Ordering,
    value: i64,
}
impl Condition {
    fn ordering_char(self) -> char {
        match self.ordering {
            cmp::Ordering::Less => '<',
            cmp::Ordering::Equal => '=',
            cmp::Ordering::Greater => '>',
        }
    }
}
impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.category, self.ordering_char(), self.value)
    }
}
fn ordering(i: &str) -> IResult<&str, cmp::Ordering> {
    alt((
        value(cmp::Ordering::Less, tag("<")),
        value(cmp::Ordering::Equal, tag("=")),
        value(cmp::Ordering::Greater, tag(">")),
    ))(i)
}
fn uint(i: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}
fn condition(i: &str) -> IResult<&str, Condition> {
    let (i, category) = category(i)?;
    let (i, ordering) = ordering(i)?;
    let (i, value) = uint(i)?;
    Ok((
        i,
        Condition {
            category,
            ordering,
            value,
        },
    ))
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Action {
    Accept,
    Reject,
    Goto(Label),
}
impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Accept => write!(f, "A"),
            Action::Reject => write!(f, "R"),
            Action::Goto(label) => write!(f, "{}", label),
        }
    }
}
fn action_goto(i: &str) -> IResult<&str, Action> {
    let (i, name) = label(i)?;
    Ok((i, Action::Goto(name)))
}
fn action(i: &str) -> IResult<&str, Action> {
    alt((
        value(Action::Accept, tag("A")),
        value(Action::Reject, tag("R")),
        action_goto,
    ))(i)
}

#[derive(Clone, Debug)]
struct Rule {
    condition: Option<Condition>,
    action: Action,
}
impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(condition) = &self.condition {
            write!(f, "{}:", condition)?;
        }
        write!(f, "{}", self.action)
    }
}
fn condition_colon(i: &str) -> IResult<&str, Condition> {
    let (i, condition) = condition(i)?;
    let (i, _) = tag(":")(i)?;
    Ok((i, condition))
}
fn rule(i: &str) -> IResult<&str, Rule> {
    let (i, condition) = opt(condition_colon)(i)?;
    let (i, action) = action(i)?;
    Ok((i, Rule { condition, action }))
}

#[derive(Clone, Debug)]
struct Workflow {
    name: Label,
    rules: Vec<Rule>,
}
impl Workflow {
    fn action(&self, part: &Part) -> util::Result<Action> {
        for rule in &self.rules {
            if part.satisfies(&rule.condition) {
                return Ok(rule.action.clone());
            }
        }
        Err(runtime_error!("workflow {} failed", self.name))
    }
}
impl fmt::Display for Workflow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{{", self.name)?;
        let mut sep = "";
        for rule in &self.rules {
            write!(f, "{}{}", sep, rule)?;
            sep = ",";
        }
        write!(f, "}}")
    }
}
fn workflow(i: &str) -> IResult<&str, Workflow> {
    let (i, name) = label(i)?;
    let (i, _) = tag("{")(i)?;
    let (i, rules) = separated_list1(tag(","), rule)(i)?;
    let (i, _) = tag("}")(i)?;
    Ok((i, Workflow { name, rules }))
}

#[derive(Clone, Copy, Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}
impl Part {
    fn satisfies(&self, condition: &Option<Condition>) -> bool {
        if let Some(Condition {
            category,
            ordering,
            value,
        }) = condition
        {
            self[category].cmp(value) == *ordering
        } else {
            true
        }
    }
    fn rating_sum(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}
impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{x={},m={},a={},s={}}}", self.x, self.m, self.a, self.s)
    }
}
impl ops::Index<&Category> for Part {
    type Output = i64;
    fn index(&self, c: &Category) -> &i64 {
        match c {
            Category::X => &self.x,
            Category::M => &self.m,
            Category::A => &self.a,
            Category::S => &self.s,
        }
    }
}
fn part(i: &str) -> IResult<&str, Part> {
    let (i, _) = tag("{x=")(i)?;
    let (i, x) = uint(i)?;
    let (i, _) = tag(",m=")(i)?;
    let (i, m) = uint(i)?;
    let (i, _) = tag(",a=")(i)?;
    let (i, a) = uint(i)?;
    let (i, _) = tag(",s=")(i)?;
    let (i, s) = uint(i)?;
    let (i, _) = tag("}")(i)?;
    Ok((i, Part { x, m, a, s }))
}

#[derive(Clone, Debug)]
struct Input {
    workflow_names: Vec<Label>,
    workflow_map: HashMap<Label, Workflow>,
    parts: Vec<Part>,
}
impl Input {
    fn new(workflows: Vec<Workflow>, parts: Vec<Part>) -> Input {
        let mut workflow_names = Vec::new();
        let mut workflow_map = HashMap::new();
        for workflow in workflows {
            workflow_names.push(workflow.name.clone());
            workflow_map.insert(workflow.name.clone(), workflow);
        }
        Input {
            workflow_names,
            workflow_map,
            parts,
        }
    }
    fn accepts(&self, part: &Part) -> util::Result<bool> {
        let mut workflow_name = Label::start();
        loop {
            let workflow = &self.workflow_map[&workflow_name];
            let action = workflow.action(part)?;
            if let Action::Goto(name) = action {
                workflow_name = name;
            } else {
                return Ok(action == Action::Accept);
            }
        }
    }
    fn count_accepted(&self) -> util::Result<i64> {
        let workflow_name = Label::start();
        let workflow_pos = 0;
        let parts = PartRange::all();
        let initial_state = State {
            parts,
            workflow_name,
            workflow_pos,
        };
        let mut stack = vec![initial_state];

        let mut count = 0;
        while let Some(state) = stack.pop() {
            if let Some(workflow) = &self.workflow_map.get(&state.workflow_name) {
                if let Some(rule) = workflow.rules.get(state.workflow_pos) {
                    let (parts_t, parts_f) = state.parts.split(&rule.condition);
                    if let Some(parts) = parts_t {
                        // Handle the case that the condition is true.
                        if let Action::Goto(name) = &rule.action {
                            let workflow_name = name.clone();
                            let workflow_pos = 0;
                            stack.push(State {
                                parts,
                                workflow_name,
                                workflow_pos,
                            })
                        } else if rule.action == Action::Accept {
                            count += parts.count();
                        } else {
                            // Reject, these don't count.
                        }
                    }
                    if let Some(parts) = parts_f {
                        // Handle the case that the condition is false.
                        let workflow_name = state.workflow_name;
                        let workflow_pos = state.workflow_pos + 1;
                        stack.push(State {
                            parts,
                            workflow_name,
                            workflow_pos,
                        })
                    }
                } else {
                    // We fell off the end of the workflow.
                    return Err(runtime_error!("workflow {} failed", workflow));
                }
            } else {
                // Unknown workflow, ignore.
            }
        }
        Ok(count)
    }
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for name in &self.workflow_names {
            let workflow = &self.workflow_map[name];
            writeln!(f, "{}", workflow)?;
        }
        writeln!(f)?;
        for part in &self.parts {
            writeln!(f, "{}", part)?;
        }
        Ok(())
    }
}
fn input(i: &str) -> IResult<&str, Input> {
    let (i, workflows) = separated_list1(line_ending, workflow)(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = line_ending(i)?;
    let (i, parts) = separated_list1(line_ending, part)(i)?;
    Ok((i, Input::new(workflows, parts)))
}

#[derive(Clone, Debug)]
struct PartRange {
    x: Range<i64>,
    m: Range<i64>,
    a: Range<i64>,
    s: Range<i64>,
}
impl PartRange {
    fn all() -> PartRange {
        let full_range = 1..4001;
        PartRange {
            x: full_range.clone(),
            m: full_range.clone(),
            a: full_range.clone(),
            s: full_range.clone(),
        }
    }
    fn split(&self, condition: &Option<Condition>) -> (Option<PartRange>, Option<PartRange>) {
        fn split_range(
            range: &Range<i64>,
            ordering: cmp::Ordering,
            value: i64,
        ) -> (Option<Range<i64>>, Option<Range<i64>>) {
            match ordering {
                cmp::Ordering::Less => {
                    if range.end <= value {
                        // The condition is satisfied for the whole range.
                        (Some(range.clone()), None)
                    } else if range.start >= value {
                        // The condition is satisfied nowhere the in range.
                        (None, Some(range.clone()))
                    } else {
                        // The condition is satisfied for part of the range.
                        (Some(range.start..value), Some(value..range.end))
                    }
                }
                cmp::Ordering::Greater => {
                    // It's simpler to use an equivalent condition with >=.
                    let value = value + 1;
                    if range.start >= value {
                        // The condition is satisfied for the whole range.
                        (Some(range.clone()), None)
                    } else if range.end <= value {
                        // The condition is satisfied nowhere the in range.
                        (None, Some(range.clone()))
                    } else {
                        // The condition is satisfied for part of the range.
                        (Some(value..range.end), Some(range.start..value))
                    }
                }
                cmp::Ordering::Equal => {
                    // Conditions using equality are not allowed here.
                    unimplemented!()
                }
            }
        }

        if let Some(Condition {
            category,
            ordering,
            value,
        }) = condition
        {
            let (opt_range_t, opt_range_f) = split_range(&self[category], *ordering, *value);
            (
                opt_range_t.map(|r| {
                    let mut pr = self.clone();
                    pr[category] = r;
                    pr
                }),
                opt_range_f.map(|r| {
                    let mut pr = self.clone();
                    pr[category] = r;
                    pr
                }),
            )
        } else {
            // Unconditional, satisfied for the whole range.
            (Some(self.clone()), None)
        }
    }
    fn count(&self) -> i64 {
        let dx = self.x.end - self.x.start;
        let dm = self.m.end - self.m.start;
        let da = self.a.end - self.a.start;
        let ds = self.s.end - self.s.start;
        dx * dm * da * ds
    }
}
impl fmt::Display for PartRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{x={}..{},m={}..{},a={}..{},s={}..{}}}",
            self.x.start,
            self.x.end,
            self.m.start,
            self.m.end,
            self.a.start,
            self.a.end,
            self.s.start,
            self.s.end
        )
    }
}
impl ops::Index<&Category> for PartRange {
    type Output = Range<i64>;
    fn index(&self, c: &Category) -> &Range<i64> {
        match c {
            Category::X => &self.x,
            Category::M => &self.m,
            Category::A => &self.a,
            Category::S => &self.s,
        }
    }
}
impl ops::IndexMut<&Category> for PartRange {
    fn index_mut(&mut self, c: &Category) -> &mut Range<i64> {
        match c {
            Category::X => &mut self.x,
            Category::M => &mut self.m,
            Category::A => &mut self.a,
            Category::S => &mut self.s,
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    parts: PartRange,
    workflow_name: Label,
    workflow_pos: usize,
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let result1 = input
        .parts
        .iter()
        .filter(|p| input.accepts(p).unwrap())
        .map(|p| p.rating_sum())
        .sum::<i64>();

    let result2 = input.count_accepted()?;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
