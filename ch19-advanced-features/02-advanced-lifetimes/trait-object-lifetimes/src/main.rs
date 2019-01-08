trait Red { }

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> { }

fn main() {
    let num = 5;
    let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;
    {
        struct StrWrap<'a>(&'a str);
        fn foo(string: &str) -> StrWrap<'_> {
            StrWrap(string)
        }
        // verbose
        impl<'a> fmt::Debug for StrWrap<'a> {
        // elided
        impl fmt::Debug for StrWrap<'_> {
    }
}
