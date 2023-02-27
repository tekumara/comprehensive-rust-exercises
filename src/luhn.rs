// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    if cc_number.len() == 0 {
        return false
    }

    let mut sum = 0;
    let mut digits_count = 0;

    for c in cc_number.chars().rev() {
        match c.to_digit(10) {
            Some(digit) => {
                digits_count += 1;
                if digits_count % 2 == 0 {
                    let doubled = digit * 2;
                    let sum_digits = doubled / 10 + doubled % 10;
                    sum += sum_digits;
                } else {
                    sum += digit;
                }
            },
            _ => continue
        }
    }

    digits_count >= 2 && sum % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}
