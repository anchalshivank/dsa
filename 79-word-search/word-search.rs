impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        
        fn solve(
            i: i32,
            j: i32,
            ind: usize,
            m: usize,
            n: usize,
            word: &Vec<char>,
            board: &Vec<Vec<char>>,
            vis: &mut Vec<Vec<bool>>
        ) -> bool{

            if ind == word.len(){
                return true;
            }

            if i < 0 || j < 0 || i as usize >= m || j as usize >= n || vis[i as usize][j as usize] || board[i as usize][j as usize] !=word[ind]{

                return false;

            }

            vis[i as usize][j as usize] = true;

            if solve(i-1, j , ind+1, m, n , word, board, vis)
                || solve(i+1, j , ind+1, m , n , word, board, vis)
                || solve(i, j-1, ind+1, m , n , word, board, vis)
                || solve(i, j+1, ind+1, m , n , word, board, vis){
                    return true;
                }
            vis[i as usize][j as usize] = false;
            false

        }

        let m = board.len();
        let n = board[0].len();
        let  mut vis = vec![vec![false; n]; m ];
        let word : Vec<char> = word.chars().collect();
        for i in 0..m{
            for j in 0..n{
                if solve(i as i32 , j as i32 , 0, m , n, &word, &board, &mut vis){
                    return true;
                }
            }
        }
        false
    }
}