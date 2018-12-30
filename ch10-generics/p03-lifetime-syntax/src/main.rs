#![allow(unused_variables,dead_code)]
fn main() {
    /* Function Signatures */
    {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        let string1 = String::from("long string is long");
        {
            let string2 = String::from("xyz");
            let result = longest(&string1, &string2);
            println!("The longest string is {:?}", result);
        }
    }
    /* Struct Definitions */
    {
        #[derive(Debug)]
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
        println!("{:?}", i);
    }
    /* Lifetime Elision */
    {
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }
            s
        }
    }
    /* Lifetime Annotations in Method Definitions */
    {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        impl<'a> ImportantExcerpt<'a> {
            fn level(&self) -> i32 {
                3
            }
            fn announce_and_return_part(&self, announcement: &str) -> &str {
                println!("Attention please: {}", announcement);
                self.part
            }
        }
    }
    /* The Static Lifetime */
    {
        let s: &'static str = "I have a static lifetime.";
    }
    /* Generic Type Parameters, Trait Bounds, and Lifetimes Together */
    {
        use std::fmt::Display;

        fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
            where T: Display
        {
            println!("Announcement! {}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
    }
}
