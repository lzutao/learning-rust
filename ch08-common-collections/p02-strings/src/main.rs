#![allow(unused_variables)]
fn main() {
    {
        // Empty string
        let s = String::new();
        // Init from &str
        let data = "initial contents";
        let s = data.to_string();
        // the method also works on a literal directly:
        let s = "initial contents".to_string();
        let s = String::from("initial contents");
        let hello = String::from("こんにちは");
    }
    /* Updating a String */
    {
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(&s2);
        s1.push('!');
        println!("s1 is {:?}", s1);
    }
    /* Concatenation with the + Operator or the format! Macro */
    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);
    }
    /* Indexing into Strings */
    {
        let s1 = String::from("hello");
        //let h = s1[0]; // Error!
        //let h = &s1[0]; // Error!

        let len = String::from("Здравствуйте").len();
        println!("{:?}", len);
    }
    /* Bytes and Scalar Values and Grapheme Clusters */
    {
        let hello = "Здравствуйте";
        let s = &hello[0..4];
        //let s = &hello[0..1]; // Runtime Crash!
    }
    /* Methods for Iterating Over Strings */
    {
        let s = "नमस्ते";
        println!("s has {:?} characters", s.chars().count());
        assert_eq!(s.len(), s.bytes().count());
    }
}
