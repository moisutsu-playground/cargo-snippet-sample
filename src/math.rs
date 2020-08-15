use cargo_snippet::snippet;
use std::ops::*;

#[snippet(name = "@gcd", prefix = "use std::ops::*;")]
fn gcd<T>(a: T, b: T) -> T
where
    T: Copy + Ord + Sub<Output = T> + Rem<Output = T>,
{
    if a < b {
        return gcd(b, a);
    }
    let (mut a, mut b) = (a, b);
    let mut r: T;
    let zero = a - a;
    while a % b != zero {
        r = a % b;
        a = b;
        b = r;
    }
    b
}

#[snippet(name = "@lcm", include = "@gcd")]
fn lcm<T>(a: T, b: T) -> T
where
    T: Copy + Ord + Sub<Output = T> + Rem<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    a * b / gcd(a, b)
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(5i128, 5i128), 5i128);
    assert_eq!(gcd(12, 3), 3);
    assert_eq!(gcd(97usize, 17usize), 1usize)
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(10, 5), 10);
    assert_eq!(lcm(7usize, 11usize), 77usize);
    assert_eq!(lcm(1i128, 1i128), 1i128);
}
