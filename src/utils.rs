use cargo_snippet::snippet;

#[snippet("@echo")]
macro_rules! echo {
    ($x:expr) => {
        println!("{}", $x);
    };
}

#[snippet("@YesNo")]
fn YesNo(flag: bool) -> &'static str {
    if flag {"Yes"} else {"No"}
}

#[snippet("@YESNO")]
fn YESNO(flag: bool) -> &'static str {
    if flag {"YES"} else {"NO"}
}

#[snippet("@joins")]
trait Joins {
    fn joins(&self, sep: &str) -> String;
}

#[snippet("@joins")]
impl<T: std::string::ToString + Copy> Joins for Vec<T> {
    fn joins(&self, sep: &str) -> String {
        self.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(sep)
    }
}

#[test]
fn test_joins() {
    assert_eq!(vec![3, 1, 8, 5].joins(" "), "3 1 8 5");
}

#[test]
fn test_yesno() {
    assert_eq!(YESNO(true), "YES");
    assert_eq!(YESNO(false), "NO");
    assert_eq!(YesNo(true), "Yes");
    assert_eq!(YESNO(false), "No");
}
