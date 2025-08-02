fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers = [3, 5, 10, 15, 22, 30, 7, 9, 12, 18];

    // Part 3: Analyze each number
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

    // Part 4: Sum using while loop
    let mut i = 0;
    let mut sum = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum: {sum}");

    // Part 5: Find largest using loop
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
}

