impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        if prices.len() < 2 { return max_profit;}
        let mut buy = 0;
        let mut sell = -1;

        for i in 1..prices.len() {
            if prices[buy] < prices[i] {
                max_profit = max_profit.max(prices[i] - prices[buy]);
            }
            else if prices[buy] > prices[i] { buy = i;}
        }
        return max_profit;
    }
}
