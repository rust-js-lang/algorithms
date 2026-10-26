/// Recursive Staircase Problem (Brute Force Solution): the number of ways
/// to climb a staircase of `stairs_num` stairs.
pub fn recursive_staircase_bf(stairs_num: i32) -> f64 {
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

    // Sum up how many steps we need to take after doing one step up with the number of
    // steps we need to take after doing two steps up.
    recursive_staircase_bf(stairs_num - 1) + recursive_staircase_bf(stairs_num - 2)
}

js::export_default!(recursive_staircase_bf);
