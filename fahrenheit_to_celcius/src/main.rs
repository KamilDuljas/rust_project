fn f_to_c(f: i32) -> i32 {
    (f - 32) * 5 / 9
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("failed input");
    let buf: i32 = match buf.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(),
    };

    let celcius = f_to_c(buf);
    println!("Farenheit: {}, Celcius: {}", buf, celcius);
}