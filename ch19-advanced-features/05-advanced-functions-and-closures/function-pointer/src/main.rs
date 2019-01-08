use std::io;

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let mut input = String::new();
    let num = loop {
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(num) = input.trim().parse::<i32>() {
            break num;
        }
    };

    let answer = do_twice(add_one, num);
    println!("The answer is: {}", answer);

    {
        fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }
    }
}
