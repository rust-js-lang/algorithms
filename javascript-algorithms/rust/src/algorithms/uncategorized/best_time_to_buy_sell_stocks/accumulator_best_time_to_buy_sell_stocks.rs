/// Finds the maximum profit from selling and buying the stocks:
/// each day's rise, accumulated. `visit` is called at each step.
pub fn accumulator_best_time_to_buy_sell_stocks(prices: &[f64], visit: Option<&dyn Fn()>) -> f64 {
    let visit = visit.unwrap_or(&|| {});
    visit();
    let mut profit = 0.0;
    for day in 1..prices.len() {
        visit();
        profit += (prices[day] - prices[day - 1]).max(0.0);
    }
    profit
}

js::export_default!(accumulator_best_time_to_buy_sell_stocks);
