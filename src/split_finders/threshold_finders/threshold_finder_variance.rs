use crate::{
    calculations_variance::{calculate_loss, calculate_variance},
    dataset::DataSet,
    feature_sorter::get_sorted_feature_tuple_vector,
    results::BestThresholdResult,
};

struct VarianceValueTracker {
    number_of_labels: f64,
    sum_of_squared_labels: f64,
    sum_of_labels: f64,
    mean_of_labels: f64,
}

pub(super) fn determine_best_threshold(
    data: &DataSet,
    column: u32,
    total_sum_of_squared_labels: f64,
    total_sum_of_labels: f64,
) -> BestThresholdResult {
    let mut best_result_container = BestThresholdResult {
        loss: f64::INFINITY,
        threshold_value: 0.0,
    };

    let mut left_value_tracker = VarianceValueTracker {
        number_of_labels: 0.0,
        sum_of_squared_labels: 0.0,
        sum_of_labels: 0.0,
        mean_of_labels: 0.0,
    };

    let right_mean_of_labels = total_sum_of_labels / data.labels.len() as f64;
    let mut right_value_tracker = VarianceValueTracker {
        number_of_labels: data.labels.len() as f64,
        sum_of_squared_labels: total_sum_of_squared_labels,
        sum_of_labels: total_sum_of_labels,
        mean_of_labels: right_mean_of_labels,
    };

    let sorted_feature_data = get_sorted_feature_tuple_vector(&data.features, column);
    let previous_feature_val = sorted_feature_data.get(0).unwrap().0;
    sorted_feature_data.iter().for_each(|tuple| {
        let feature_value = tuple.0;

        //only calculate 'loss' on first encounter of a feature value
        if feature_value != previous_feature_val {
            let left_variance = calculate_variance(
                left_value_tracker.sum_of_squared_labels,
                left_value_tracker.mean_of_labels,
                left_value_tracker.number_of_labels,
            );
            let right_variance = calculate_variance(
                right_value_tracker.sum_of_squared_labels,
                right_value_tracker.mean_of_labels,
                right_value_tracker.number_of_labels,
            );
            let split_variance = calculate_loss(
                left_variance,
                right_variance,
                left_value_tracker.number_of_labels,
                right_value_tracker.number_of_labels,
            );
            if split_variance < best_result_container.loss {
                best_result_container.loss = split_variance;
                best_result_container.threshold_value = feature_value;
            }
        }

        let real_row_index = tuple.1;
        let label_value = *data.labels.get(real_row_index as usize).unwrap();
        update_left_value_tracker(&mut left_value_tracker, label_value);
        update_right_value_tracker(&mut right_value_tracker, label_value);
    });
    best_result_container
}

fn update_left_value_tracker(left_value_tracker: &mut VarianceValueTracker, label_value: f64) {
    left_value_tracker.sum_of_squared_labels += label_value * label_value;
    left_value_tracker.number_of_labels += 1.0;
    left_value_tracker.sum_of_labels += label_value;
    left_value_tracker.mean_of_labels =
        left_value_tracker.sum_of_labels / left_value_tracker.number_of_labels;
}

fn update_right_value_tracker(right_value_tracker: &mut VarianceValueTracker, label_value: f64) {
    right_value_tracker.sum_of_squared_labels -= label_value * label_value;
    right_value_tracker.number_of_labels -= 1.0;
    right_value_tracker.sum_of_labels -= label_value;
    right_value_tracker.mean_of_labels =
        right_value_tracker.sum_of_labels / right_value_tracker.number_of_labels;
}

mod tests {
    use crate::{
        calculations_variance::get_label_sums, data_reader::read_csv_data, dataset::DataSet,
    };

    #[test]
    fn test_best_threshold_for_particular_feature() {
        let features = vec![vec![10., 2., 0.], vec![6., 2., 0.], vec![1., 2., 1.]];
        let labels = vec![0., 0., 1.];
        let data = DataSet { features, labels };
        let column = 0;
        let best = super::determine_best_threshold(&data, column, 1.0, 1.0);
        assert_eq!(best.loss, 0.0);
        assert_eq!(best.threshold_value, 6.0);
    }

    #[test]
    fn test_best_threshold_for_particular_feature_in_iris() {
        let iris = read_csv_data("./data-files/iris.csv");
        let column = 2;
        let label_sums = get_label_sums(&iris.labels);
        let result = super::determine_best_threshold(&iris, column, label_sums.1, label_sums.0);
        assert_eq!(result.threshold_value, 30.0);
    }
}
