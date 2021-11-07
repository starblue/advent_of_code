use std::collections::VecDeque;

fn main() {
    let input = 3012210;

    let mut have_presents = (1..=input).collect::<VecDeque<_>>();
    while have_presents.len() >= 2 {
        let taker = have_presents.pop_front().unwrap();
        let _giver = have_presents.pop_front().unwrap();
        have_presents.push_back(taker);
    }
    let result_a = have_presents.pop_front().unwrap();

    let mut have_presents_h0 = VecDeque::new();
    let mut have_presents_h1 = (1..=input).collect::<VecDeque<_>>();
    while have_presents_h0.len() + have_presents_h1.len() >= 2 {
        // Balance the halves so that they are either equal
        // or the second half is one larger.
        while have_presents_h1.len() - have_presents_h0.len() >= 2 {
            have_presents_h0.push_back(have_presents_h1.pop_front().unwrap());
        }
        let taker = have_presents_h0.pop_front().unwrap();
        let _giver = have_presents_h1.pop_front().unwrap();
        have_presents_h1.push_back(taker);
    }
    let result_b = have_presents_h1.pop_front().unwrap();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
