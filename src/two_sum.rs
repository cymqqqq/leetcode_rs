pub fn two_sum(nums: &mut Vec<i32>, target: i32) -> Vec<i32> {
    let len = nums.len() as usize;
    let mut res: Vec<i32> = Vec::new();
    let mut n1 = 0;
    let mut n2 = 0;
    for i in 0..len - 1 {
        let mut j = i + 1;
        while j <= len - 1 {
            if nums[i] + nums[j] == target { n1 = i; n2 = j; }
            j += 1;
        }
        
        
    }
    res.push(n1 as i32);
    res.push(n2 as i32);
    res
}
fn main() {
    let mut nums1 = vec![3,2,4];
    println!("{:?}", two_sum(&mut nums1, 6));
}
