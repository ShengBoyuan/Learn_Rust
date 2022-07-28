// functions.rs

fn add(mut a: u64, b: u64) {
    a += b;
    println!("Result {}", a);
}

fn main() { 
    let a: u64 = 17;
    let b: u64 = 21;
    let c = add(a, b);
}