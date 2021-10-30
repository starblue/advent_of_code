use std::collections::HashMap;
use std::iter::repeat;

fn hash(salt: &[u8], n: usize) -> String {
    let mut md5_input = salt.to_vec();
    md5_input.extend_from_slice(n.to_string().as_bytes());
    let digest = md5::compute(md5_input);
    format!("{:x}", digest)
}

fn stretched_hash(salt: &[u8], n: usize) -> String {
    let mut md5_input = salt.to_vec();
    md5_input.extend_from_slice(n.to_string().as_bytes());
    let mut digest = md5::compute(md5_input);
    for _ in 0..2016 {
        digest = md5::compute(format!("{:x}", digest));
    }
    format!("{:x}", digest)
}

const LOOKAHEAD: usize = 1000;

fn memoize<HF>(hf: &HF, table: &mut HashMap<usize, String>, salt: &[u8], n: usize) -> String
where
    HF: Fn(&[u8], usize) -> String,
{
    if let Some(hash) = table.get(&n) {
        hash.clone()
    } else {
        let hash = hf(salt, n);
        table.insert(n, hash.clone());
        hash
    }
}

fn find_password_index<HF>(salt: &[u8], hf: HF) -> usize
where
    HF: Fn(&[u8], usize) -> String,
{
    let mut hashes = HashMap::new();
    let mut n = 0;
    let mut count = 0;
    loop {
        let hash = memoize(&hf, &mut hashes, salt, n);
        let chars = hash.chars().collect::<Vec<_>>();
        for w in chars.windows(3) {
            if let [c0, c1, c2] = w {
                if c0 == c1 && c1 == c2 {
                    let c5 = repeat(c0).take(5).collect::<String>();
                    for n1 in (n + 1)..=(n + LOOKAHEAD) {
                        let hash1 = memoize(&hf, &mut hashes, salt, n1);
                        if hash1.contains(&c5) {
                            count += 1;

                            if count >= 64 {
                                return n;
                            } else {
                                break;
                            }
                        }
                    }
                    break;
                }
            }
        }
        n += 1;
    }
}

fn main() {
    let salt = b"ngcjuoqr";

    let result_a = find_password_index(salt, hash);
    let result_b = find_password_index(salt, stretched_hash);

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
