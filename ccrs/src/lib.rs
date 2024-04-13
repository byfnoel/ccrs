use std::io::stdin;

fn main() {
    let mut credit_card_number = String::new();
    loop {
        println!("You can quit at anytime.");
        println!("Type 'exit' to quit OR enter your credit card number to check validation.");
        credit_card_number.clear();
        stdin().read_line(&mut credit_card_number).unwrap();
        credit_card_number = credit_card_number.trim().to_string();
        if credit_card_number == "exit" {
            break;
        } else if !is_valid(&credit_card_number) {
            println!("Invalid input. Please enter a valid credit card number.");
            continue;
        }

        let len = credit_card_number.len();
        let mut double_even_sum = 0;
        for i in (0..len - 1).step_by(2) {
            let dbl = (credit_card_number.chars().nth(i).unwrap() as i32 - 48) * 2;
            if dbl > 9 {
                double_even_sum += dbl / 10 + dbl % 10;
            } else {
                double_even_sum += dbl;
            }
        }
        for i in (1..len).step_by(2) {
            double_even_sum += credit_card_number.chars().nth(i).unwrap() as i32 - 48;
        }
        println!(
            "{}",
            if double_even_sum % 10 == 0 {
                "Valid!"
            } else {
                "Invalid!"
            }
        );
        continue;
    }
}

fn is_valid(s: &str) -> bool {
    let len = s.len();
    for i in 0..len {
        if s.chars().nth(i).unwrap() < '0' || s.chars().nth(i).unwrap() > '9' {
            return false;
        }
    }
    return true;
}
