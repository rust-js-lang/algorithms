/// Finds the maximum profit from selling and buying the stocks,
/// by dynamic programming. `visit` is called at each step.
pub fn dp_best_time_to_buy_sell_stocks(prices: &[f64], visit: Option<&dyn Fn()>) -> f64 {
    let visit = visit.unwrap_or(&|| {});
    visit();
    let mut last_buy = -prices[0];
    let mut last_sold = 0.0;

    for &price in &prices[1..] {
        visit();
        let cur_buy = last_buy.max(last_sold - price);
        let cur_sold = last_sold.max(last_buy + price);
        last_buy = cur_buy;
        last_sold = cur_sold;
    }

    last_sold
}

js::export_default!(dp_best_time_to_buy_sell_stocks);
