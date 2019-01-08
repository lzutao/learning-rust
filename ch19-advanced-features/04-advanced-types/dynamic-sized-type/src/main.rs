#![allow(unused_variables)]
#![allow(dead_code)]
fn main() {
    {
        // That is, a generic function definition like this:
        fn generic<T>(t: T) {
            // --snip--
        }
    }
    {
        // is actually treated as though we had written this:
        fn generic<T: Sized>(t: T) {
            // --snip--
        }
    }
    // By default, generic functions will work only on types that have a
    // known size at compile time. However, you can use the following
    // special syntax to relax this restriction:
    {
        fn generic<T: ?Sized>(t: &T) {
            // --snip--
        }
        // A trait bound on ?Sized is the opposite of a trait bound on Sized:
        // we would read this as "T may or may not be Sized." This syntax is
        // only available for Sized, not any other traits.

        // Also note that we switched the type of the t parameter from T to &T.
        // Because the type might not be Sized, we need to use it behind some
        // kind of pointer. In this case, we’ve chosen a reference.
    }
}
