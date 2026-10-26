use std::collections::HashMap;

/// The number of ways to climb `stairs_num` stairs, each already counted
/// kept in `memo`.
fn get_steps(stairs_num: i32, memo: &mut HashMap<i32, f64>) -> f64 {
    if stairs_num <= 0 {
        // There is no way to go down - you climb the stairs only upwards.
        // Also if you're standing on the ground floor that you don't need to do any further steps.
        return 0.0;
    }

    if stairs_num == 1 {
        // There is only one way to go to the first step.
        return 1.0;
    }

    if stairs_num == 2 {
        // There are two ways to get to the second steps: (1 + 1) or (2).
        return 2.0;
    }

    // Avoid recursion for the steps that we've calculated recently.
    if let Some(&steps) = memo.get(&stairs_num) {
        return steps;
    }

    // Sum up how many steps we need to take after doing one step up with the number of
    // steps we need to take after doing two steps up.
    let steps = get_steps(stairs_num - 1, memo) + get_steps(stairs_num - 2, memo);
    memo.insert(stairs_num, steps);

    steps
}

/// Recursive Staircase Problem (Recursive Solution With Memoization): the
/// number of ways to climb a staircase of `total_stairs` stairs.
pub fn recursive_staircase_mem(total_stairs: i32) -> f64 {
    // Memo table that will hold all recursively calculated results to avoid calculating them
    // over and over again.
    let mut memo = HashMap::new();

    // Return possible ways to get to the requested step.
    get_steps(total_stairs, &mut memo)
}

js::export_default!(recursive_staircase_mem);
