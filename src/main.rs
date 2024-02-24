use std::io;

enum Mode {
    Celc,
    Far,
}

fn get_cel_from_far(far: &f64) -> f64 {
    (far - 32.0) * (5.0 / 9.0)
}

fn get_far_from_cel(cel: &f64) -> f64 {
    cel * (9.0 / 5.0) + 32.0
}

fn main() {
    let far_symbol = "f".to_owned();
    let cel_symbol = "c".to_owned();

    let source: Mode;

    println!(
        "You want to put celcius of fahrenheits? {}/{}",
        cel_symbol, far_symbol
    );

    loop {
        let mut mode = String::new();
        io::stdin().read_line(&mut mode).expect("An error while reading temperature sign");

        mode = String::from(mode.trim());

        if mode == far_symbol {
            source = Mode::Far;
            break;
        }

        if mode == cel_symbol {
            source = Mode::Celc;
            break;
        }

        println!("Please, press only {} or {}", far_symbol, cel_symbol)
    }

    println!("Type numeric value of choosen source");

    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("An error when typing value");
    let numeric_value: f64 = value.trim().parse().unwrap();

    match source {
        Mode::Celc => {
            let result = get_far_from_cel(&numeric_value);
            println!("It is {} in fahrenheits", result)
        }
        Mode::Far => {
            let result = get_cel_from_far(&numeric_value);
            println!("It is {} in celcius", result)
        }
    }
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
