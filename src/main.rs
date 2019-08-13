use rand::{Rng, thread_rng, seq::SliceRandom};
use std::io::BufRead;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: usize = if args.len() > 1 {
        let n = args[1].parse().unwrap();
        if n > 14 {
            panic!("too many lights, 14 max");
        }
        n
    } else {
        5
    };
    let mut rng = thread_rng();
    let mut patterns = vec![];
    for i in 0..n {
        let mut pat: u16 = rng.gen();
        if pat & (1 << i) == 0 {
            pat ^= !0;
        }
        pat &= (1 << n) - 1;
        patterns.push(pat);
    }
    let mut goal = 0;
    for _ in 0..100 {
        let step = *patterns.choose(&mut rng).unwrap();
        goal ^= step;
    }
    let lines = std::io::BufReader::new(std::io::stdin());
    let mut lines = lines.lines();
    println!("lights out: enter light number from 1 to {} and press return", n);
    while goal != 0 {
        let mut lights = String::new();
        for i in 0..n {
            match (goal >> i) & 1 {
                0 => lights.push('-'),
                1 => lights.push('*'),
                _ => panic!("internal error: bad bit"),
            }
        }
        println!("{}", lights);
        let choice: usize = lines.next().unwrap().unwrap().parse().unwrap();
        if choice < 1 || choice > n {
            println!("nope");
            continue;
        }
        goal ^= patterns[choice - 1];
    }
    println!("gratz!");
}
