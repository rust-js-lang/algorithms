/// Finds the maximum profit from selling and buying the stocks:
/// each valley to its next peak. `visit` is called at each step.
pub fn peakvalley_best_time_to_buy_sell_stocks(prices: &[f64], visit: Option<&dyn Fn()>) -> f64 {
    let visit = visit.unwrap_or(&|| {});
    visit();
    let mut profit = 0.0;
    let mut low = prices[0];
    let mut high = prices[0];

    for &current_price in &prices[1..] {
        visit();
        if current_price < high {
            // If price went down, we need to sell.
            profit += high - low;
            low = current_price;
            high = current_price;
        } else {
            // If price went up, we don't need to do anything but increase a high record.
            high = current_price;
        }
    }

    // In case if price went up during the last day
    // and we didn't have chance to sell inside the forEach loop.
    profit += high - low;

    profit
}

js::export_default!(peakvalley_best_time_to_buy_sell_stocks);
