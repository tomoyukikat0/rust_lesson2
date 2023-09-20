// fn fizzbuzz(end: i32) {
//     let mut x: i32 = 1;

//     while x <= end {
//         if x % 3 == 0 && x % 5 == 0 {
//             println!("FizzBuzz");
//         } else if x % 3 == 0 {
//             println!("Fizz");
//         } else if x % 5 == 0 {
//             println!("Buzz");
//         } else {
//             println!("{}", x);
//         }
//         x += 1;
//     }
// }

use std::ops::RangeInclusive;

fn fizzbuzz(end: i32) {
    let r: RangeInclusive<i32> = 1..=end;

    for x in r {
        if x % 3 == 0 && x % 5 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }
}

fn main() {
    fizzbuzz(30)
}
