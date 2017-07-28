fn main() {
	let s = String::from("Hedgehog");
	i_borrow_strings(&s);
	you_give_me_string(s);
	// i_borrow_strings(&s); // 'value used here after move'
}

fn i_borrow_strings(on_loan: &String) {
	println!("I'm borrowing the {}", on_loan);
}

fn you_give_me_string(mine_now: String) {
	println!("The {} is mine now", mine_now);
}