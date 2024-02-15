struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut chars = s.chars();

        while chars.clone().count() > 0 {
            let first_char = loop {
                if let Some(valid_char) = chars.next() {
                    if valid_char.is_alphanumeric() {
                        break valid_char.to_ascii_lowercase()
                    }
                } else {
                    break ' '
                }
            };

            let last_char = loop {
                if let Some(valid_char) = chars.next_back() {
                    if valid_char.is_alphanumeric() {
                        break valid_char.to_ascii_lowercase()
                    }
                } else {
                    break first_char
                }
            };

            if first_char != last_char {
                return false
            }
        }

        true
    }
}

fn main() {
    let input: String = String::from("A man, a plan, a canal: Panama");
    println!("The result is {}", Solution::is_palindrome(input));
}
