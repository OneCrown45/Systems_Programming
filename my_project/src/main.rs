const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

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
    let mut temp_f: f64 = 32.0;

    println!("{temp_f}°F is {:.2}°C", fahrenheit_to_celsius(temp_f));

    for _ in 0..5 {
        temp_f += 1.0;
        println!("{temp_f}°F is {:.2}°C", fahrenheit_to_celsius(temp_f));
    }
     let numbers = [3, 5, 10, 15, 22, 30, 7, 9, 12, 18];

    // Analyze each number
    for &num in numbers.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{num}: FizzBuzz");
        } else if num % 3 == 0 {
            println!("{num}: Fizz");
        } else if num % 5 == 0 {
            println!("{num}: Buzz");
        } else {
            let kind = if is_even(num) { "Even" } else { "Odd" };
            println!("{num}: {kind}");
        }
    }

    // Sum using while loop
    let mut i = 0;
    let mut sum = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum: {sum}");

    // Find largest using loop
    let mut max = numbers[0];
    let mut j = 1;
    loop {
        if j >= numbers.len() {
            break;
        }
        if numbers[j] > max {
            max = numbers[j];
        }
        j += 1;
    }
    println!("Largest: {max}");

    let secret = 42;
    let mut guess = 30; // Starting guess
    let mut attempts = 0;

    loop {
        attempts += 1;
        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Guess {guess} is correct!");
            break;
        } else if result == 1 {
            println!("Guess {guess} is too high.");
            guess -= 1;
        } else {
            println!("Guess {guess} is too low.");
            guess += 1;
        }
    }

    println!("You guessed the number in {attempts} attempts.");
}
