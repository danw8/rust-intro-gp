fn main() {
	let x = 1;
	if x == 1 {
		println!("x is 1;");
	}
	let mut x = if x != 5 { 5 } else { 1 };
	while x == 5 {
		x -= 1;
		println!("We will exit eventually");
	}
	let array = [1, 2, 3, 4];
	for number in array.iter() {
		println!("{}", number);
	}
	loop {
		println!("This is the loop that never ends");
	}
}