/*
Island Perimeter
You are given row x col grid representing a map where grid[i][j] = 1 represents land and grid[i][j] = 0 represents water.

Grid cells are connected horizontally/vertically (not diagonally). The grid is completely surrounded by water, and there is exactly one island (i.e., one or more connected land cells).

The island doesn't have "lakes", meaning the water inside isn't connected to the water around the island. One cell is a square with side length 1. The grid is rectangular, width and height don't exceed 100. Determine the perimeter of the island.
*/
pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m =grid[0].len();
    let mut sum = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 1 {
                if i > 0 && grid[i - 1][j] == 0 || i == 0 {
                    sum += 1;
                }
                if i < n - 1 && grid[i + 1][j] == 0 || i == n - 1 {
                    sum += 1;
                }
                if j > 0 && grid[i][j - 1] == 0 || j == 0 {
                    sum += 1;
                }
                if j < m - 1 && grid[i][j + 1] == 0 || j == m - 1 {
                    sum += 1;
                }
            }
        }
    }
    sum
}
fn main() {
    let grid: Vec<Vec<i32>> = vec![
        vec![0, 1, 0, 0],
        vec![1, 1, 1, 0],
        vec![0, 1, 0, 0],
        vec![1, 1, 0, 0],
    ];
    println!("{:?}", island_perimeter(grid));
}
