#[macro_export]
macro_rules! vec {
    ($($x:expr),*) => {
        {
            let mut tmp_vec = Vec::new();
            $(
                tmp_vec.push($x);
            )*
            tmp_vec
        }
    };
}
