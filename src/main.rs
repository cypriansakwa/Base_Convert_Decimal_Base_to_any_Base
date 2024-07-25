fn main() {
    let number = 40.8125;
    let base = 2; // Required be between 2 and 36

    match convert_to_base(number, base) {
    Some(result) 
    => println!("The number {} in base {} is {}", number, base, result),
        None => println!("Invalid base."),
    }
}

fn convert_to_base(number: f64, base: u32) -> Option<String> {
    if base < 2 || base > 36 {
        return None;
    }

    let integer_part = number.trunc() as u64;
    let fractional_part = number.fract();

    let integer_str = convert_integer_part_to_base(integer_part, base);
    let fractional_str = convert_fractional_part_to_base(fractional_part, base);

    Some(format!("{}.{}", integer_str, fractional_str))
}

fn convert_integer_part_to_base(mut number: u64, base: u32) 
-> String {
    let mut result = String::new();
    let digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    if number == 0 {
        return "0".to_string();
    }

    while number > 0 {
        let remainder = number % base as u64;
        result.push(digits.chars().nth(remainder as usize).unwrap());
        number /= base as u64;
    }

    result.chars().rev().collect()
}

fn convert_fractional_part_to_base(mut number: f64, base: u32) -> String {
    let mut result = String::new();
    let digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    for _ in 0..10 { 
// Limits to 10 digits for fractional precision
        number *= base as f64;
        let digit = number.trunc() as usize;
        result.push(digits.chars().nth(digit).unwrap());
        number -= digit as f64;
        if number == 0.0 {
            break;
        }
    }

    result
}