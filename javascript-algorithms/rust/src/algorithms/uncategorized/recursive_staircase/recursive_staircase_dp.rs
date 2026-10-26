/// Recursive Staircase Problem (Dynamic Programming Solution): the number
/// of ways to climb a staircase of `stairs_num` stairs.
pub fn recursive_staircase_dp(stairs_num: i32) -> f64 {
    let Ok(stairs_num) = usize::try_from(stairs_num) else {
        // There is no way to go down - you climb the stairs only upwards.
        return 0.0;
    };

    // Init the steps vector that will hold all possible ways to get to the corresponding step.
    let mut steps = vec![0.0; stairs_num.max(2) + 1];

    // Init the number of ways to get to the 0th, 1st and 2nd steps.
    steps[0] = 0.0;
    steps[1] = 1.0;
    steps[2] = 2.0;

    if stairs_num <= 2 {
        // Return the number of ways to get to the 0th or 1st or 2nd steps.
        return steps[stairs_num];
    }

    // Calculate every next step based on two previous ones.
    for current_step in 3..=stairs_num {
        steps[current_step] = steps[current_step - 1] + steps[current_step - 2];
    }

    // Return possible ways to get to the requested step.
    steps[stairs_num]
}

js::export_default!(recursive_staircase_dp);
