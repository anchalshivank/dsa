impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {

        let mut buy_at = prices[0];

        let mut max_p = i32::MIN;

        for (index, curr_sell_at)  in prices[1..].iter().enumerate(){
            let i = index+1;
            if *curr_sell_at > buy_at {

                //means we can sell at 
                let profit = *curr_sell_at - buy_at;
                max_p = max_p.max(profit);


            }else{
                buy_at = *curr_sell_at;
            }
        }
        if max_p == i32::MIN{
            return 0;            
        }
        max_p
        
    }
}