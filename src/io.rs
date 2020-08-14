use cargo_snippet::snippet;

#[snippet("@echo")]
macro_rules! echo {
    ($x:expr) => {
        println!("{}", $x);
    };
}
