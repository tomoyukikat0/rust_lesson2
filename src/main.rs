fn say_hello() {
    println!("Hello, world!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    say_hello();
    println!("1 + 2 = {}", add(1, 2));
}
