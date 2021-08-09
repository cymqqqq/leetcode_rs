use std::cmp::Ordering::*;
pub fn search(num: Vec<i32>, tar: i32) -> i32 {
    let n = num.len();
    let mut l: usize = 0;
    let mut r: usize = num.len() - 1;
    while l <= r {
        let mid = (l + r) >> 1;
        match num[mid].cmp(&tar) {
            Equal => {
                return mid as i32;
            }
            Less => {
                if mid + 1 > n - 1 {
                    break;
                }
                l = mid + 1;
            }
            Greater => {
                if mid < 1 {
                    break;
                }
                r = mid - 1;
            }
        }
    }
    -1
}
