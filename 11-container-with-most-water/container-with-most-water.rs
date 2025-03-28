impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        
        
        let mut i =0;
        let mut j = height.len()-1;

        let mut area = i32::MIN;

        while i < j {

            let h1 = height[i];
            let h2 = height[j];

            if h1<h2{
                //we need to move i_++
                let vol = h1*(j as i32 -i as i32);
    
                area = area.max(vol);

                i+=1;
            
            }else{
                let vol = h2*(j as i32-i as i32);

                area = area.max(vol);

                j-=1;
            }


        }

        area

    }
}