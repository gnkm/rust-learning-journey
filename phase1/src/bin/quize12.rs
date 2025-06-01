fn main() {
    let n = 10;
    println!("最初の{}個のフィボナッチ数:", n);

    let mut a = 0;
    let b = 1;
    for _i in 0..n {
        println!("{}", a);
        let _next = a + b;
        a = b;
    }
}
