pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = prices[0];
    let mut max_profit = 0;

    for price in prices {
        if price < min_price {
            min_price = price;
        }else if price - min_price > max_profit {
            max_profit = price -min_price;
        }
    }
    max_profit
        
}

pub fn max_profit_2(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    for i in 1..prices.len() {
        if prices[i] > prices[i - 1] {
            max_profit += prices[i] - prices[i - 1];
        }
    }
    max_profit
        
}