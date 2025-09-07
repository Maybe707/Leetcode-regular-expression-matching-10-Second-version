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
		let mut no_machted_substring: Vec<char> = vec![];
		let mut asterisk_symbol_array_iterator: usize = 0;
		let mut consumed_symbols_number_after_asterisk: usize = 0;
		let mut is_s_match: bool = true;
		let mut is_p_match: bool = true;
		let mut is_asterisk_next: bool = false;
		let mut is_asterisk_match: bool = false;
		
		while p_stack.len() > 0 || s_stack.len() > 0
		{
			if p_stack.len() == 0 && s_stack.len() > 0 {
				return false;
			}

			if s_stack.len() > 0 && is_s_match {
				current_s_symbol = s_stack.pop().expect("boom");
			} else if s_stack.len() == 0 && !is_asterisk_next {
				current_s_symbol = '#';
			}

			if p_stack.len() > 0 && is_p_match {
				current_p_symbol = p_stack.pop().expect("boom");
			}

			if is_asterisk_next {
				is_asterisk_next = false;
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
//			asterisk_symbol_array.reverse();
			println!("AR: {:?}", asterisk_symbol_array);
			
			if Solution::is_symbols_correct( current_p_symbol, current_s_symbol ) {
				if !is_asterisk_next && current_s_symbol != '#' &&
					(current_p_symbol == current_s_symbol || current_p_symbol == '.') {
						println!("match case");
						is_s_match = true;
						is_p_match = true;
					} else if is_asterisk_next {
						println!("* case");
						while s_stack.len() > 0 {
							if (current_p_symbol == current_s_symbol ||
								current_p_symbol == '.') && current_s_symbol != '#' {
									is_s_match = true;
									println!("* match: {}", current_p_symbol);
									asterisk_symbol_array.push( current_s_symbol );
									current_s_symbol = s_stack.pop().expect("boom");
								} else {
									is_s_match = false;
									break;
								}
						}

						if (current_p_symbol == current_s_symbol ||
							current_p_symbol == '.') && current_s_symbol != '#' {
								is_s_match = true;
								println!("* last match: {}", current_p_symbol);
								asterisk_symbol_array.push( current_s_symbol );
							} else {
								is_s_match = false;
							}

						println!("* is s matched: {}", is_s_match);
					} else {
						println!("no match case");
						let mut asterisk_symbol_array_temp: Vec<char> = asterisk_symbol_array.clone();
						asterisk_symbol_array_temp.reverse();
						no_machted_substring.push( current_p_symbol );
						let mut no_matched_substring_temp: Vec<char> = no_machted_substring.clone();
						no_matched_substring_temp.reverse();
						println!("reversed array: {:?}", asterisk_symbol_array_temp);
						println!("substring: {:?}", no_matched_substring_temp);
						let mut is_contain_substring: bool = false;
						let mut outer_counter: usize = asterisk_symbol_array_iterator;
						while outer_counter < asterisk_symbol_array_temp.len() {
							println!("iter #: {}", outer_counter);
							let mut inner_counter: usize = 0;
							while inner_counter < no_matched_substring_temp.len() &&
								outer_counter + inner_counter < asterisk_symbol_array_temp.len() {
									if no_matched_substring_temp[inner_counter] ==
										asterisk_symbol_array_temp[outer_counter + inner_counter] ||
										no_matched_substring_temp[inner_counter] == '.' {
											println!("ELEM: {}", asterisk_symbol_array_temp[outer_counter + inner_counter]);
											is_contain_substring = true;
										} else {
											is_contain_substring = false;
											break;
										}
									inner_counter += 1;
								}
							if is_contain_substring {
								break;
							}
							outer_counter += 1;
						}
						asterisk_symbol_array_iterator = outer_counter;
						println!("is s matched: {}", is_s_match);						
						if s_stack.len() == 0 && p_stack.len() == 0 {
							return false;
						} else if !is_contain_substring {
							return false;
						}
					}
			} else {
				println!("wrong symbol case");
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
	// let s: String = String::from("abbabaaaaaaacaa");  // 14 last index
	// let p: String = String::from("a*.*b.a.*c*b*a*c*");  // 16 last index

	let s: String = String::from("a");  // 14 last index
	let p: String = String::from(".*..a*");  // 16 last index

	println!("{}", Solution::is_match( s, p ));
}
