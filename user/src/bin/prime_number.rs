#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let mut count = 0;

    let limit = 10000;

    for i in 0..limit {
        if is_prime(i) {
            count += 1;
        }
    }

    println!("There are {} prime numbers less than {}.", count, limit);
    0
}

fn is_prime(n: usize) -> bool {
    match n {
        0 | 1 => false,
        2 => true,
        _ => {
            for i in 2..=n - 1 {
                if n % i == 0 {
                    return false;
                }
            }
            true
        }
    }
}
