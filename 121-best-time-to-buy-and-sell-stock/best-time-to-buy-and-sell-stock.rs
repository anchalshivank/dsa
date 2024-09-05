impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut buy, mut sell) = (prices[0], 0);

        for &price in &prices[1..] {
            if price < buy {
                buy = price;
            } else if price - buy > sell {
                sell = price - buy;
            }
        }

        return sell;
    }
}
