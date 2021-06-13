/*
Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:

Open brackets must be closed by the same type of brackets.
Open brackets must be closed in the correct order.
*/
pub fn valid_parenthese(s: String) -> bool {
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => match stack.pop() {
                Some(t) => {
                    if !((t == '{' && c == '}')
                        || (t == '(' && c == ')')
                        || (t == '[' && c == ']'))
                    {
                        return false;
                    }
                }
                None => { return false; }
            },
            _ => {}
        }
        
    }
    stack.is_empty()
}
fn main() {
    let s = String::from("()");
    println!("{:?}", valid_parenthese(s));
}
