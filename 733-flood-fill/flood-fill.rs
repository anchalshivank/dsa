impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        
        fn fill(
            image: &Vec<Vec<i32>>,
            sr: i32,
            sc: i32,
            color: i32,
            result: &mut Vec<Vec<i32>>,
            val: i32
        ){

            if sr <0 || sr>= image.len() as i32 || sc<0 || sc>=image[0].len() as i32 || result[sr as usize ][sc as usize] != val || result[sr as usize ][sc as usize] == color{

                return;

            }else{

                result[sr as usize][sc as usize] = color;
                fill(image, sr+1, sc, color, result, val);
                fill(image, sr-1, sc, color, result, val);
                fill(image, sr, sc+1, color, result, val);
                fill(image, sr, sc-1, color, result, val);

            }
        }

        let val = image[sr as usize][sc as usize];
        let mut result = image.clone();
        fill(&image,sr, sc, color, &mut result, val );
        result

    }
}