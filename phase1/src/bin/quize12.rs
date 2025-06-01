fn main() {
    let n = 10;
    println!("最初の{}個のフィボナッチ数:", n);

    let mut a = 0;
    let mut b = 1;
    for i in 0..n {
        if i == 0 {
            println!("{}", a);
        } else if i == 1 {
            println!("{}", b);
        } else {
            let next = a + b;
            println!("{}", next);
            a = b;
            b = next
        }
    }
}
