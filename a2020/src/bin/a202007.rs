use core::str::FromStr;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::io;
use std::io::Read;

use nom::alt;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::do_parse;
use nom::many1;
use nom::map;
use nom::map_res;
use nom::named;
use nom::opt;
use nom::recognize;
use nom::separated_list1;
use nom::tag;
use nom::value;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Color(String, String);
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ContentItem {
    n: i64,
    color: Color,
}
impl fmt::Display for ContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.n,
            self.color,
            if self.n == 1 { "bag" } else { "bags" }
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Rule {
    color: Color,
    contents: Vec<ContentItem>,
}
impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} bags contain ", self.color)?;

        if self.contents.is_empty() {
            write!(f, "no other bags")?;
        } else {
            let mut sep = "";
            for ci in &self.contents {
                write!(f, "{}{}", sep, ci)?;
                sep = ", ";
            }
        }
        write!(f, ".")
    }
}

named!(int64<&str, i64>,
    map_res!(digit1, FromStr::from_str)
);
named!(word<&str, String>,
    map!(recognize!(alpha1), String::from)
);

named!(color<&str, Color>,
    do_parse!(
        w0: word >>
        tag!(" ") >>
        w1: word >> (Color(w0, w1))
    )
);
named!(content_item<&str, ContentItem>,
    do_parse!(
        n: int64 >>
        tag!(" ") >>
        color: color >>
        tag!(" bag") >>
        opt!(tag!("s")) >> (ContentItem { n, color })
    )
);
named!(contents<&str, Vec<ContentItem>>,
    alt!(
        value!(Vec::new(), tag!("no other bags")) |
        separated_list1!(tag!(", "), content_item)
    )
);
named!(rule<&str, Rule>,
    do_parse!(
        color: color >>
        tag!(" bags contain ") >>
        contents: contents >>
        tag!(".") >>
        line_ending >> (Rule { color, contents })
    )
);
named!(
    input<&str, Vec<Rule>>,
    many1!(rule)
);

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // make nom happy
    input_data.push('\n');

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let rules = result.unwrap().1;
    // for r in rules {
    //     println!("{}", r);
    // }

    let color = Color("shiny".to_string(), "gold".to_string());
    let mut old_colors = HashSet::new();
    let mut new_colors = HashSet::new();
    new_colors.insert(color);
    while !new_colors.is_empty() {
        let working_colors = new_colors;
        new_colors = HashSet::new();

        for c in working_colors.iter() {
            if !old_colors.contains(c) {
                old_colors.insert(c.clone());
                for r in &rules {
                    for ci in &r.contents {
                        if &ci.color == c {
                            new_colors.insert(r.color.clone());
                        }
                    }
                }
            }
        }
    }
    // don't count the original bag
    let result_a = old_colors.len() - 1;

    let rule_map = rules
        .into_iter()
        .map(|r| (r.color, r.contents))
        .collect::<HashMap<_, _>>();

    let color = Color("shiny".to_string(), "gold".to_string());
    let ci = ContentItem { n: 1, color };
    let mut items = vec![ci];
    let mut count = 0;
    while let Some(item) = items.pop() {
        let n = item.n;
        let c = item.color;
        for ci in &rule_map[&c] {
            items.push(ContentItem {
                n: n * ci.n,
                color: ci.color.clone(),
            });
        }
        count += n;
    }
    // don't count the original bag
    let result_b = count - 1;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}

#[cfg(test)]
mod tests {
    use crate::content_item;
    use crate::contents;
    use crate::rule;
    use crate::Color;
    use crate::ContentItem;
    use crate::Rule;

    #[test]
    fn test_content_item() {
        let ci = ContentItem {
            n: 5,
            color: Color("faded".to_string(), "blue".to_string()),
        };
        assert_eq!(ci, content_item("5 faded blue bags,").unwrap().1);
    }

    #[test]
    fn test_contents() {
        let ci0 = ContentItem {
            n: 5,
            color: Color("faded".to_string(), "blue".to_string()),
        };
        let ci1 = ContentItem {
            n: 6,
            color: Color("dotted".to_string(), "black".to_string()),
        };
        let cis = vec![ci0, ci1];
        assert_eq!(
            cis,
            contents("5 faded blue bags, 6 dotted black bags.")
                .unwrap()
                .1
        );
    }

    #[test]
    fn test_rule_no_contents() {
        let r = Rule {
            color: Color("faded".to_string(), "blue".to_string()),
            contents: Vec::new(),
        };
        assert_eq!(
            r,
            rule("faded blue bags contain no other bags.\n").unwrap().1
        );
    }
    #[test]
    fn test_rule_with_contents() {
        let ci0 = ContentItem {
            n: 5,
            color: Color("faded".to_string(), "blue".to_string()),
        };
        let ci1 = ContentItem {
            n: 6,
            color: Color("dotted".to_string(), "black".to_string()),
        };
        let r = Rule {
            color: Color("vibrant".to_string(), "plum".to_string()),
            contents: vec![ci0, ci1],
        };
        assert_eq!(
            r,
            rule("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n")
                .unwrap()
                .1
        );
    }
}
