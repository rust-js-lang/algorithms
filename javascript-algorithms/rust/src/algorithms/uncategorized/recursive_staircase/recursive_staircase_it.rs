/// Recursive Staircase Problem (Iterative Solution): the number of ways to
/// climb a staircase of `stairs_num` stairs.
pub fn recursive_staircase_it(stairs_num: i32) -> f64 {
    if stairs_num <= 0 {
        // There is no way to go down - you climb the stairs only upwards.
        // Also you don't need to do anything to stay on the 0th step.
        return 0.0;
    }

    // Init the number of ways to get to the 0th, 1st and 2nd steps.
    let mut steps = [1.0, 2.0];

    if stairs_num <= 2 {
        // Return the number of possible ways of how to get to the 1st or 2nd steps.
        return steps[stairs_num as usize - 1];
    }

    // Calculate the number of ways to get to the n'th step based on previous ones.
    // Comparing to Dynamic Programming solution we don't store info for all the steps but
    // rather for two previous ones only.
    for _ in 3..=stairs_num {
        steps = [steps[1], steps[0] + steps[1]];
    }

    // Return possible ways to get to the requested step.
    steps[1]
}

js::export_default!(recursive_staircase_it);
