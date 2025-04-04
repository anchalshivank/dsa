impl Solution {
    pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
        

        //first rule to keep in mind is that we do not know where to start from !!! 
        // we will brute force from any all location! and after visiting it we will mark the place 0 , as we shall not visit that again
        fn helper(
            grid: &mut Vec<Vec<i32>>,
            r: i32,
            c: i32,
            curr: i32
        ) -> i32{

            if r<0 || r>=grid.len() as i32 || c<0 || c>=grid[0].len() as i32  || grid[r as usize][c as usize] == 0 {
                return curr;

            }else{

                //Now here we have a choice to move to any place
                let curr = curr+ grid[r as usize][c as usize];
                //One more thing to keep in mind.. we shall not visit this place again!
                //mark place visited
                let t = grid[r as usize][c as usize];
                grid[r as usize][c as usize] = 0;
                //I think reverting this shall make no difference!!
                //these are four path //choosing any path will collect max coins
                let a = helper(grid, r+1, c, curr);
                let b = helper(grid, r-1, c, curr);
                let c1 = helper(grid, r, c+1, curr);
                let d = helper(grid, r, c-1, curr);
                //Now one more thing to keep in mind is that!!I shall collect coins from where every possible so I need to add them !
                grid[r as usize][c as usize] = t;
                a.max(b.max(c1.max(d)))
            }

        }


        //let's traverse the whole grid
        let mut ans = i32::MIN;

        for r in 0..grid.len(){
            for c in 0..grid[0].len(){

                //Now we need to keep 
                let a = helper(&mut grid, r as i32, c as i32, 0);
                ans = ans.max(a);
                // println!("{:?} collectect {}", grid, a);

            } 
        }

        ans




    }
}