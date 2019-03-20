fn main() {
    let path = std::env::var("HOME")
        .unwrap_or("");

    println!("{}", path);
}
