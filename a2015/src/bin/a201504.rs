fn find_coin(key: &[u8], start: &str) -> usize {
    let mut n = 1;
    loop {
        let mut md5_input = key.to_vec();
        md5_input.extend_from_slice(n.to_string().as_bytes());
        let digest = md5::compute(md5_input);
        let s = format!("{:x}", digest);
        if s.starts_with(start) {
            break;
        }
        n += 1;
    }
    n
}

fn main() {
    let key = b"yzbqklnj";

    let result_a = find_coin(key, "00000");
    let result_b = find_coin(key, "000000");

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
