#![allow(dead_code)]
#![allow(unused_variables)]

extern crate num;

use std::ops::Rem;
use num::traits::NumCast;

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
fn is_odd_i32_test() {
    for num in -10..10{
        if num % 2 == 0  {
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

fn get_odds<I>(set: Vec<I>) -> Vec<I> 
	where I: Rem<Output=I> + NumCast + Copy + PartialEq 
{
	let two = I::from(2).unwrap();
	let zero = I::from(0).unwrap();
	set.into_iter().filter(|n| *n % two != zero).collect()
}

#[test]
fn get_odds_test_i32(){
	let ints = vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 
			0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 ];
	let odds : Vec<i32> = get_odds(ints);
	assert!(odds == vec![-9,-7,-5,-3,-1,1,3,5,7,9]);
}

/* Hints
	Use the Rem<Output=T>, NumCast, Copy, PartialEq traits
*/
#[test]
fn get_odds_test_floats(){
	let floats = vec![-10.0, -9.0, -8.0, -7.0, -6.0, 
				-5.0, -4.0, -3.0, -2.0, -1.0,
				0.0, 1.0, 2.0, 3.0, 4.0, 5.0,
				6.0, 7.0, 8.0, 9.0, 10.0];
	let odds: Vec<f64> = get_odds(floats);
	assert!(odds == vec![-9.0, -7.0, -5.0, -3.0, -1.0, 1.0, 3.0, 5.0, 7.0, 9.0]);
}




fn longest_str<'a>(o: &'a str, t: &'a str) -> &'a str {
    if o.len() > t.len() {
        o
    } else {
        t
    }
}

/* Hints 
	Use the .len() function for &str
	Use lifetimes <'a>
*/
#[test]
fn longest_str_test(){
    let short = "short";
    let longer = "longer";
    assert!(longest_str(short, longer) == longer);
}



