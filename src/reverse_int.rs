//solution1
pub fn reverse(nums: i32) -> i32 {
    if nums == 0i32 { return 0; }
    else if nums > 0i32{
        let  hundred = nums / 100;
        let  res_hundred = nums % 100;
        let  ten = res_hundred / 10;
        let  res_ten = res_hundred % 10;
        return res_ten * 100 + ten * 10 + hundred;
    } else {
        let  hundred = -nums / 100;
        let  res_hundred = -nums % 100;
        let  ten = res_hundred / 10;
        let  res_ten = res_hundred % 10;
        return -(res_ten * 100 + ten * 10 + hundred);
    }
    
}
fn main() {
    let nums = 0;
    println!("{:?}", reverse(nums));
}
