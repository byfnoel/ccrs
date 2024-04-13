use std::io;

fn is_valid(s: &str) -> bool {
    let len = s.len();
    for i in 0..len {
        if s.chars().nth(i).unwrap() < '0' || s.chars().nth(i).unwrap() > '9' {
            return false;
        }
    }
    return true;
}
