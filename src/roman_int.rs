use std::collections::HashMap;
pub fn roman_int(s: String) -> i32 {
    let map: HashMap<char, i32> = vec![
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]
    .into_iter()
    .collect();
    let mut sum = 0;
    let mut end = 0;
    for c in s.chars() {
        if let Some(&v) = map.get(&c) {
            if v > end {
                sum += v - end - end;
            } else {
                sum += v;
            }
            end = v;
        }
    }
    sum
}
fn main() {
    let s = String::from("IV");
    println!("{:?}", roman_int(s));
}
