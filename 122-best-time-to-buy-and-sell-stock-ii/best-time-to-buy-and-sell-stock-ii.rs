impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        
        //just sell every time 
        //means add all increasing thing

        let mut bp = prices[0];

        let mut tp = 0;
        for &price in prices.iter(){

            if price>bp {
                //means we can sell 
                tp += price - bp;

            }
            bp = price;

        }

        tp

    }
}