mod gcd;

fn main () {
	let numbers = vec![14, 28, 35];

	if numbers.len() == 0 {

	}

	let mut result = numbers[0];
	for number in &numbers[1..] {
		result = gcd::greatest_common_divisor(result, *number);
	}
}
