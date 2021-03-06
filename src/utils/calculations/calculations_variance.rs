pub fn calculate_loss(
    left_variance: f64,
    right_variance: f64,
    left_size: f64,
    right_size: f64,
) -> f64 {
    let total_size = left_size + right_size;
    ((left_size / total_size) * left_variance) + ((right_size / total_size) * right_variance)
}

pub fn calculate_variance(
    sum_of_squared_labels: f64,
    mean_of_labels: f64,
    number_of_labels: f64,
) -> f64 {
    if number_of_labels == 0.0 {
        return 0.0;
    }
    let left = sum_of_squared_labels;
    let right = number_of_labels * (mean_of_labels * mean_of_labels);
    let variance = (left - right) / number_of_labels;
    variance
}

pub(super) fn get_label_sums(labels: &Vec<f64>) -> (f64, f64) {
    let mut sum_of_labels = 0.0;
    let mut sum_of_squared_labels = 0.0;
    labels.iter().for_each(|label| {
        let label_value = *label;
        sum_of_labels += label_value;
        sum_of_squared_labels += label_value * label_value;
    });
    (sum_of_labels, sum_of_squared_labels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_variance_calculations() {
        let data = vec![vec![1, 2, 1], vec![2, 2, 2], vec![4, 2, 3], vec![5, 2, 4]];
        let true_variance = test_calculation_functions::variance(&data);
        let calulated_variance = calculate_variance(30.0, 2.5, 4.0);
        assert_eq!(true_variance, calulated_variance);
    }

    mod test_calculation_functions {
        pub fn variance(data: &Vec<Vec<i32>>) -> f64 {
            let mean = output_mean(data);
            let mut sum_differences_squared = 0.0;
            data.iter().for_each(|row| {
                let output_value = row[row.len() - 1];
                let difference = output_value as f64 - mean;
                sum_differences_squared += difference * difference;
            });
            sum_differences_squared / data.len() as f64
        }

        fn output_mean(data: &Vec<Vec<i32>>) -> f64 {
            let mut sum = 0.0;
            data.iter().for_each(|row| {
                let output_value = row[row.len() - 1];
                sum += output_value as f64;
            });
            sum / data.len() as f64
        }
    }
}
