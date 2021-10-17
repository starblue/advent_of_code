use std::convert::TryFrom;

fn find_password_1(door_id: &[u8]) -> String {
    let mut result = String::new();
    let mut n = 0;
    while result.len() < 8 {
        let mut md5_input = door_id.to_vec();
        md5_input.extend_from_slice(n.to_string().as_bytes());
        let digest = md5::compute(md5_input);
        let s = format!("{:x}", digest);
        if s.starts_with("00000") {
            result.push(s.chars().nth(5).unwrap());
        }
        n += 1;
    }
    result
}

fn partial_password_to_string(cs: &[Option<char>]) -> String {
    cs.iter()
        .map(|opt| match opt {
            None => '-',
            Some(c) => *c,
        })
        .collect::<String>()
}

fn find_password_2(door_id: &[u8]) -> String {
    let mut cs = (0..8).map(|_| None).collect::<Vec<_>>();
    let mut n = 0;
    let mut count = 0;
    while count < 8 {
        let mut md5_input = door_id.to_vec();
        md5_input.extend_from_slice(n.to_string().as_bytes());
        let digest = md5::compute(md5_input);
        let s = format!("{:x}", digest);
        if s.starts_with("00000") {
            let mut iter = s.chars().skip(5);
            let pos = usize::try_from(iter.next().unwrap().to_digit(16).unwrap()).unwrap();
            let c = iter.next().unwrap();
            if (0..8).contains(&pos) && cs[pos] == None {
                cs[pos] = Some(c);
                count += 1;
                println!("{}", partial_password_to_string(&cs));
            }
        }
        n += 1;
    }
    partial_password_to_string(&cs)
}

fn main() {
    let door_id = b"ojvtpuvg";

    let result_a = find_password_1(door_id);
    let result_b = find_password_2(door_id);

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
