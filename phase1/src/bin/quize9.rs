fn main() {
    let x: i32 = 42;
    let y: f64 = 3.14;
    let z: bool = true;
    let c: String = String::from("ABC");

    let x_plus_1 = x + 1;

    println!(
        "x: {}, y: {}, z: {}, c: {}, x_plus_1: {}",
        x, y, z, c, x_plus_1
    );
}
