use std::println;

pub fn predict_next_number(numbers: Vec<i32>) -> i32 {
    let mut sequences: Vec<Vec<i32>> = vec![numbers];

    let mut i = 0;
    loop {
        let current_sequence = sequences.get(i).unwrap();
        if current_sequence.iter().all(|n| n == &0) {
            break;
        }

        let next_sequence = current_sequence
            .windows(2)
            .map(|window| window.get(1).unwrap() - window.get(0).unwrap())
            .collect();

        sequences.push(next_sequence);

        i += 1;
    }

    for i in (0..i).rev() {
        // Needs to be cloned for `error[E0716]: temporary value dropped while borrowed`
        let calculated_value = sequences.get(i + 1).unwrap().last().unwrap().clone();
        let sequence_to_predict = sequences.get_mut(i).unwrap();

        let latest_value = sequence_to_predict.last().unwrap();

        sequence_to_predict.push(latest_value + calculated_value)
    }

    sequences.first().unwrap().last().unwrap().to_owned()
}

pub fn predict_prev_number(numbers: Vec<i32>) -> i32 {
    let mut sequences: Vec<Vec<i32>> = vec![numbers];

    let mut i = 0;
    loop {
        let current_sequence = sequences.get(i).unwrap();
        if current_sequence.iter().all(|n| n == &0) {
            break;
        }

        let next_sequence = current_sequence
            .windows(2)
            .map(|window| window.get(1).unwrap() - window.get(0).unwrap())
            .collect();

        sequences.push(next_sequence);

        i += 1;
    }

    for i in (0..i).rev() {
        let calculated_value = sequences.get(i + 1).unwrap().first().unwrap().clone();
        let sequence_to_predict = sequences.get_mut(i).unwrap();

        let earliest_value = sequence_to_predict.first().unwrap().clone();

        // Dirty push to front
        sequence_to_predict.reverse();
        sequence_to_predict.push(earliest_value - calculated_value);
        sequence_to_predict.reverse();
    }

    sequences.first().unwrap().first().unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::predict_next_number;
    use super::predict_prev_number;

    #[test]
    fn test_predict_next_number() {
        assert_eq!(predict_next_number(vec![0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(predict_next_number(vec![1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(predict_next_number(vec![10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn test_predict_prev_number() {
        assert_eq!(predict_prev_number(vec![10, 13, 16, 21, 30, 45]), 5);
        assert_eq!(predict_prev_number(vec![0, 3, 6, 9, 12, 15]), -3);
    }
}
