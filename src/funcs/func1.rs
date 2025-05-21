pub fn add(a: i32, b: i32) -> () {
    let c: i32 = a + b;
    println!("Summing a={} and b={}, equalling c={}", a, b, c);
}


pub fn add10(a: &mut i32) -> () {
    *a += 10;
}