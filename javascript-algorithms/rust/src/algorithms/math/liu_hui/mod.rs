const CIRCLE_RADIUS: f64 = 1.0;

/// The side of the n-gon `split_counter` splits of `side_length`'s make.
fn get_n_gon_side_length(side_length: f64, split_counter: f64) -> f64 {
    if split_counter <= 0.0 {
        return side_length;
    }

    let half_side = side_length / 2.0;

    // Liu Hui used the Gou Gu (Pythagorean theorem) theorem repetitively.
    let perpendicular = (CIRCLE_RADIUS.powi(2) - half_side.powi(2)).sqrt();
    let excess_radius = CIRCLE_RADIUS - perpendicular;
    let split_side_length = (excess_radius.powi(2) + half_side.powi(2)).sqrt();

    get_n_gon_side_length(split_side_length, split_counter - 1.0)
}

/// How many sides the n-gon of `split_count` splits has.
fn get_n_gon_side_count(split_count: f64) -> f64 {
    // Liu Hui began with an inscribed hexagon (6-gon).
    let hexagon_sides_count = 6.0;

    // On every split iteration we make N-gons: 6-gon, 12-gon, 24-gon, 48-gon and so on.
    hexagon_sides_count
        * if split_count != 0.0 {
            2f64.powf(split_count)
        } else {
            1.0
        }
}

/// π, approximated by Liu Hui's n-gon of `split_count` splits, 1 where it's
/// left out.
pub fn liu_hui(split_count: Option<f64>) -> f64 {
    let split_count = split_count.unwrap_or(1.0);
    let n_gon_side_length = get_n_gon_side_length(CIRCLE_RADIUS, split_count - 1.0);
    let n_gon_side_count = get_n_gon_side_count(split_count - 1.0);
    let n_gon_perimeter = n_gon_side_length * n_gon_side_count;
    let approximate_circle_area = (n_gon_perimeter / 2.0) * CIRCLE_RADIUS;

    // Return approximate value of pi.
    approximate_circle_area / CIRCLE_RADIUS.powi(2)
}

js::export_default!(liu_hui);
