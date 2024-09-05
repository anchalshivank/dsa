impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        
        let mut mbp = i32::MAX;
        let mut mp = 0;

        for price in prices{

            let profit = price-mbp;

            if profit > mp{
                mp = profit;
            }

            if price< mbp{
                mbp = price;
            }

        }



        mp


    }
}