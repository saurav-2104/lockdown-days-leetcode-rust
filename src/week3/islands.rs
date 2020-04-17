// Given a 2d grid map of '1's (land) and '0's (water), count the number of islands. An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.

// Example 1:

// Input:
// 11110
// 11010
// 11000
// 00000

// Output: 1

impl Solution {
	pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
		let mut grid = grid;
		if grid.is_empty() {
			return 0 as i32;
		}
		let r = grid.len();
		let c = grid[0].len();

		fn dfs(i: usize, j: usize, v: &mut Vec<Vec<char>>, r: usize, c: usize) {
			if i < 0 || j < 0 || i >= r || j >= c || v[i][j] == '0' {
				return;
			}
			v[i][j] = '0';
			dfs(i + 1, j, v, r, c);
			dfs(i - 1, j, v, r, c);
			dfs(i, j + 1, v, r, c);
			dfs(i, j - 1, v, r, c);
		}

		let mut count = 0;
		for i in 0..r {
			for j in 0..c {
				if grid[i][j] != '0' {
					dfs(i, j, &mut grid, r, c);
					count += 1;
				}
			}
		}
		count
	}
}
