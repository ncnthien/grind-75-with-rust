use std::collections::HashMap;
struct Solution;

impl Solution {
    fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];
        let mut rules: HashMap<char, char> = HashMap::new();
        let opens: Vec<char> = vec!['(', '{', '['];
        rules.insert(')', '(');
        rules.insert('}', '{');
        rules.insert(']', '[');

        for char in s.chars() {
            if opens.contains(&char) {
                stack.push(char);
            } else {
                if let Some(open) = stack.pop() {
                    if let Some(open_from_rule) = rules.get(&char) {
                        if open != *open_from_rule {
                            return false;
                        }
                    }
                } else {
                    return false;
                }
            }
        }
        stack.is_empty()
    }
}

fn main() {
    let input: String = String::from("()[]}{");
    println!("The result is {}", Solution::is_valid(input));
}
