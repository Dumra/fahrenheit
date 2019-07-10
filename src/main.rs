use std::io;

const WELCOME_MESSAGE: &str = "Welcome to converter application between Fahrenheit and Celsius";
const FAHRENHEIT_LITERAL: &str = "f";
const CELSIUS_LITERAL: &str = "c";
const EXIT_LITERAL: &str = "e";
const EXIT: &str = "exit";
const FAHRENHEIT: &str = "fahrenheit";
const CELSIUS: &str = "celsius";

fn main() {
    println!("{}", WELCOME_MESSAGE);

    loop {
        println!("Type `C` to convert from Celsius to Fahrenheit. \nType `F` to vice versa. \nType `E` to exit");
        println!("Please input temperature scale");
        let mut input_temperature_literal = String::new();

        io::stdin()
            .read_line(&mut input_temperature_literal)
            .expect("Failed to read temperature scale");

        let input_temperature_scale = match input_temperature_literal.trim().to_lowercase().as_str()
        {
            FAHRENHEIT_LITERAL | FAHRENHEIT => FAHRENHEIT,
            CELSIUS_LITERAL | CELSIUS => CELSIUS,
            EXIT_LITERAL | EXIT => break,
            _ => {
                println!("Incorrect temperature scale. Try again");
                continue;
            }
        };

        println!("Please input temperature in {}", input_temperature_scale);

        let mut input_temperature = String::new();
        io::stdin()
            .read_line(&mut input_temperature)
            .expect("Failed to read temperature value");

        let input_temperature: f64 = match input_temperature.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Incorrect temperature value. Try again");
                continue;
            }
        };

        let convert_to = match input_temperature_scale {
            FAHRENHEIT => CELSIUS,
            CELSIUS => FAHRENHEIT,
            _ => "Unknown value",
        };

        println!(
            "U try to convert {} in {} to the {} value",
            input_temperature, input_temperature_scale, convert_to
        );

        println!(
            "The temperature is: {} in {}",
            (abstract_converter(input_temperature_scale, input_temperature) * 10.0).round() / 10.0,
            convert_to
        )
    }
}

fn abstract_converter(type_converter: &str, value: f64) -> f64 {
    let converted_value: f64 = match type_converter {
        FAHRENHEIT => fahrenheit_to_celsius(value),
        CELSIUS => celsius_to_fahrenheit(value),
        _ => 0.0,
    };

    converted_value
}

fn celsius_to_fahrenheit(value: f64) -> f64 {
    value * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(value: f64) -> f64 {
    (value - 32.0) * 5.0 / 9.0
}
