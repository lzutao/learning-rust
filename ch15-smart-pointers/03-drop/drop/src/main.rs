struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {:?}!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("c"),
    };
    let d = CustomSmartPointer {
        data: String::from("d"),
    };
    //c.drop(); // Error!
    drop(c);
    println!("CustomSmartPointers created.");
}
