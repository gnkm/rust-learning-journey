fn main() {
    let width: f64 = 3.0;
    let height: f64 = 4.0;
    let area: f64 = calculate_area(width, height);
    println!("長方形の面積は: {}", area);
}

fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}
