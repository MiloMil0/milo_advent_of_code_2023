pub fn calculate_differences(sequence: &[i32]) -> Vec<Vec<i32>> {
    if sequence.iter().all(|num| *num == 0) {
        vec![sequence.to_vec()]
    } else {
        let diff = sequence
            .windows(2)
            .map(|win| win[1] - win[0])
            .collect::<Vec<i32>>();
        let mut differences = vec![sequence.to_vec()];
        differences.extend(calculate_differences(&diff));
        differences
    }
}

pub fn get_final_number(differences: &[Vec<i32>]) -> i32 {
    let mut sum = 0;
    for difference in differences.iter().rev() {
        sum += difference.last().unwrap_or(&0);
    }
    sum
}
