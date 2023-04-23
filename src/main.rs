use std::io;

fn main() {
    let answer = String::from("tests");
    let mut lives = 3;
    let mut guess = String::new();
    let stdin = io::stdin();

    loop {
        if guess.trim() == answer {
            println!("You Win!");
            break;
        }

        if lives == 0 {
            println!("You Lose...");
            break;
        }

        println!("Guess a five-lettered word:");

        stdin.read_line(&mut guess).expect("An error occured");

        if guess.trim().len() != 5 {
            guess.clear();
            println!("\nPlease choose a five-letter word\n");
            continue;
        }

        let mut res = String::new();
        for (idx, char) in guess.trim().char_indices() {
            let answer_char: char = answer
                .chars()
                .nth(idx)
                .expect("Should choose a five-lettered word only");

            if char == answer_char {
                res.push('/');
                continue;
            }

            res.push('X');
        }
        println!("\n{res}\n");
        guess.clear();
        lives -= 1;
        println!("{}/3 lives\n", lives.to_string());
    }
}
