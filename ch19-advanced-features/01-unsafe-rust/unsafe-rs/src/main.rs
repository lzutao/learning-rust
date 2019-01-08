fn main() {
    {
        let mut num = 5;
        let r1 = &num as *const i32;
        // let r2 = &mut num as *mut i32;
        let r2 = &mut num as *mut i32;
        unsafe {
            assert_eq!(*r1, 5);
            assert_eq!(*r2, 5);
        }
    }
    // Creating a Safe Abstraction over Unsafe Code
    {
        use std::slice;
        fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = slice.len();
            assert!(mid <= len);
            let ptr = slice.as_mut_ptr();
            unsafe {
                (
                    slice::from_raw_parts_mut(ptr, mid),
                    slice::from_raw_parts_mut(ptr.add(mid), len - mid),
                )
            }
        }
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let (a, b) = split_at_mut(&mut v, 3);
        assert_eq!(a, [1, 2, 3]);
        assert_eq!(b, [4, 5, 6]);
    }
    {
        use std::slice;
        let address = 0x01234usize;
        let r = address as *mut i32;
        let s: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
        assert_eq!(s.len(), 10000);
    }
    // Using extern Functions to Call External Code
    {
        extern "C" {
            fn abs(input: i32) -> i32;
        }

        unsafe {
            assert_eq!(3, abs(-3));
        }
    }
    // Accessing or Modifying a Mutable Static Variable
    {
        static HELLO_WORLD: &str = "Hello, world!";
        println!("name is: {}", HELLO_WORLD);
    }
    {
        static mut COUNTER: u32 = 0;
        fn add_to_count(inc: u32) {
            unsafe {
                COUNTER += inc;
            }
        }
        add_to_count(3);
        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }
    // Implementing an Unsafe Trait
    {
        unsafe trait Foo {
            // methods go here
        }
        unsafe impl Foo for i32 {
            // method implementations go here
        }
    }
}
