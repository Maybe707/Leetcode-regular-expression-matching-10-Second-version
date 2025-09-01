use std::{thread::current, time::Duration};

struct Solution {
}

impl Solution {
	pub fn is_symbols_correct( current_p_symbol: char, current_s_symbol: char ) -> bool {
		if (current_p_symbol.is_ascii_alphabetic() &&
			current_p_symbol.is_ascii_lowercase() ||
			current_p_symbol as char == '.' ||
			current_p_symbol as char == '*' ||
			current_p_symbol as char != '!') &&
			(current_s_symbol.is_ascii_alphabetic() &&
			 current_s_symbol.is_ascii_lowercase() ||
			 current_s_symbol as char != '!') {
			 	true
			} else {
				false
			}
	}

	pub fn shift_back_matched_symbols_to_first_after_asterisk ( p_iterator: &mut usize, s_iterator: &mut usize,
																consumed_symbols_number_after_asterisk:
																&mut usize, is_no_match: bool ) {
		if is_no_match {
			// println!("shift back");
			// println!("consumed: {}", consumed_symbols_number_after_asterisk);
			*p_iterator -= *consumed_symbols_number_after_asterisk;
			*s_iterator -= *consumed_symbols_number_after_asterisk;
		}
	}
	
	pub fn is_match(s: String, p: String) -> bool {
		let mut p_stack: Vec<char> = p.chars().collect();
		let mut s_stack: Vec<char> = s.chars().collect();

		let mut p_iterator: usize = 0;
		let mut s_iterator: usize = 0;
		let mut current_p_symbol: char = '!';
		let mut current_s_symbol: char = '!';
		let mut asterisk_symbol_array: Vec<char> = vec![];
		let mut consumed_symbols_number_after_asterisk: usize = 0;
		let mut is_match: bool = true;
		let mut is_asterisk_next: bool = false;
		
		while p_stack.len() > 0 || s_stack.len() > 0
		{
			if is_match {
				if is_asterisk_next {
					current_s_symbol = s_stack.pop().unwrap_or('!');
				} else {
					current_s_symbol = s_stack.pop().unwrap_or('!');
					current_p_symbol = p_stack.pop().unwrap_or('!');
				}
			} else {
				if is_asterisk_next {
					current_p_symbol = p_stack.pop().unwrap_or('!');
					is_asterisk_next = false;
				} else {
					return false;
				}
			}
			
			if current_p_symbol == '*' {
				is_asterisk_next = true;
				current_p_symbol = p_stack.pop().unwrap_or('!');
			}

			if current_p_symbol == '!' || current_s_symbol == '!' {
				return false;
			}
			
			println!("current s symbol: {}", current_s_symbol);
			println!("current p symbol: {}", current_p_symbol);
			println!("s length: {}", s_stack.len());
			println!("p length: {}", p_stack.len());

			// if current_p_symbol == '!' || current_s_symbol == '!' {
			// 	return false;
			// }
			
			if Solution::is_symbols_correct( current_p_symbol, current_s_symbol ) {
				if !is_asterisk_next && (current_p_symbol == current_s_symbol || current_p_symbol == '.') {
					is_match = true;
				 	continue;
				} else if is_asterisk_next {
					if current_p_symbol == current_s_symbol || current_p_symbol == '.' {
						asterisk_symbol_array.push( current_s_symbol );
						is_match = true;
					} else {
						is_match = false;
					}
				} else {
					is_match = false;
				}
			}
		}

		println!("final p iterator: {}", p_iterator);
		println!("final p elem: {}", current_p_symbol);

		println!("final s iterator: {}", s_iterator);
		println!("final s elem: {}", current_s_symbol);

		println!("array: {:?}", asterisk_symbol_array);

		true
	}
}

fn main() {
	// let s: String = String::from("mississippi");
	// let p: String = String::from("mis*is*ip*.");

	// let s: String = String::from("ab");
	// let p: String = String::from(".*");

	// let s: String = String::from("mississippi");
	// let p: String = String::from("mis*is*p*.");

	// let s: String = String::from("mississippi");
	// let p: String = String::from("mis*is*ip*.");
	
	// let s: String = String::from("aaax");
	// let p: String = String::from("aaax");

	// let s: String = String::from("a");
	// let p: String = String::from("ab*a");

	// let s: String = String::from("aaa");
	// let p: String = String::from("ab*a*c*a");

	// let s: String = String::from("aaa");
	// let p: String = String::from("ab*a*c*a");

	// let s: String = String::from("aaa");
	// let p: String = String::from("ab*a*c*a");

	// let s: String = String::from("zxawqeraaaaazxaazx");
	// let p: String = String::from("zxa.*aazx");

	// let s: String = String::from("aweraxcvwer");
	// let p: String = String::from("a.*wer");

	// let s: String = String::from("aab");
	// let p: String = String::from("c*a*b");

	// let s: String = String::from("ab");
	// let p: String = String::from(".*c");

	// let s: String = String::from("aaaaaaaaa");
	// let p: String = String::from("a*aa");

	// let s: String = String::from("aaca");
	// let p: String = String::from("ab*a*c*a");

	// let s: String = String::from("aaaaaaarc");
	// let p: String = String::from("ab*a*r*c*aaarc");

	// let s: String = String::from("aaba");
	// let p: String = String::from("ab*a*c*a");

	// let s: String = String::from("a");
	// let p: String = String::from("ab*");

	// let s: String = String::from("aa");
	// let p: String = String::from("a*");

	// let s: String = String::from("a");
	// let p: String = String::from("ab*");

	// let s: String = String::from("bbbba");
	// let p: String = String::from(".*a*a");

	// let s: String = String::from("aaa");
	// let p: String = String::from("a*a");

	// let s: String = String::from("aa");
	// let p: String = String::from("a");

	let s: String = String::from("aab");
	let p: String = String::from("c*a*b");
	
	println!("{}", Solution::is_match( s, p ));
}
