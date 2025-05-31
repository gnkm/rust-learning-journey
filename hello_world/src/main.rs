use indexmap::IndexMap;
use std::collections::HashMap;
use std::fmt::Debug;

fn main() {
    let num1: i8 = 1;
    let num2: f32 = 2.0;
    let num3 = 3.0;

    println!("num1: {}", num1);
    println!("num2: {}", num2);
    println!("num3: {}", num3);

    let b1: bool = true;
    let b2: bool = false;

    println!("b1: {}", b1);
    println!("b2: {}", b2);

    let c1: char = 'a';
    let c2: char = 'b';
    let c3: char = '😀';

    println!("c1: {}", c1);
    println!("c2: {}", c2);
    println!("c3: {}", c3);

    let mut map1: HashMap<String, i8> = HashMap::new();
    map1.insert(String::from("num1"), 1);
    map1.insert(String::from("num2"), 2);
    map1.insert(String::from("num3"), 3);
    map1.insert(String::from("num4"), 4);
    map1.insert(String::from("num5"), 5);
    map1.insert(String::from("num6"), 6);
    map1.insert(String::from("num7"), 7);
    map1.insert(String::from("num8"), 8);
    map1.insert(String::from("num9"), 9);

    for (key, value) in &map1 {
        println!("key: {}, value: {}", key, value);
    }

    let mut map2: IndexMap<String, i8> = IndexMap::new();
    map2.insert(String::from("num1"), 1);
    map2.insert(String::from("num2"), 2);
    map2.insert(String::from("num3"), 3);
    map2.insert(String::from("num4"), 4);
    map2.insert(String::from("num5"), 5);
    map2.insert(String::from("num6"), 6);
    map2.insert(String::from("num7"), 7);
    map2.insert(String::from("num8"), 8);
    map2.insert(String::from("num9"), 9);

    for (key, value) in &map2 {
        println!("key: {}, value: {}", key, value);
    }

    let notmut_str: &str = "イミュータブルな文字列";
    // notmut_str = "Hello, world!!"; // cannot assign twice to immutable variable
    println!("notmut_str: {}", notmut_str);

    let mut mut_str: &str = "ミュータブルな文字列";
    println!("mut_str: {}", mut_str);
    mut_str = "ミュータブルな文字列2";
    println!("mut_str: {}", mut_str);

    println!("add(1, 2): {}", add(1, 2));
    println!("sub(1, 2): {}", sub(1, 2));
    println!("mul(1, 2): {}", mul(1, 2));
    println!("div(1, 2): {}", div(1, 2));
    println!("mod_op(1, 2): {}", mod_op(1, 2));
    println!("pow(1, 2): {}", pow(1, 2));
    println!("sqrt(1.0): {}", sqrt(1.0));
    println!("abs(1.0): {}", abs(1.0));
    println!("max(1, 2): {}", max(1, 2));
    my_print("Hello, world!");
}

fn add(a: i8, b: i8) -> i8 {
    a + b
}

fn sub(a: i8, b: i8) -> i8 {
    a - b
}

fn mul(a: i8, b: i8) -> i8 {
    a * b
}

fn div(a: i8, b: i8) -> i8 {
    a / b
}

fn mod_op(a: i8, b: i8) -> i8 {
    a % b
}

fn pow(a: i8, b: u32) -> i8 {
    a.pow(b)
}

fn sqrt(a: f32) -> f32 {
    a.sqrt()
}

fn abs(a: f32) -> f32 {
    a.abs()
}

fn max(a: i8, b: i8) -> i8 {
    a.max(b)
}

fn my_print(x: impl Debug) {
    println!("{:?}", x)
}
