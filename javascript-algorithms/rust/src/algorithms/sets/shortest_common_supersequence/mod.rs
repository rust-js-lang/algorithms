use crate::algorithms::sets::longest_common_subsequence::longest_common_subsequence;

/// The shortest sequence both sets are subsequences of.
pub fn shortest_common_supersequence(set1: &[&str], set2: &[&str]) -> Vec<String> {
    // Let's first find the longest common subsequence of two sets.
    let lcs = longest_common_subsequence(set1, set2);

    // If LCS is empty then the shortest common supersequence would be just
    // concatenation of two sequences.
    if lcs.len() == 1 && lcs[0].is_empty() {
        return set1
            .iter()
            .chain(set2)
            .map(|item| item.to_string())
            .collect();
    }

    // Now let's add elements of set1 and set2 in order before/inside/after the LCS.
    let mut supersequence = Vec::new();

    let mut set_index1 = 0;
    let mut set_index2 = 0;
    let mut lcs_index = 0;
    let mut set_on_hold1 = false;
    let mut set_on_hold2 = false;

    while lcs_index < lcs.len() {
        // Add elements of the first set to supersequence in correct order.
        if set_index1 < set1.len() {
            if !set_on_hold1 && set1[set_index1] != lcs[lcs_index] {
                supersequence.push(set1[set_index1].to_string());
                set_index1 += 1;
            } else {
                set_on_hold1 = true;
            }
        }

        // Add elements of the second set to supersequence in correct order.
        if set_index2 < set2.len() {
            if !set_on_hold2 && set2[set_index2] != lcs[lcs_index] {
                supersequence.push(set2[set_index2].to_string());
                set_index2 += 1;
            } else {
                set_on_hold2 = true;
            }
        }

        // Add LCS element to the supersequence in correct order.
        if set_on_hold1 && set_on_hold2 {
            supersequence.push(lcs[lcs_index].clone());
            lcs_index += 1;
            set_index1 += 1;
            set_index2 += 1;
            set_on_hold1 = false;
            set_on_hold2 = false;
        }
    }

    // Attach set1 leftovers.
    supersequence.extend(set1.iter().skip(set_index1).map(|item| item.to_string()));

    // Attach set2 leftovers.
    supersequence.extend(set2.iter().skip(set_index2).map(|item| item.to_string()));

    supersequence
}

js::export_default!(shortest_common_supersequence);
