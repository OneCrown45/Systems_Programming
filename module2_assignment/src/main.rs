fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {
    let mut i = low;
    while i <= high {
        *total += i;
        i += step;
    }
}

fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut max_word = String::new();
    let mut max_count = 0;

    for i in 0..words.len() {
        let mut count = 1;

        for j in (i + 1)..words.len() {
            if words[i] == words[j] {
                count += 1;
            }
        }

        if count > max_count {
            max_count = count;
            max_word = words[i].to_string();
        }
    }

    (max_word, max_count)
}

fn main() {
    let mut result = 0;
    sum_with_step(&mut result, 0, 100, 1);
    println!("Sum 0 to 100, step 1: {}", result);

    result = 0;
    sum_with_step(&mut result, 0, 10, 2);
    println!("Sum 0 to 10, step 2: {}", result);

    result = 0;
    sum_with_step(&mut result, 5, 15, 3);
    println!("Sum 5 to 15, step 3: {}", result);

    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
