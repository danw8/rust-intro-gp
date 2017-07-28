fn main() {
	let bool_type = true;
	let char_type = 'A';
	let string_ref = "Hedgehog";
	let tuple = (char_type, bool_type, string_ref);
	let (c, b, s) = tuple;
	println!("The value in the tuple is: {} {} {}", c, b, s);
}