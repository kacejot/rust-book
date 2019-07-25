use std::str::FromStr;
mod gcd;

fn parse_args() -> Vec<i64> {
    let mut numbers = Vec::new();
	for arg in std::env::args().skip(1) {
        numbers.push(i64::from_str(&arg)
			.expect("Error passing arg"));
	}

    numbers
}

fn main() {
	let numbers = parse_args();
	if numbers.len() == 0 {
        // TODO
	}

	let mut result = numbers[0];
	for number in &numbers[1..] {
		result = gcd::greatest_common_divisor(result, *number);
	}

	println!("The greatest common divisor for {:?} is {}", numbers, result);
}
