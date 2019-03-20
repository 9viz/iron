fn fahr(cel: &f32) -> f32 {
    let fahr: f32 = (9.0 / 5.0) * (cel + 32.0);

    fahr
}

//fn cel(fahr: &f32) -> f32 {
//    let cel: f32 = (5.0 / 9.0) * (fahr - 32.0);
//
//    cel
//}

fn main() {
    println!("celsius\tfahr");

    for cel in (100..200).step_by(5) {
        let cel = cel as f32;
        let fahr = fahr(&cel);

        println!("{}\t{}", cel, fahr);
    }
}
