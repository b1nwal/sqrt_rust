use std::io;

fn main() {
    let radicand = get_input();
    let mut prevguess: f32 = 0.0;
    let mut guess: f32 = radicand / 2.0;
    loop {
        if check(radicand,guess) {
            println!("Answer is {}",guess);
            break;
        } else {
            if guess == prevguess {
                panic!("FATAL: Hit infinite loop.");
            }
            prevguess = guess;
            guess = (guess+radicand/guess)/2.0;
        }
    }
    println!("Exit Program.");
}

fn get_input() -> f32 {
    println!("Please Enter a Number: ");
    let mut s = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut s)
        .unwrap();
    let x = s
        .trim_end()
        .to_string();
    let a: f32 = match x.parse() {
        Ok(b) => b,
        Err(_) => {println!("Invalid Number. Try Again.");get_input()}
    };
    return a;
}

fn check(radicand: f32, guess: f32) -> bool {
    if (radicand / guess) == guess {
        return true;
    } else {
        return false;
    }
}
