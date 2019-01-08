#![allow(unused_variables,dead_code)]
fn main() {
    {
        fn bar() -> ! {
            panic!("this is never type");
        }
    }
    {
        let guess = "666";
        loop {
            let guess = match guess.trim().parse::<u64>() {
                Ok(_) => 5,
                Err(_) => break,
            };
        }
    }
}
