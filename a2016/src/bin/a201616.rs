fn step(input: &[u8]) -> Vec<u8> {
    let mut result = input.to_vec();
    result.push(b'0');
    let mut i = input.len() - 1;
    loop {
        let c_in = input[i];
        let c_out = if c_in == b'0' { b'1' } else { b'0' };
        result.push(c_out);
        if i == 0 {
            break;
        }
        i -= 1;
    }
    result
}

fn disk_data(len: usize, input: &[u8]) -> Vec<u8> {
    let mut data = input.to_vec();
    while data.len() < len {
        data = step(&data);
    }
    data.truncate(len);
    data
}

fn checksum(data: &[u8]) -> String {
    let mut data = data.to_vec();
    while data.len() % 2 == 0 {
        let mut new_data = Vec::new();
        for chunk in data.chunks(2) {
            if let [b0, b1] = chunk {
                let new_b = if b0 == b1 { b'1' } else { b'0' };
                new_data.push(new_b);
            }
        }
        data = new_data;
    }
    data.into_iter().map(|b| b as char).collect::<String>()
}

fn main() {
    let input = b"10001001100000001";

    let disk_len1 = 272;
    let data = disk_data(disk_len1, input);
    let result_a = checksum(&data);

    let disk_len2 = 35651584;
    let data = disk_data(disk_len2, input);
    let result_b = checksum(&data);

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}

#[cfg(test)]
mod tests {
    use crate::step;

    #[test]
    fn test_1() {
        assert_eq!(b"100".to_vec(), step(b"1"));
    }
    #[test]
    fn test_0() {
        assert_eq!(b"001".to_vec(), step(b"0"));
    }
    #[test]
    fn test_11111() {
        assert_eq!(b"11111000000".to_vec(), step(b"11111"));
    }
    #[test]
    fn test_111100001010() {
        assert_eq!(b"1111000010100101011110000".to_vec(), step(b"111100001010"));
    }
}
