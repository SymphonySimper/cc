pub fn min_processing_time(mut processor_time: Vec<i32>, mut tasks: Vec<i32>) -> i32 {
    let processor_time_len = processor_time.len() - 1;
    tasks.sort();
    processor_time.sort();

    tasks
        .windows(4)
        .step_by(4)
        .enumerate()
        .fold(0, |mut max, (i, w)| {
            let current = processor_time[processor_time_len - i] + w.iter().max().unwrap();
            if current > max {
                max = current;
            }
            max
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            min_processing_time(vec![8, 10], vec![2, 2, 3, 1, 8, 7, 4, 5]),
            16
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            min_processing_time(vec![10, 20], vec![2, 3, 1, 2, 5, 8, 4, 3]),
            23
        );
    }
}
