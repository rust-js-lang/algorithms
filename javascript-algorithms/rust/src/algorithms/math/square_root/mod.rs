/// The square root of `number` by Newton's method, to `tolerance` digits
/// after the point, none where it's left out.
pub fn square_root(number: f64, tolerance: Option<f64>) -> f64 {
    let tolerance = tolerance.unwrap_or(0.0);
    // For now we won't support operations that involves manipulation with complex numbers.
    if number < 0.0 {
        panic!("The method supports only positive integers");
    }

    // Handle edge case with finding the square root of zero.
    if number == 0.0 {
        return 0.0;
    }

    // We will start approximation from value 1.
    let mut root: f64 = 1.0;

    // Delta is a desired distance between the number and the square of the root.
    // - if tolerance=0 then delta=1
    // - if tolerance=1 then delta=0.1
    // - if tolerance=2 then delta=0.01
    // - and so on...
    let required_delta = 1.0 / 10f64.powf(tolerance);

    // Approximating the root value to the point when we get a desired precision.
    while (number - root.powi(2)).abs() > required_delta {
        // Newton's method reduces in this case to the so-called Babylonian method.
        // These methods generally yield approximate results, but can be made arbitrarily
        // precise by increasing the number of calculation steps.
        root -= (root.powi(2) - number) / (2.0 * root);
    }

    // Cut off undesired floating digits and return the root value.
    (root * 10f64.powf(tolerance)).round() / 10f64.powf(tolerance)
}

js::export_default!(square_root);
