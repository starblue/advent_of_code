use std::io;

fn main() {
    let mut line = String::new();

    let mut ns = Vec::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line).expect("I/O error");

        let v: usize = match line.trim().parse() {
            Result::Ok(mass) => mass,
            Result::Err(_) => break,
        };
        ns.push(v);
    }

    // add jolts of outlet
    ns.push(0);
    // add jolts of device
    let &max = ns.iter().max().unwrap();
    ns.push(max + 3);

    ns.sort();

    let mut count_d1 = 0;
    let mut count_d3 = 0;
    for i in 0..(ns.len() - 1) {
        let d = ns[i + 1] - ns[i];
        if d == 1 {
            count_d1 += 1;
        } else if d == 3 {
            count_d3 += 1;
        } else {
            // do nothing
        }
    }

    let result_a = count_d1 * count_d3;

    // ways[i] = number of ways to reach i jolts
    let mut ways = (0..=max + 3).map(|_| 0).collect::<Vec<i64>>();
    ways[0] = 1;
    for &n in ns.iter().skip(1) {
        let mut ws = 0;
        if n >= 1 {
            ws += ways[n - 1];
        }
        if n >= 2 {
            ws += ways[n - 2];
        }
        if n >= 3 {
            ws += ways[n - 3];
        }
        ways[n] = ws;
        println!("{} {}", n, ws);
    }

    let result_b = ways[max + 3];

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
