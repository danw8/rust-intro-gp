fn main() {
	let boolean_type = true;
	let character_type = 'A';
	let string_reference = "Hedgehog";
	let tuple = (character_type, boolean_type, string_reference);
	let (c, b, s) = tuple;
	println!("The value in the tuple is: {} {} {}", c, b, s);
}