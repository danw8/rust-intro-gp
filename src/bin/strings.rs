fn main() {
	let reference: &str = "I am a reference to a String";
	println!("{}", reference);
	let string: String = String::from("I am a String");
	println!("{}", string);
}