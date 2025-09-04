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
			if p_stack.len() == 0 && s_stack.len() > 0 {
				return false;
			}

			if is_match {
				if is_asterisk_next {
					// if s_stack.len() > 0 {
					// 	current_s_symbol = s_stack.pop().expect("boom");
					// }
					if p_stack.len() > 0 {
						current_p_symbol = p_stack.pop().expect("boom");
					} else {
						current_p_symbol = '!';
					}
					is_asterisk_next = false;
				} else {
					if s_stack.len() > 0 {
						current_s_symbol = s_stack.pop().expect("boom");
					} else {
						current_s_symbol = '!';
					}
					if p_stack.len() > 0 {
						current_p_symbol = p_stack.pop().expect("boom");
					} else {
						current_p_symbol = '!';
					}
				}
			} else {
				if is_asterisk_next {
					if p_stack.len() > 0 {
						current_p_symbol = p_stack.pop().expect("boom");
					} else {
						current_p_symbol = '!';
					}
					is_asterisk_next = false;
				} else {
					let mut is_matched: bool = false;
					if s_stack.len() == 0 && p_stack.len() > 0 {
						asterisk_symbol_array.reverse();
						println!("ARRAY: {:?}", asterisk_symbol_array);
						while asterisk_symbol_array.len() > 0 {
							let current_elem: char = asterisk_symbol_array.pop().expect("boom");
							if current_elem == current_p_symbol {
								if p_stack.len() > 0 {
									current_p_symbol = p_stack.pop().expect("boom");
								} else {
									current_p_symbol = '!';
								}

								is_matched = true;
								break;
							}
						}
					}
					println!("ARRAY: {:?}", asterisk_symbol_array);
					println!("is matched: {}", is_matched);
					if !is_matched {
						return false;
					}
				}
			}
			
			if current_p_symbol == '*' {
				is_asterisk_next = true;
				if p_stack.len() > 0 {
					current_p_symbol = p_stack.pop().expect("boom");
				}
			}

			
			println!("current s symbol: {}", current_s_symbol);
			println!("current p symbol: {}", current_p_symbol);
			println!("s length: {}", s_stack.len());
			println!("p length: {}", p_stack.len());
			
			if Solution::is_symbols_correct( current_p_symbol, current_s_symbol ) {
				if !is_asterisk_next && (current_p_symbol == current_s_symbol || current_p_symbol == '.') {
					println!("match case");
					is_match = true;
				 	continue;
				} else if is_asterisk_next {
					println!("* case");
					while current_p_symbol == current_s_symbol || current_p_symbol == '.' {
						is_match = true;
						println!("* match: {}", current_p_symbol);
						asterisk_symbol_array.push( current_s_symbol );
						if s_stack.len() > 0 {
							current_s_symbol = s_stack.pop().expect("boom");
						} else {
							break;
						}
					}
					continue;
				} else {
					println!("no match case");
					if s_stack.len() == 0 && p_stack.len() == 0 {
						return false;
					}
					
					is_match = false;
					continue;
				}
			} else {
				if !is_asterisk_next {
					return false;
				}
			}

			current_p_symbol = '#';
			current_s_symbol = '#';
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
	let s: String = String::from("abcdede");
	let p: String = String::from("ab.*de");

	println!("{}", Solution::is_match( s, p ));
}
