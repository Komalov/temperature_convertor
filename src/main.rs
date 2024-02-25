use core::num;
use std::io;

enum Mode {
    Celc,
    Far,
    None,
}

fn get_cel_from_far(far: &f64) -> f64 {
    (far - 32.0) * (5.0 / 9.0)
}

fn get_far_from_cel(cel: &f64) -> f64 {
    cel * (9.0 / 5.0) + 32.0
}

fn get_value_from_user() -> Result<f64, num::ParseFloatError> {
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("An error when typing value");
    let numeric_value: f64 = value.trim().parse()?;
    Ok(numeric_value)
}

struct Symbols {
    far: String,
    cel: String,
}

fn get_temp_sign_from_user(symbols: Symbols) -> Mode {
    let mut source = Mode::None;

    while matches!(source, Mode::None) {
        let mut mode = String::new();
        io::stdin()
            .read_line(&mut mode)
            .expect("An error while reading temperature sign");

        mode = String::from(mode.trim());

        match mode {
            _ if mode == symbols.far => source = Mode::Far,
            _ if mode == symbols.cel => source = Mode::Celc,
            _ => println!("Please, press only {} or {}", symbols.far, symbols.cel),
        };
    }

    source
}

fn handle_numeric_value(source: Mode) {
    let numeric_value = get_value_from_user().expect("Unable to parse given value");
    match source {
        Mode::Celc => {
            let result = get_far_from_cel(&numeric_value);
            println!("It is {} in fahrenheits", result)
        }
        Mode::Far => {
            let result = get_cel_from_far(&numeric_value);
            println!("It is {} in celcius", result)
        }
        _ => (),
    }
}

fn main() {
    let symbols = Symbols {
        far: "f".to_owned(),
        cel: "c".to_owned(),
    };

    println!(
        "You want to put celcius of fahrenheits? {}/{}",
        symbols.cel, symbols.far
    );

    let source: Mode = get_temp_sign_from_user(symbols);

    println!("Type numeric value of choosen source");

    handle_numeric_value(source);
}

#[cfg(test)]
mod tests {
    use crate::{get_cel_from_far, get_far_from_cel};

    #[test]
    fn c2f() {
        let x: f64 = -15.0;
        let result = get_far_from_cel(&x);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn f2c() {
        let x: f64 = 5.0;
        let result = get_cel_from_far(&x);
        assert_eq!(result, -15.0);
    }
}
