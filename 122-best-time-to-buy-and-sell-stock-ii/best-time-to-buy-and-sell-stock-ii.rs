impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {

        let (mut buy, mut profit) = (prices[0], 0);

        for &price in &prices[1..]{
                
            if price > buy{
                profit +=price-buy;
   
            }
            buy = price;
    

        }

        profit
        
    }
}