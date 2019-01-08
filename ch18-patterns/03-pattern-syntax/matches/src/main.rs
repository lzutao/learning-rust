fn main() {
    // Match syntax
    {
        let x = 1;
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }
    {
        let x = 5;
        match x {
            1...5 => println!("one through five"),
            _ => println!("something else"),
        }
    }
    {
        let x = 'c';
        match x {
            'a'...'j' => println!("early ASCII letter"),
            'k'...'z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }
    // Destructuring to Break Apart Values
    {
        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 0, y: 7 };
        {
            let Point { x: a, y: b } = p;
            assert_eq!(0, a);
            assert_eq!(7, b);
        }
        {
            let Point { x, y } = p;
            assert_eq!(0, x);
            assert_eq!(7, y);
        }
        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
    }
    {
        #[allow(dead_code)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        // let msg = Message::Quit;
        // let msg = Message::Move { x: 3, y: 4 };
        let msg = Message::Write("hello weird!".to_string());
        // let msg = Message::ChangeColor(0, 160, 255);
        match msg {
            Message::Quit => println!("The Quit variant has no data to destructure."),
            Message::Move { x, y } => {
                println!("Move in the x direction {} and in the y direction {}", x, y)
            }
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {}, green {}, and blue {}", r, g, b)
            }
        }
    }
    {
        #[allow(dead_code)]
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }
        #[allow(dead_code)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }
        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change the color to red {}, green {}, and blue {}", r, g, b)
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            ),
            _ => (),
        }
    }
    {
        struct Point {
            x: i32,
            y: i32,
        }
        let points = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 5 },
            Point { x: 10, y: -3 },
        ];
        let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
        assert_eq!(sum_of_squares, 135);

        #[allow(unused_variables)]
        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    }
    {
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);
        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }
        println!("setting is {:?}", setting_value);

        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {}, {}, {}", first, third, fifth)
            }
        }
    }
    {
        #[allow(dead_code)]
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }
        let origin = Point { x: 0, y: 0, z: 0 };
        match origin {
            Point { x, .. } => println!("x is {}", x),
        }

        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, .., last) => {
                println!("Some numbers: {}, {}", first, last);
            }
        }
    }
    // Extra Conditionals with Match Guards
    {
        let num = Some(4);
        match num {
            Some(x) if x < 5 => println!("less than five: {}", x),
            Some(x) => println!("{}", x),
            None => (),
        }
        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matched, n = {:?}", n),
            _ => println!("Default case, x = {:?}", x),
        }
        println!("at the end: x = {:?}, y = {:?}", x, y);
        let x = 4;
        let y = false;
        match x {
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }
    }
    // @ Bindings
    {
        enum Message {
            Hello { id: i32 },
        }
        let msg = Message::Hello { id: 5 };
        match msg {
            Message::Hello {
                id: id_variable @ 3...7,
            } => println!("Found an id in range: {}", id_variable),
            Message::Hello { id: 10...12 } => println!("Found an id in another range"),
            Message::Hello { id } => println!("Found some other id: {}", id),
        }
    }
    // Legacy patterns: ref and ref mut
    {
        let robot_name = &Some(String::from("Bors"));
        match robot_name {
            &Some(ref name) => println!("Found a name: {}", name),
            None => (),
        }
        println!("robot_name is: {:?}", robot_name);
    }
}
