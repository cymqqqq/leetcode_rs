/*
Image Smoother
Given a 2D integer matrix M representing the gray scale of an image, you need to design a smoother to make the gray scale of each cell becomes the average gray scale (rounding down) of all the 8 surrounding cells and itself. If a cell has less than 8 surrounding cells, then use as many as you can.

Example 1:
Input:
[[1,1,1],
 [1,0,1],
 [1,1,1]]
Output:
[[0, 0, 0],
 [0, 0, 0],
 [0, 0, 0]]
Explanation:
For the point (0,0), (0,2), (2,0), (2,2): floor(3/4) = floor(0.75) = 0
For the point (0,1), (1,0), (1,2), (2,1): floor(5/6) = floor(0.83333333) = 0
For the point (1,1): floor(8/9) = floor(0.88888889) = 0
Note:
The value in the given matrix is in the range of [0, 255].
The length and width of the given matrix are in the range of [1, 150].
*/
pub fn smooth(m: &[Vec<i32>], r: usize, c: usize, h: usize, w: usize) -> i32 {
    let mut sum = 0;
    let mut n = 0;
    if r > 0 && c > 0 {
        sum += m[r - 1][c - 1];
        n += 1;
    }
    if r > 0 {
        sum += m[r - 1][c];
        n += 1;
    }
    if r > 0 && c < w - 1 {
        sum += m[r - 1][c + 1];
        n += 1;
    }
    if c > 0 {
        sum += m[r][c - 1];
        n += 1;
    }
    sum += m[r][c];
    n += 1;
    if c < w - 1 {
        sum += m[r][c + 1];
        n += 1;
    }
    if c < h - 1 && c > 0 {
        sum += m[r + 1][c - 1];
        n += 1;
    }
    if r < h - 1 {
        sum += m[r + 1][c];
        n += 1;
    }
    if r < h - 1 && c < w - 1 {
        sum += m[r + 1][c + 1];
        n += 1;
    }
    sum / n
}
pub fn image_smoother(m: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let h = m.len();
    let w = m[0].len();
    let mut res = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            res[i][j] = smooth(&m, i, j, h, w);
        }
    }
    res
}
