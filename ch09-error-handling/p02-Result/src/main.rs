#![allow(unused_variables)]
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    // {
    //     use std::io::ErrorKind;
    //     let f = File::open("hello.txt").map_err(|error| {
    //         if error.kind() == ErrorKind::NotFound {
    //             File::create("hello.txt").unwrap_or_else(|error| {
    //                 panic!("Tried to create file but there was a problem: {:?}", error);
    //             })
    //         } else {
    //             panic!("There was a problem opening the file: {:?}", error);
    //         }
    //     });
    // }
    /* Shortcuts for Panic on Error: unwrap and expect */
    {
        //let f = File::open("hello.txt").unwrap();
        // Or using expect
        //let f = File::open("hello.txt").expect("Failed to open hello.txt");
    }
    /* Propagating Errors */
    {
        fn read_username_from_file() -> Result<String, io::Error> {
            let f = File::open("hello.txt");

            let mut f = match f {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut s = String::new();

            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }
    }
    /* A Shortcut for Propagating Errors: the ? Operator */
    {
        fn read_username_from_file() -> Result<String, io::Error> {
            let mut f = File::open("hello.txt")?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        }
    }
    /* Even simpler */
    {
        fn read_username_from_file() -> Result<String, io::Error> {
            let mut s = String::new();
            File::open("hello.txt")?.read_to_string(&mut s)?;
            Ok(s)
        }
    }
    {
        use std::fs;
        fn read_username_from_file() -> Result<String, io::Error> {
            fs::read_to_string("hello.txt")
        }
    }
}
