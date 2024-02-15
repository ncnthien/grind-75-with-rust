struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit: i32 = 0;
        let mut min: i32 = prices[0];
        for price in prices {
            min = if price < min { price } else { min };
            let new_profit = price - min;
            profit = if new_profit > profit { new_profit } else { profit };
        }
        profit
    }
}

fn main() {
    let input: Vec<i32> = vec![7, 1, 5, 3, 6, 4];
    println!("The result is {}", Solution::max_profit(input));
}
