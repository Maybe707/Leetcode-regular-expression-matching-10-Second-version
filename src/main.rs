use std::{thread::current, time::Duration};

struct Solution {
}

impl Solution {
	pub fn is_symbols_correct( current_p_symbol: char, current_s_symbol: char ) -> bool {
		if (current_p_symbol.is_ascii_alphabetic() &&
			current_p_symbol.is_ascii_lowercase() ||
			current_p_symbol as char == '.' ||
			current_p_symbol as char == '*' ||
			current_p_symbol as char == '!') &&
			(current_s_symbol.is_ascii_alphabetic() &&
			 current_s_symbol.is_ascii_lowercase() ||
			 current_s_symbol as char == '*') {
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
		let p_stack = p.as_bytes();
		let s_stack = s.as_bytes();

		let mut p_iterator: usize = 0;
		let mut s_iterator: usize = 0;
		let mut current_p_symbol: char = '*';
		let mut current_s_symbol: char = '*';
		let mut asterisk_symbol_array: Vec<char> = vec![];
		let mut consumed_symbols_number_after_asterisk: usize = 0;
		let mut is_no_match: bool = true;
		
		while p_iterator < p_stack.len() || s_iterator < s_stack.len()
		{
 			if p_iterator < p_stack.len() && p_iterator + 1 < p_stack.len() &&
				p_stack[p_iterator + 1] as char == '*' {
					consumed_symbols_number_after_asterisk = 0;
					asterisk_symbol_array.clear();
					while p_iterator + 1 < p_stack.len() &&
						p_stack[p_iterator + 1] as char == '*' {
							asterisk_symbol_array.push( p_stack[p_iterator] as char );
							p_iterator += 2;
						}
				} else if p_iterator < p_stack.len() {
					current_p_symbol = p_stack[p_iterator] as char;
					//				p_iterator += 1;
				} else {
					current_p_symbol = '!';
				}
			println!("p iterator: {}", p_iterator);
			println!("p elem: {}", current_p_symbol);
			if s_iterator < s_stack.len() {
				current_s_symbol = s_stack[s_iterator] as char;
				//				s_iterator += 1;
			} else {
				current_s_symbol = '!';
			}
			println!("s iterator: {}", s_iterator);
			println!("s elem: {}", current_s_symbol);

			if (asterisk_symbol_array.len() == 0 && p_iterator == p_stack.len() && s_iterator < s_stack.len()) ||
				(s_iterator == s_stack.len() && p_iterator < p_stack.len()) {
					println!("TESTS p iterator: {}", p_iterator);
					return false;
				}
			
			if Solution::is_symbols_correct( current_p_symbol, current_s_symbol ) {
//				println!("TEST");
				if current_p_symbol == current_s_symbol ||
				current_p_symbol == '.' {
					println!("case match");
					p_iterator += 1;
					s_iterator += 1;
					is_no_match = false;

					if asterisk_symbol_array.len() > 0 {
						consumed_symbols_number_after_asterisk += 1;
					}
				} else {
					println!("case no match");
					is_no_match = true;
				}					

				if is_no_match && asterisk_symbol_array.len() > 0 {
					println!("case *");
					Solution::shift_back_matched_symbols_to_first_after_asterisk(
						&mut p_iterator, &mut s_iterator,
						&mut consumed_symbols_number_after_asterisk, is_no_match
					);

					let s_iterator_old: usize = s_iterator;
					if consumed_symbols_number_after_asterisk > 0 {
						let mut asterisk_array_counter: usize = 0;
						while asterisk_array_counter < asterisk_symbol_array.len() {
							let current_asterisk_symbol: char = asterisk_symbol_array[asterisk_array_counter];
							//						println!("* symbol: {}", consumed_symbols_number_after_asterisk);
							let mut matched_substring_counter: usize = 0;
							while matched_substring_counter < consumed_symbols_number_after_asterisk {
  								if s_iterator < s_stack.len() && (current_asterisk_symbol == '.' ||
									current_asterisk_symbol == s_stack[s_iterator] as char) {
									println!("case 0 * symbol: {}", current_asterisk_symbol);
									s_iterator += 1;
								}
								matched_substring_counter += 1;
							}
							asterisk_array_counter += 1;
						}
					} else {
						let mut asterisk_array_counter: usize = 0;
						while asterisk_array_counter < asterisk_symbol_array.len() {
							let current_asterisk_symbol: char = asterisk_symbol_array[asterisk_array_counter];
							//						println!("* symbol: {}", consumed_symbols_number_after_asterisk);
  							if s_iterator < s_stack.len() && (current_asterisk_symbol == '.' ||
								current_asterisk_symbol == s_stack[s_iterator] as char) {
								println!("case 1 * symbol: {}", current_asterisk_symbol);
								s_iterator += 1;
								break;
							}

							asterisk_array_counter += 1;
						}
					}

					if s_iterator == s_iterator_old {
						return false;
					}

					consumed_symbols_number_after_asterisk = 0;
				}
			}
		}

		println!("final p iterator: {}", p_iterator);
		println!("final p elem: {}", current_p_symbol);

		println!("final s iterator: {}", s_iterator);
		println!("final s elem: {}", current_s_symbol);
		
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
	
	// let s: String = String::from("aaa");
	// let p: String = String::from("aaaa");

	// let s: String = String::from("a");
	// let p: String = String::from("ab*a");

	// let s: String = String::from("aaa");
	// let p: String = String::from("ab*a*c*a");

	// let s: String = String::from("aaa");
	// let p: String = String::from("ab*a*c*a");

	// let s: String = String::from("aaa");
	// let p: String = String::from("ab*a*c*a");

	// let s: String = String::from("aa");
	// let p: String = String::from("a");

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

	// let s: String = String::from("aaca");
	// let p: String = String::from("ab*a*c*a");

	let s: String = String::from("aaba");
	let p: String = String::from("ab*a*c*a");

	// let s: String = String::from("a");
	// let p: String = String::from("ab*");

	// let s: String = String::from("aa");
	// let p: String = String::from("a*");

	// let s: String = String::from("a");
	// let p: String = String::from("ab*");

	// let s: String = String::from("bbbba");
	// let p: String = String::from(".*a*a");

	println!("{}", Solution::is_match( s, p ));
}
