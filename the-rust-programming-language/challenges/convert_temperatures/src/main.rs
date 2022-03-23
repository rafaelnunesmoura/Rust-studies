use std::io;

fn main() {
    // Wellcome to the app
    println!("Wellcome to the Temperature Converter!");
    println!("Please digit 1 to convert the temperature to Celsius.
        or 2 for Fahrenheit");
    let user = input();

    // Conditional conversion
    if user != 1.0 {
        println!("Conveting Celsius to Fahrenheit");
        println!("Please enter the temperature that you want to convert.");
        let celsius_input = input();
        let fahrenheit = convert_to_fahrenheit(celsius_input);
        println!(
            "{} degrees celcius is {} fahrenheit.",
            celsius_input, fahrenheit
        );
    } else {
        println!("Converting fahrenheit to Celsius");
        println!("Please enter the fahrenheit that you want to convert.");
        let fahrenheit_input = input();
        let celsius = convert_to_celsius(fahrenheit_input);
        println!(
            "{} degrees fahrenheit is {} degrees celcius.",
            fahrenheit_input, celsius
        );
    }

}

fn input() -> f32 {
    loop {
        let mut input_string = String::new();
        println!("Enter Number: ");

        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read line");

        let _input_string = match input_string.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
}

 fn convert_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 1.8) + 32.0
}

 fn convert_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) / 1.8

}
