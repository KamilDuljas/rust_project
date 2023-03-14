fn fibonacci(n: i32) {
    println!("0");
    println!("1");

    let mut f0 = 0;
    let mut f1 = 1;
    let mut next = f0 + f1;

    for _ in 3..=n {
        println!("{}", next);
        f0 = f1;
        f1 = next;
        next = f0 + f1;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    fibonacci(args[1].parse().unwrap());
}
