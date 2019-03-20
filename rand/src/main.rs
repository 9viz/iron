use std::time::{SystemTime, UNIX_EPOCH};

static mut rand: u64 = 1;

fn get_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("lmao")
        .subsec_micros() as u64
}

unsafe fn randi() {
    let a: u64 = get_time();

    let c: u64 = get_time() % 6;
    let m: u64 = 2_u64.pow((get_time() % 50) as u32) + rand / 3;

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
