/// Finds the maximum profit from selling and buying the stocks,
/// by trying each day's choice. `visit` is called at each step.
pub fn dq_best_time_to_buy_sell_stocks(prices: &[f64], visit: Option<&dyn Fn()>) -> f64 {
    let visit = visit.unwrap_or(&|| {});

    fn recursive_buyer_seller(prices: &[f64], visit: &dyn Fn(), buy: bool, day: usize) -> f64 {
        // Registering the recursive call visit to calculate the complexity.
        visit();

        // Quitting the recursion if this is the last day of trading (prices array ended).
        if day == prices.len() {
            return 0.0;
        }

        // If we're buying - we're loosing money (-1), if we're selling we're getting money (+1).
        let operation_sign = if buy { -1.0 } else { 1.0 };
        recursive_buyer_seller(prices, visit, buy, day + 1)
            // Option 1: Don't do anything.
            .max(
                // Option 2: Sell or Buy at the current price.
                operation_sign * prices[day] + recursive_buyer_seller(prices, visit, !buy, day + 1),
            )
    }

    let buy = true;
    let day = 0;

    recursive_buyer_seller(prices, visit, buy, day)
}

js::export_default!(dq_best_time_to_buy_sell_stocks);
