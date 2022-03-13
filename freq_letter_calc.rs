use std::cmp::Ordering;

fn main() {

	// let l1: [u8; 5] = [3, 1, 0, 0, 0];
	// println!("{:?}", l1);
	// println!("{:?}", &l1[1..2]);
	// println!("{:?}", std::str::from_utf8(&l1));
	// println!("{:?}", std::str::from_utf8(&l1[..1]));

    let mut words: Vec<&str> = vec![
        "joker", "aptly", "virls", "burly", "untie", "hunky", "known", "kains", "kedgy", "mourn", "mayor", "minus",
        "moans", "music", "racer", "rover", "riper", "rayon", "rates", "rusty", "perky",
        "paper", "picky", "patch", "prone", "proxy", "paste", "troop", "tower", "tasty", "teary",
        "table", "spike", "steer", "stark", "steal", "stack", "store", "spite", "scary", "super", "sower",
        "worst", "winch", "watch", "worse", "wares", "layer", "water", "baldy", "ceils",];

    // let mut words: Vec<&str> = vec!["joker", "aptly", "virls"];
	words.sort();
    let words: Vec<&[u8]> = words.iter().map(|w| w.as_bytes()).collect();
	let pfs = calculate_frequency(&words);

	let mut guess = [0u8; 5];
	find_word(&pfs, &words, &mut guess, 0);
	println!("Guess is: {:?}", guess);
	println!("Guess is: {:?}", std::str::from_utf8(&guess));
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

#[allow(dead_code)]
fn print_pos_freq(pf: &[[u16; 26]; 5]) {
	for l in 0..26 {
		for p in 0..5 {
			print!("{} - {:<5} |", (l + 97) as u8 as char, pf[p][l]);
		}
		println!();
	}
}

