
const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn main() {
    let mut temp_f: f64 = 32.0;

    println!("{temp_f}°F is {:.2}°C", fahrenheit_to_celsius(temp_f));

    for _ in 0..5 {
        temp_f += 1.0;
        println!("{temp_f}°F is {:.2}°C", fahrenheit_to_celsius(temp_f));
    }
}
