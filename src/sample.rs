use cargo_snippet::snippet;

#[macro_export]
#[snippet("echo")]
macro_rules! echo {
    ($x:expr) => {
        println!("{}", $x);
    };
}

#[snippet]
pub fn sample() -> usize {
    5
}

#[snippet("joins")]
pub trait Joins {
    fn joins(&self, sep: &str) -> String;
}

#[snippet("joins")]
impl<T: std::string::ToString + Copy> Joins for Vec<T> {
    fn joins(&self, sep: &str) -> String {
        self.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(sep)
    }
}

#[test]
fn test_sample() {
    assert_eq!(sample(), 5);
}
