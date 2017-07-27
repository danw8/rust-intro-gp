fn main() {
	let mut x = 0;
	'outer: loop {
		x += 1;
		println!("In outer loop, x's value is: {}", x);
		'inner: while x >= 3 {
			x += 1;
			println!("In inner loop, x's value is: {}", x);
			if x > 6 { break 'outer }
			if x % 2 == 0 { break 'inner }
		}

	}
}