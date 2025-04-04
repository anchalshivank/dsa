impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {

        fn is_valid(
            row: usize, 
            col: usize, 
            cols: &Vec<bool>, 
            n: usize,
            d1: &Vec<bool>, 
            d2: &Vec<bool>
        ) -> bool {
            !cols[col] && !d1[row + col] && !d2[(row as i32 - col as i32 + (n - 1) as i32) as usize]
        }

        fn backtrack(
            row: usize, 
            n: usize, 
            board: &mut Vec<Vec<char>>, 
            cols: &mut Vec<bool>, 
            d1: &mut Vec<bool>, 
            d2: &mut Vec<bool>, 
            result: &mut Vec<Vec<String>>
        ) {
            if row == n {
                result.push(board.iter().map(|r| r.iter().collect()).collect());
                return;
            }

            for col in 0..n {
                if is_valid(row, col, cols,n, d1, d2) {
                    board[row][col] = 'Q';
                    cols[col] = true;
                    d1[row + col] = true;
                    d2[(row as i32 - col as i32 + (n - 1) as i32) as usize] = true;

                    backtrack(row + 1, n, board, cols, d1, d2, result);

                    board[row][col] = '.';
                    cols[col] = false;
                    d1[row + col] = false;
                    d2[(row as i32 - col as i32 + (n - 1) as i32) as usize] = false;
                }
            }
        }

        let mut result = Vec::new();
        let mut board = vec![vec!['.'; n as usize]; n as usize];
        let mut cols = vec![false; n as usize];
        let mut d1 = vec![false; 2 * n as usize - 1];
        let mut d2 = vec![false; 2 * n as usize - 1];

        backtrack(0, n as usize, &mut board, &mut cols, &mut d1, &mut d2, &mut result);
        result
    }
}
