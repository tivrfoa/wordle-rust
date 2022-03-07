const MAX_ATTEMPS: u8 = 6;

/*

enum PlaceStatus {
	Wrong, // 0
	Misplaced, // 1
	Correct, // 2
}
*/

fn main() {


	// let letters: Vec<u8> = ('A'..'Z').map(|c| c as u8).collect();
	let letters: [u8; 25] = [65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89];
	let answer: &[u8] = "CLOTH".as_bytes();

	let guess = "TABLE".as_bytes();
	assert!(! is_guess_correct(&check_guess(answer, guess)));

	let guess = "CLOTH".as_bytes();
	assert!(is_guess_correct(&check_guess(answer, guess)));

	let mut bad_letters: [bool; 25] = [false; 25];
	// (u8, u8) -> letter, PlaceStatus
	let mut good_letters: [Option<(u8, u8)>; 5] = [None; 5];
	let mut attempts = 0;
	while attemps <= MAX_ATTEMPS {
		attemps += 1;
		let guess = guess(&history, &dictionary);
		let resp = check_guess(answer, guess);
		let mut correct_guess = true;
		for i in 0..5 {
			if guess[i] != 2 {
				correct_guess = false;
				break;
			}
		}
	correct_guess
		if is_guess_correct(&resp) {
			println!("Solved in {attempts} attempts.");
			return;
		}
	}
}

fn guess(history: ss, dictionary: ss) -> &str {
	"TABLE"
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

fn is_guess_correct(guess: &[u8; 5]) -> bool {
	let mut correct_guess = true;
	for i in 0..5 {
		if guess[i] != 2 {
			correct_guess = false;
			break;
		}
	}
	correct_guess
}
