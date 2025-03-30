use std::env;
use std::sync::Once;

static INIT: Once = Once::new();

fn init_logger() {
    INIT.call_once(|| {
        env_logger::Builder::from_default_env().init();
    });
}

pub fn on_debug() {
    init_logger();
    std::env::set_var("RUST_LOG", "debug");
}

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
