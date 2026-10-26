use js::Unknown;

/// Each combination of `combo_length` of `combo_options`, an option any
/// number of times, in their order.
pub fn combine_with_repetitions<'a>(
    combo_options: &[&'a Unknown],
    combo_length: isize,
) -> Vec<Vec<&'a Unknown>> {
    // If the length of the combination is 0 then there is an empty combination.
    if combo_length == 0 {
        return vec![Vec::new()];
    }

    // If the length of the combination is negative, there are no combinations.
    if combo_length < 0 {
        return Vec::new();
    }

    // If the length of the combination is 1 then each element of the original array
    // is a combination itself.
    if combo_length == 1 {
        return combo_options
            .iter()
            .map(|&combo_option| vec![combo_option])
            .collect();
    }

    // Init combinations array.
    let mut combos = Vec::new();

    // Remember characters one by one and concatenate them to combinations of smaller lengths.
    // We don't extract elements here because the repetitions are allowed.
    for (option_index, &current_option) in combo_options.iter().enumerate() {
        // Generate combinations of smaller size.
        let smaller_combos =
            combine_with_repetitions(&combo_options[option_index..], combo_length - 1);

        // Concatenate currentOption with all combinations of smaller size.
        for smaller_combo in smaller_combos {
            combos.push([vec![current_option], smaller_combo].concat());
        }
    }

    combos
}

js::export_default!(combine_with_repetitions);
