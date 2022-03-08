const MAX_ATTEMPS: u8 = 6;
const ALPHABET_SIZE: usize = 26;

/*

enum PlaceStatus {
	Wrong, // 0
	Misplaced, // 1
	Correct, // 2
}
*/

fn main() {
	solve("yuppy");
	solve("table");
	solve("music");
	solve("proxy");
	solve("cloth");
}

fn solve(ans: &str) {
	let answer: &[u8] = ans.as_bytes();

	// let dictionary = ["TABLE", "PROXY", "MUSIC", "SWING", "CLOTH"];
	// let dictionary = vec!["TABLE", "BREAD", "PROXY", "MUSIC", "SWING", "CLOTH"];
	let dictionary: Vec<&str> = include_str!("dictionary.txt")
		.split_whitespace()
		// .map(|w| w.to_uppercase())
		.collect();
	let mut possible_words = dictionary.clone();

	let mut bad_letters: [bool; ALPHABET_SIZE] = [false; ALPHABET_SIZE];
	let mut good_letters: [Option<u8>; 5] = [None; 5];
	// Rust does not accept this ... =(
	// let mut misplaced_letters: [Vec<u8>; 5] = [vec![]; 5];
	let mut misplaced_letters: [Vec<u8>; 5] = [vec![], vec![], vec![], vec![], vec![]];
	let mut attempts = 0;
	while attempts <= MAX_ATTEMPS {
		// dbg!(possible_words); move value?!!
		// println!("{:?}", possible_words);
		attempts += 1;
		let guess = possible_words[0].as_bytes();
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
					misplaced_letters[i].push(guess[i]);
				},
				2 => {
					good_letters[i] = Some(guess[i]);
				},
				_ => panic!("Invalid value: {}", resp[i]),
			}
		}
		if correct_guess {
			println!("Solved '{ans}' in {attempts} attempts.");
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
				for j in 0..misplaced_letters[i].len() {
					if b[i] == misplaced_letters[i][j] {
						continue 'w;
					}
				}
			}
			new_words.push(w);
		}
		possible_words = new_words;
	}
	println!("It was not able to solve: {}", ans);
	dbg!(possible_words);
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
				} else {
					resp[i] = 1;
				}
			}
		}
	}
	resp
}

