use std::collections::HashSet;

const MAX_ATTEMPS: u8 = 6;
const ALPHABET_SIZE: usize = 26;
const LAST_BEST: u32 = 2020;

/*

enum PlaceStatus {
	Wrong, // 0
	Misplaced, // 1
	Correct, // 2
}
*/

fn main() {
	/*solve("sweet");
	solve("yuppy");
	solve("crazy");
	solve("proxy");
	solve("hunky");
	solve("junky");
	solve("bubbe");
	solve("audad");
	solve("bagel");
	solve("unbox");*/

	let mut qt_solved = 0;
	let mut qt_failed = 0;

	let dictionary: Vec<&str> = include_str!("dictionary.txt")
		.split_whitespace()
		.collect();
	// for a in include_str!("answers.txt").split_whitespace().take(10) {
	for a in include_str!("answers.txt").split_whitespace() {
		solve(a, &dictionary, &mut qt_solved, &mut qt_failed);
	}
	println!("Solved {qt_solved}");
	println!("Failed {qt_failed}");

	assert!(qt_solved >= LAST_BEST);
}

// fn solve(ans: &str) {
fn solve(ans: &str, dictionary: &Vec<&str>, qt_solved: &mut u32, qt_failed: &mut u32) {
	let answer: &[u8] = ans.as_bytes();
	// println!("------------------> word of the day: {ans}");
	let mut possible_words = dictionary.clone();

	let mut bad_letters: [bool; ALPHABET_SIZE] = [false; ALPHABET_SIZE];
	let mut good_letters: [Option<u8>; 5] = [None; 5];
	// Rust does not accept this ... =(
	// let mut misplaced_letters: [Vec<u8>; 5] = [vec![]; 5];
	let mut misplaced_letters: [HashSet<u8>; 5] = [
		HashSet::new(),
		HashSet::new(),
		HashSet::new(),
		HashSet::new(),
		HashSet::new(),
	];
	// let mut guess = "music".as_bytes();
	let mut guess = "abode".as_bytes();
	/*
	other good words: about, acute, adbot, acyls, adept
      wains, wafts, waift, wagyu, wacko, (watch, kedgy, virls)
	*/
	let mut best_candidates = vec!["picky", "minus", "joker", "tower", "winch", "stark", "mayor", "tasty", "proxy", "store", "paper", "rover", "spike", "troop", "worst", "untie", "steer", "known", "kedgy", "moans", "hunky", "virls", "watch", "state", "table", "scary"];
	let mut attempts = 0;
	while attempts <= MAX_ATTEMPS {
		// dbg!(possible_words); move value?!!
		// println!("{:?}", possible_words);
		attempts += 1;
		/*let guess = match attempts {
			1 => "music".as_bytes(),
			2 => "table".as_bytes(),
			3 => "proxy".as_bytes(),
			_ => possible_words[0].as_bytes(),
		};*/
		// println!("guess is {}", std::str::from_utf8(guess).unwrap());
		let resp = check_guess(answer, guess);
		let mut correct_guess = true;
		for i in 0..5 {
			match resp[i] {
				0 => {
					correct_guess = false;
					set_bad_letter(&mut bad_letters, guess[i]);
				},
				1 => {
					correct_guess = false;
					misplaced_letters[i].insert(guess[i]);
				},
				2 => {
					good_letters[i] = Some(guess[i]);
				},
				_ => panic!("Invalid value: {}", resp[i]),
			}
		}
		// dbg!(resp);
		if correct_guess {
			// println!("Solved '{ans}' in {attempts} attempts.");
			*qt_solved += 1;
			return;
		}
		if attempts == MAX_ATTEMPS { break; }
		// update possible_words
		let mut new_words = Vec::with_capacity(possible_words.len());
		'w:
		for w in possible_words {
			let b = w.as_bytes();
			for i in 0..5 {
				if is_bad_letter(&bad_letters, b[i]) {
					continue 'w;
				}
				if let Some(l) = good_letters[i] {
					if b[i] != l {
						continue 'w;
					}
				}
				for l in misplaced_letters[i].iter() {
					if b[i] == *l {
						continue 'w;
					}
				}
			}
			// check if misplaced letters exist in another place
			for i in 0..5 {
				let mut qt = 0;
				for l in misplaced_letters[i].iter() {
					for z in 0..5 {
						if z == i { continue; }
						if b[z] == *l { qt += 1; }
					}
				}
				if qt < misplaced_letters[i].len() {
					continue 'w;
				}
			}
			new_words.push(w);
		}
		possible_words = new_words;
		if possible_words.len() == 0 {
			//println!("Failed to find answer for {}", ans);
			*qt_failed += 1;
			return;
		}
		let mut found = false;
		while best_candidates.len() > 0 {
			let tmp = best_candidates.pop().unwrap();
			if let Ok(_) = possible_words.binary_search(&tmp) {
				guess = tmp.as_bytes();
				found = true;
				break;
			}
		}
		if ! found {
			guess = possible_words[0].as_bytes();
		}
	}
	println!("It was not able to solve: {}", ans);
	//dbg!(possible_words);
	*qt_failed += 1;
}

// 'a' -> 97
fn is_bad_letter(bad_letters: &[bool; ALPHABET_SIZE], letter: u8) -> bool {
	bad_letters[(letter - 97) as usize]
}

fn set_bad_letter(bad_letters: &mut [bool; ALPHABET_SIZE], letter: u8) {
	bad_letters[(letter - 97) as usize] = true;
}

fn check_guess(answer: &[u8], guess: &[u8]) -> [u8; 5] {
	// 0 -> wrong, 1 -> misplaced, 2 -> correct
	let mut resp: [u8; 5] = [0; 5];

	for i in 0..5 {
		for j in 0..5 {
			if guess[i] == answer[j] {
				if i == j {
					resp[i] = 2;
				} else if resp[i] != 2 {
					resp[i] = 1;
				}
			}
		}
	}
	resp
}

