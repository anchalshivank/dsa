impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {

        fn helper(
            grid: &mut Vec<Vec<i32>>,
            i: i32,
            j: i32,
            m: i32,
            n: i32
            ) -> i32 {

                if i < 0 || i >=m || j < 0 || j >=n{

                    return 0;

                }

                if grid[i as usize][j as usize] == 1{
                    grid[i as usize ][j as usize] = 0 ;
                    return  1 + helper(grid, i+1, j , m , n) + helper(grid, i-1, j, m , n) + helper(grid, i, j+1, m , n) + helper(grid, i , j-1, m , n);
                }else{
                    return 0;
                }

            }

        let mut grid = grid.clone();

        let m = grid.len();
        let n = grid[0].len();
        let mut area = i32::MIN;
        for i in 0..m{
            for j in 0..n{

                area = area.max(helper(&mut grid, i as i32 , j as i32 , m as i32 , n as i32));
                

            }
        }

        area


        
    }
}