use std::cmp::Ordering;
use std::collections::HashSet;

const MAX_ATTEMPS: u8 = 6;
const ALPHABET_SIZE: usize = 26;
const LAST_BEST: u32 = 2132;

/*

enum PlaceStatus {
    Wrong, // 0
    Misplaced, // 1
    Correct, // 2
}
*/

fn main() {
    let mut qt_solved = 0;
    let mut qt_failed = 0;

    let dictionary: Vec<&str> = include_str!("dictionary.txt").split_whitespace().collect();
    // for a in include_str!("answers.txt").split_whitespace().take(10) {
    for a in include_str!("answers.txt").split_whitespace() {
        solve(a, &dictionary, &mut qt_solved, &mut qt_failed);
    }
    println!("Solved {qt_solved}");
    println!("Failed {qt_failed}");

    assert!(qt_solved >= LAST_BEST);
}

fn solve(ans: &str, dictionary: &Vec<&str>, qt_solved: &mut u32, qt_failed: &mut u32) {
    let answer: &[u8] = ans.as_bytes();
    // println!("------------------> word of the day: {ans}");
    let mut possible_words: Vec<&[u8]> = dictionary.clone().iter().map(|w| w.as_bytes()).collect();
    let mut bad_letters: [bool; ALPHABET_SIZE] = [false; ALPHABET_SIZE];
    let mut good_letters: [Option<u8>; 5] = [None; 5];
    let mut misplaced_letters: [HashSet<u8>; 5] = [
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
    ];

	// first guess is louts
    let mut guess: [u8; 5] = [108, 111, 117, 116, 115];
    let mut attempts = 0;
    while attempts <= MAX_ATTEMPS {
        // dbg!(possible_words); move value?!!
        // println!("{:?}", possible_words);
        attempts += 1;
        // println!("guess is {}", std::str::from_utf8(guess).unwrap());
        let resp = check_guess(answer, &guess);
        let mut correct_guess = true;
        for i in 0..5 {
            match resp[i] {
                0 => {
                    correct_guess = false;
                    set_bad_letter(&mut bad_letters, guess[i]);
                }
                1 => {
                    correct_guess = false;
                    misplaced_letters[i].insert(guess[i]);
                }
                2 => {
                    good_letters[i] = Some(guess[i]);
                }
                _ => panic!("Invalid value: {}", resp[i]),
            }
        }
        // dbg!(resp);
        if correct_guess {
            // println!("Solved '{ans}' in {attempts} attempts.");
            *qt_solved += 1;
            return;
        }
        if attempts == MAX_ATTEMPS {
            break;
        }

        // update possible_words
        let mut new_words = Vec::with_capacity(possible_words.len());
        'w:
		for w in possible_words {
            for i in 0..5 {
                if is_bad_letter(&bad_letters, w[i]) {
                    continue 'w;
                }
                if let Some(l) = good_letters[i] {
                    if w[i] != l {
                        continue 'w;
                    }
                }
                for l in misplaced_letters[i].iter() {
                    if w[i] == *l {
                        continue 'w;
                    }
                }
            }
            // check if misplaced letters exist in another place
            for i in 0..5 {
                let mut qt = 0;
                for l in misplaced_letters[i].iter() {
                    for z in 0..5 {
                        if z == i {
                            continue;
                        }
                        if w[z] == *l {
                            qt += 1;
                        }
                    }
                }
                if qt < misplaced_letters[i].len() {
                    continue 'w;
                }
            }
            new_words.push(w);
        }
        possible_words = new_words;

		let pfs = calculate_frequency(&possible_words);
		find_word(&pfs, &possible_words, &mut guess, 0);
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

fn calculate_frequency(words: &Vec<&[u8]>) -> [Vec<(u16, u8)>; 5] {
	let mut pos_freq = [[0u16; 26]; 5];

	for w in words.iter() {
		for i in 0..5 {
			pos_freq[i][(w[i] - 97) as usize] += 1;
		}
	}

	// use a tuple(freq, letter) so I'll be able to sort it
	let mut pfs: [Vec<(u16, u8)>; 5] = [
		vec![],
		vec![],
		vec![],
		vec![],
		vec![],
	];
	for i in 0..5 {
		for l in 0..26 {
			pfs[i].push((pos_freq[i][l], (l + 97) as u8));
		}
		pfs[i].sort_by(|a, b| b.cmp(a));
	}

	pfs
}

fn find_word(pfs: & [Vec<(u16, u8)>; 5], words: &Vec<&[u8]>, word: &mut[u8],
		pos: usize) -> bool {

	for i in 0..26 {
		if pos == 0 && pfs[pos][i].0 == 0 {
			panic!("Unable to find a word.");
		}
		word[pos] = pfs[pos][i].1;
		if pos > 0 {
			if let Err(_) = words.binary_search_by(|w| cmp(w, word, pos)) {
				continue;
			}
		}
		if pos == 4 {
			return true;
		}
		if find_word(pfs, words, word, pos + 1) {
			return true;
		}
	}
	false
}


fn cmp(word1: &[u8], word2: &[u8], pos: usize) -> Ordering {
	for i in 0..=pos {
		// let w1 = word1[i] - 97;
		if word1[i] == word2[i] { continue; }
		if word1[i] < word2[i] {
			return Ordering::Greater;
		}
		return Ordering::Less;
	}
	Ordering::Equal
}

