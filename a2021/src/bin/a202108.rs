use core::fmt;

use std::collections::HashMap;
use std::io;

use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::character::complete::space1;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Clone, Debug)]
struct Pattern {
    segments: Vec<char>,
}
impl Pattern {
    fn len(&self) -> usize {
        self.segments.len()
    }
    fn contains(&self, c: &char) -> bool {
        self.segments.contains(c)
    }
    fn segments(&self) -> String {
        let mut segments = self.segments.clone();
        segments.sort();
        segments.into_iter().collect::<String>()
    }
}
impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for s in &self.segments {
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}
impl From<&str> for Pattern {
    fn from(s: &str) -> Self {
        Pattern {
            segments: s.chars().collect::<Vec<_>>(),
        }
    }
}

#[derive(Clone, Debug)]
struct SignalPatterns {
    patterns: Vec<Pattern>,
}
impl SignalPatterns {
    fn spectrum(&self, c: char) -> Vec<usize> {
        let mut spectrum = (0..=7).map(|_| 0).collect::<Vec<_>>();
        for p in &self.patterns {
            if p.contains(&c) {
                spectrum[p.len()] += 1;
            }
        }
        spectrum
    }
}
impl fmt::Display for SignalPatterns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut sep = "";
        for p in &self.patterns {
            write!(f, "{}{}", sep, p)?;
            sep = " ";
        }
        Ok(())
    }
}
impl From<Vec<Pattern>> for SignalPatterns {
    fn from(patterns: Vec<Pattern>) -> SignalPatterns {
        SignalPatterns { patterns }
    }
}

#[derive(Clone, Debug)]
struct Entry {
    signal_patterns: SignalPatterns,
    outputs: Vec<Pattern>,
}
impl Entry {
    fn output_value(&self) -> usize {
        let decoder = Decoder::new(&self.signal_patterns);
        decoder.decode(&self.outputs)
    }
}
impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} |", self.signal_patterns)?;
        for p in &self.outputs {
            write!(f, " {}", p)?;
        }
        Ok(())
    }
}

//  aa
// b  c
// b  c
//  dd
// e  f
// e  f
//  gg
//
//  digit    segments     count
//    0    a b c - e f g    6
//    1    - - c - - f -    2
//    2    a - c d e - g    5
//    3    a - c d - f g    5
//    4    - b c d - f -    4
//    5    a b - d - f g    5
//    6    a b - d e f g    6
//    7    a - c - - f -    3
//    8    a b c d e f g    7
//    9    a b c d - f g    6
//
// spectrum (counts per segment count)
//         a b c d e f g
//    2    0 0 1 0 0 1 0   these occur in 1, which has 2 segments
//    3    1 0 1 0 0 1 0   these occur in 7, which has 3 segments
//    4    0 1 1 1 0 1 0   these occur in 4, which has 4 segments
//    5    3 1 2 3 1 2 3   these occur in 2, 3 and 5, which have 5 segments
//    6    3 3 2 2 2 3 3   these occur in 0, 6 and 9, which have 6 segments
//    7    1 1 1 1 1 1 1   these occur in 8, which has 7 segments
//
// Each segment has a unique spectrum.
// We use these to identify the segments.

const SEGMENTS: [&str; 10] = [
    "abcefg",  // 0
    "cf",      // 1
    "acdeg",   // 2
    "acdfg",   // 3
    "bcdf",    // 4
    "abdfg",   // 5
    "abdefg",  // 6
    "acf",     // 7
    "abcdefg", // 8
    "abcdfg",  // 9
];

#[derive(Clone, Debug)]
struct Decoder {
    // Map from sorted strings of signal segments to digits
    digit_map: HashMap<String, usize>,
}
impl Decoder {
    fn new(signal_patterns: &SignalPatterns) -> Decoder {
        let standard_patterns = SignalPatterns::from(
            SEGMENTS
                .iter()
                .map(|&segments| Pattern::from(segments))
                .collect::<Vec<_>>(),
        );
        // Map from standard segment to spectrum
        let spectrum_map = "abcdefg"
            .chars()
            .map(|c| (c, standard_patterns.spectrum(c)))
            .collect::<HashMap<_, _>>();
        // Map from spectrum to signal segment
        let signal_map = "abcdefg"
            .chars()
            .map(|c| (signal_patterns.spectrum(c), c))
            .collect::<HashMap<_, _>>();
        // Map from signal segments to digit
        let mut digit_map = HashMap::new();
        for (d, standard_segments) in SEGMENTS.iter().enumerate() {
            let mut signal_segments = standard_segments
                .chars()
                .map(|c| signal_map[&spectrum_map[&c]])
                .collect::<Vec<_>>();
            signal_segments.sort();
            let segments = signal_segments.into_iter().collect::<String>();
            digit_map.insert(segments, d);
        }
        Decoder { digit_map }
    }
    fn digit(&self, signal_pattern: &Pattern) -> usize {
        self.digit_map[&signal_pattern.segments()]
    }
    fn decode(&self, digits: &[Pattern]) -> usize {
        let mut result = 0;
        for d in digits {
            result = 10 * result + self.digit(d);
        }
        result
    }
}

fn pattern(i: &str) -> IResult<&str, Pattern> {
    let (i, segments) = many1(one_of("abcdefg"))(i)?;
    Ok((i, Pattern { segments }))
}

fn signal_patterns(i: &str) -> IResult<&str, SignalPatterns> {
    let (i, patterns) = separated_list1(space1, pattern)(i)?;
    Ok((i, SignalPatterns { patterns }))
}

fn entry(i: &str) -> IResult<&str, Entry> {
    let (i, signal_patterns) = signal_patterns(i)?;
    let (i, _) = tag(" | ")(i)?;
    let (i, outputs) = separated_list1(space1, pattern)(i)?;
    Ok((
        i,
        Entry {
            signal_patterns,
            outputs,
        },
    ))
}

fn input(i: &str) -> IResult<&str, Vec<Entry>> {
    separated_list1(line_ending, entry)(i)
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // for entry in &input {
    //     println!("{}", entry);
    // }

    let result_a = input
        .iter()
        .flat_map(|e| &e.outputs)
        .filter(|p| vec![2, 3, 4, 7].contains(&p.len()))
        .count();

    let result_b = input.iter().map(|e| e.output_value()).sum::<usize>();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
