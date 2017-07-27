fn main(){
	let default_bits = 5; 
	println!("The value of default_bits: {}", default_bits);
	let eight_bits_unsigned = 2u8;
	println!("The value of eight_bits_unsigned: {}", eight_bits_unsigned);
	let sixteen_bits_signed = 0xfi16;
	println!("The value of sixteen_bits_signed: {}", sixteen_bits_signed);
	let sixtyfour_bits = 0o77i64;
	println!("The value of sixtyfour_bits: {}", sixtyfour_bits);
	let binary = 0b1111_0000i32;
	println!("The value of binary: {}", binary);
	let byte = b'A';
	println!("The value of byte: {}", byte);
}