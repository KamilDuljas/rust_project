fn main() {
    //let s = "hello world";
    let s = String::from("Hello String");

    let word = first_world(&s);
    println!("Word is {}", word);

    //s.clear();
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}
