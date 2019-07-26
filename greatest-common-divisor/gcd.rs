use std::mem;

pub fn greatest_common_divisor(mut a: i64, mut b: i64) -> i64 {
	while b != 0 {
		if a > b {
			mem::swap(&mut a, &mut b);
		}
		b = b % a;
	}
	a
}

#[test]
fn gcd_for_similar_numbers() {
	let number = greatest_common_divisor(14, 14);
	assert_eq!(number, 14);
}

#[test]
fn gcd_for_coprime_numbers() {
	let number = greatest_common_divisor(14, 15);
	assert_eq!(number, 1);
}

#[test]
fn gcd_for_non_coprime_numbers() {
	let number = greatest_common_divisor(
		7 * 4 * 1,
		7 * 5 * 1);

	assert_eq!(number, 7);
}
