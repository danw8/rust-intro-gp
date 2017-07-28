fn main(){
	let s = String::from("Hedgehog");
	let s = return_ownership(s);
	println!("{}", s)
}

fn return_ownership(mine: String) -> String{
	println!("{}", mine);
	mine // nevermind you can have it back
}