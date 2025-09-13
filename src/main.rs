use std::{thread::current, time::Duration};

struct Solution {
}

#[derive(Debug, Clone)]
struct RegexpSymbol {
	symbol_type: Vec<char>,
	symbols: Vec<char>,
	min: usize,
	max: usize
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

	pub fn is_match(s: String, p: String) -> bool {
		let mut p_stack: Vec<char> = p.chars().collect();
		let mut s_stack: Vec<char> = s.chars().collect();

		let mut p_iterator: usize = 0;
		let mut s_iterator: usize = 0;
		let mut current_regexp_symbol: RegexpSymbol = RegexpSymbol{ symbol_type: vec![],
		symbols: vec![], min: 0, max: 0 };
		let mut current_s_symbol: char = '!';
		let mut regexp_array: Vec<RegexpSymbol> = vec![];
		let mut counter: usize = 0;
		while p_stack.len() > 0 {
			regexp_array.push( RegexpSymbol { symbol_type: vec![], symbols: vec![], min: 0, max: 0 } );
			let mut current_p_symbol_local: char = p_stack.pop().expect("boom");
			if current_p_symbol_local == '*' {
				regexp_array[counter].min = 0;
				regexp_array[counter].max = usize::MAX;
				regexp_array[counter].symbol_type.push( current_p_symbol_local );
				if p_stack.len() > 0 {
					current_p_symbol_local = p_stack.pop().expect("boom");
					regexp_array[counter].symbol_type.push( current_p_symbol_local );
				}
//				println!("element: {:?}", regexp_array[regexp_array.len() - 1].symbols);

			} else {
				regexp_array[counter].min = 1;
				regexp_array[counter].max = 1;
				regexp_array[counter].symbol_type.push( current_p_symbol_local );
//				println!("TEST");
//				println!("element: {:?}", regexp_array[regexp_array.len() - 1].symbols);
//				println!("element: {:?}", regexp_array[regexp_array.len() - 1].max);
			}

			counter += 1;
		}
		
		let mut no_machted_substring: Vec<char> = vec![];
		let mut asterisk_symbol_array_iterator: usize = 0;
		let mut consumed_symbols_number_after_asterisk: usize = 0;
		let mut is_s_match: bool = true;
		let mut is_p_match: bool = true;
		let mut is_asterisk_next: bool = false;
		let mut is_asterisk_match: bool = false;
//		println!("array: {:#?}", regexp_array);

		while s_stack.len() > 0
		{
			if s_stack.len() > 0 {
				current_s_symbol = s_stack.pop().expect("boom");
			} else {
				current_s_symbol = '#';
			}

			println!("current symbol: {}", current_s_symbol);
			let mut regexp_iterator: usize = 0;
			let regexp_array_size: usize = regexp_array.len();
			while regexp_iterator < regexp_array_size {
				let current_regexp_symbol: &mut RegexpSymbol = &mut regexp_array[regexp_iterator];
				if current_regexp_symbol.max == 1 && current_regexp_symbol.min == 1 &&
					current_regexp_symbol.symbols.len() < current_regexp_symbol.max {
						let current_iterator: usize = current_regexp_symbol.symbol_type.len();
						if current_iterator > 0 &&
							current_regexp_symbol.symbol_type[current_iterator - 1] ==
							current_s_symbol {
							current_regexp_symbol.symbols.push( current_s_symbol );
							println!("current regexp symbol 0: {:?}", current_regexp_symbol.symbols);
						}
						break;
					} else if current_regexp_symbol.max == usize::MAX && current_regexp_symbol.min == 0 {
						let current_iterator: usize = current_regexp_symbol.symbol_type.len();
						if current_iterator > 0 && current_regexp_symbol.symbol_type[current_iterator - 1] == '.' {
							current_regexp_symbol.symbols.push( current_s_symbol );
							println!("current regexp symbol 1: {:?}", current_regexp_symbol.symbols);
						} else if current_iterator > 0 &&
							current_regexp_symbol.symbol_type[current_iterator - 1] ==
							current_s_symbol {
								
							}

						break;
					}
				regexp_iterator += 1;
			}
		}

		true
	}
}

fn main() {
	// let s: String = String::from("abbabaaaaaaacaa");  // 14 last index
	// let p: String = String::from("a*.*b.a.*c*b*a*c*");  // 16 last index

	// let s: String = String::from("qweacbmzcab");  // 14 last index
	// let p: String = String::from("qweacbmzca*ab.*");  // 16 last index

	let s: String = String::from("baabbbaccbccacacc");  // 14 last index
	let p: String = String::from("c*..b*a*a.*a..*c");  // 16 last index

	println!("{}", Solution::is_match( s, p ));
}
