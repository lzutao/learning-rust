#![allow(unused_variables)]
fn main() {
    // Conditional if let Expressions
    {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {}, as the background", color);
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }
    // while let Conditional Loops
    {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            print!("{}; ", top);
        }
        println!();
    }
    // for Loops
    {
        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            print!("{} is at index {}; ", value, index);
        }
        println!();
    }
    // let Statements
    {
        let (x, y, z) = (1, 2, 3);
    }
    // Function Parameters
    {
        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("Current location: ({}, {})", x, y);
        }
        let point = (3, 5);
        print_coordinates(&point);
    }

}
