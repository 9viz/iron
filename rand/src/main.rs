use std::time::{SystemTime, UNIX_EPOCH};

static mut rand: u128 = 1;

fn get_time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("lmao")
        .as_nanos()
}

unsafe fn randi() {
    let a: u128 = get_time();

    let c: u128 = get_time() % 6;
    let m: u128 = 2_u128.pow((get_time() % 50) as u32) + rand / 3;

    rand = (a * rand + c) % m
}

fn main() {
    unsafe {
        for _ in 0..10 {
            randi();
            println!("{:?}", rand);
        }
    }
}
