fn main() {
    let mut a: i32 = 0;
    let mut b: i32 = 1;

    for _ in 0..20 {
        b += a;
        a += b;

        println!("{}\n{}", b, a);
    }
}
