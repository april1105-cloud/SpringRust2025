fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret_number = 25;  // Hardcoded secret number
    let mut guess = 41;      // Initial guess
    let mut attempts = 0;

    loop {
        attempts += 1;
        let result = check_guess(guess, secret_number);

        if result == 0 {
            println!("Congratulations! {} is the correct number!", guess);
            break;
        } else if result == 1 {
            println!("{} is too high!", guess);
        } else {
            println!("{} is too low!", guess);
        }

        if result == 1 {
            guess -= 3; // Decrease the guess
        } else {
            guess += 5; // Increase the guess
        }
    }

    println!("You guessed the number in {} attempts.", attempts);
}