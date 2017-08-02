#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
let x = 5;
let five  = if x == 5 { "five" } else { "not five" };
println!("{:?}", five);
}


fn add_two_i32(number: i32) -> i32{
    number + 2
}

/* immutably add two*/
#[test]
fn add_two_test() {
    let number = 0;
    assert!(add_two_i32(number) == 2);
}

fn add_three_i32(number : &mut i32){
    *number += 3;
}

/* mutably add three*/
#[test]
fn add_three_test() {
    let mut number = 0;
    add_three_i32(&mut number);
    assert!(number == 3);
}

fn is_odd_i32(number: &i32) -> bool {
    number % 2 != 0
}

#[test]
fn is_odd_test() {
    for num in 1..10{
        if num % 2 == 0 {
            assert!(!is_odd_i32(&num));
        } else {
            assert!(is_odd_i32(&num));
        }
    }
}

fn count_odds(numbers: &[i32]) -> i32 {
    let mut number_odd = 0;
    for num in numbers.iter() {
        if is_odd_i32(&num) {
            number_odd += 1;
        }
    }
    number_odd
}

#[test]
fn count_odds_test() {
    let numbers: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];
    assert!(count_odds(&numbers) == 5);
}

fn longest_str<'a>(o: &'a str, t: &'a str) -> &'a str {
    if o.len() > t.len() {
        o
    } else {
        t
    }
}

// (Hint) Use the .len() function for &str
// (Hint) Use lifetimes <'a>
#[test]
fn longest_str_test(){
    let short = "short";
    let longer = "longer";
    assert!(longest_str(short, longer) == longer);
}



