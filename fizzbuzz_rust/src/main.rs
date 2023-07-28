use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: i32 = args.get(1).unwrap().parse().unwrap();
    fizzbuzz(n);
}

fn fizzbuzz(n: i32) {
    for i in 1..n + 1 {
        if i % 15 == 0 {
            println!("fizzbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }
}
