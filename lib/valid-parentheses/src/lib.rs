/*
Given a string containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:

    Open brackets must be closed by the same type of brackets.
    Open brackets must be closed in the correct order.

Note that an empty string is also considered valid.

Example 1:

Input: "()"
Output: true

Example 2:

Input: "()[]{}"
Output: true

Example 3:

Input: "(]"
Output: false

Example 4:

Input: "([)]"
Output: false

Example 5:

Input: "{[]}"
Output: true


*/
pub fn is_valid(s: String) -> bool {
    use std::collections::HashMap;

    // match closing parentheses to their corresponding opener
    let mut completes = HashMap::new();
    completes.insert(')', '(');
    completes.insert('}', '{');
    completes.insert(']', '[');

    // use a stack and push values on as the occur, pop values as they are closed
    let mut stack: Vec<char> = Vec::new();
    // create Vec<char> from the argument, track previous value
    let values: Vec<char> = s.trim().chars().collect();
    let mut prev: char = ' ';

    for value in values {
        // is the value a closing paren, and does it close the most recent value
        if let Some(v) = completes.get(&value) {
            if v == &prev {
                let _ = stack.pop().unwrap();
                prev = match stack.len() {
                    0 => ' ',
                    _ => stack[stack.len() - 1],
                };
                continue;
            }
        }

        // otherwise push it on the stack
        prev = value;
        stack.push(value);
    }

    stack.len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_valid(String::from("{[()]}")), true);
    }

    #[test]
    fn it_works2() {
        assert_eq!(is_valid(String::from("{[}")), false);
    }

    #[test]
    fn it_works3() {
        assert_eq!(is_valid(String::from("{}[]()")), true);
    }
}
