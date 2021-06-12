pub fn palimdrome(num: i32) -> bool {
    if num < 0 { return false; }
    else {
        let x_str = num.abs().to_string();
        if x_str == x_str.chars().rev().collect::<String>() {
            return true;
        } else { return false; }
    }
    
}
fn main() {
    let x = 2120;
    println!("{:?}", palimdrome(x));
}
