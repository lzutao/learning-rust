#![allow(unused_variables,unused_imports,dead_code)]

fn main() {
    {
        type Kilometers = i32;

        let x: i32 = 5;
        let y: Kilometers = 5;

        println!("x + y = {}", x + y);
    }
    {
        type Thunk = Box<dyn Fn() + Send + 'static>;

        let f: Thunk = Box::new(|| println!("hi"));

        fn takes_long_type(f: Thunk) {
            // --snip--
        }

        fn returns_long_type() -> Thunk {
            Box::new(|| println!("world"))
        }
    }
    {
        type Result<T> = std::result::Result<T, std::io::Error>;

        pub trait Write {
            fn write(&mut self, buf: &[u8]) -> Result<usize>;
            fn flush(&mut self) -> Result<()>;

            fn write_all(&mut self, buf: &[u8]) -> Result<()>;
            fn write_fmt(&mut self, fmt: std::fmt::Arguments) -> Result<()>;
        }
    }
}
