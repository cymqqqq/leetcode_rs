/*
Fizz Buzz
Write a program that outputs the string representation of numbers from 1 to n.

But for multiples of three it should output â€œFizzâ€ instead of the number and for the multiples of five output â€œBuzzâ€. For numbers which are multiples of both three and five output â€œFizzBuzzâ€.

Example:

n = 15,

Return:
[
    "1",
    "2",
    "Fizz",
    "4",
    "Buzz",
    "Fizz",
    "7",
    "8",
    "Fizz",
    "Buzz",
    "11",
    "Fizz",
    "13",
    "14",
    "FizzBuzz"
]

*/
pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut res = vec![];
    for i in 1..=n {
        let fizz = i % 3 == 0;
        let buzz = i % 5 == 0;
        let s = match (fizz, buzz) {
            (true, true) => "fizzbuzz".to_string(),
            (true, false) => "fizz".to_string(),
            (false, true) => "buzz".to_string(),
            (false, false) => format!("{}", i),
            
        };
        res.push(s);
        
    }
    res
}
fn main() {
    println!("{:?}", fizz_buzz(15));
}
