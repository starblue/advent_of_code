use std::collections::HashSet;
use std::io;

fn all_different(chars: &[char]) -> bool {
    let set = chars.iter().collect::<HashSet<_>>();
    set.len() == chars.len()
}

fn start_pos(s: &str, n: usize) -> usize {
    let chars = s.chars().collect::<Vec<_>>();
    chars.windows(n).take_while(|w| !all_different(w)).count() + n
}

fn start_of_packet(s: &str) -> usize {
    start_pos(s, 4)
}

fn start_of_message(s: &str) -> usize {
    start_pos(s, 14)
}

fn main() -> util::Result<()> {
    let input = io::read_to_string(io::stdin())?;

    let result1 = start_of_packet(&input);

    let result2 = start_of_message(&input);

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::start_of_message;
    use crate::start_of_packet;

    #[test]
    fn test_start_of_packet() {
        assert_eq!(7, start_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, start_of_packet("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn test_start_of_message() {
        assert_eq!(19, start_of_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, start_of_message("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, start_of_message("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, start_of_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, start_of_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
