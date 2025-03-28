use std::{env, fmt::Debug};

pub fn is_debug() -> bool {
    env::var("RUST_LOG").as_deref() == Ok("debug")
}

pub fn if_debug<T>(x: T) -> Option<T> {
    if is_debug() {
        Some(x)
    } else {
        None
    }
}

pub fn map_to_alphabet(num: usize) -> String {
    let mut n = num + 1;
    let mut result = String::new();
    while n > 0 {
        n -= 1;
        result.push((b'a' + (n % 26) as u8) as char);
        n /= 26;
    }
    // result.push('_');
    result.chars().rev().collect()
}

pub fn if_empty<T>(xs: Vec<T>, default: T) -> Vec<T> {
    if xs.is_empty() {
        vec![default]
    } else {
        xs
    }
}

pub fn to_superscript(num: u64) -> String {
    let superscripts = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
    num.to_string()
        .chars()
        .map(|c| superscripts[c.to_digit(10).expect("Invalid digit") as usize])
        .collect()
}

pub fn to_subscript(num: u64) -> String {
    let subscripts = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];
    num.to_string()
        .chars()
        .map(|c| subscripts[c.to_digit(10).expect("Invalid digit") as usize])
        .collect()
}

pub fn pair_app<T, U>(f: impl Fn(T) -> U) -> impl Fn((T, T)) -> (U, U) {
    move |(a, b)| (f(a), f(b))
}

pub fn map_fix<const N: usize, T, U: Debug>(xs: &[T; N], f: impl FnMut(&T) -> U) -> [U; N] {
    xs.iter().map(f).collect::<Vec<U>>().try_into().unwrap()
}

pub fn zip_fix<const N: usize, T: Debug + Clone, U: Debug + Clone>(
    xs: &[T; N],
    ys: &[U; N],
) -> [(T, U); N] {
    xs.iter()
        .zip(ys.iter())
        .map(|(p, q)| (p.clone(), q.clone()))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
