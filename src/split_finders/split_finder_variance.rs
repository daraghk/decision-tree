#[path = "threshold_finders/threshold_finder_variance.rs"]
mod threshold_finder_variance;

use crate::{
    calculations_variance::{calculate_variance, get_label_sums},
    dataset::DataSet,
    question::Question,
    results::BestSplitResult,
};

pub fn find_best_split(data: &DataSet, number_of_classes: u32) -> BestSplitResult {
    let mut best_gain = 0.0;
    let mut best_question = Question::new(0, 0.);

    let label_sums = get_label_sums(&data.labels);
    let sum_of_labels = label_sums.0;
    let sum_of_squared_labels = label_sums.1;
    let total_variance = get_total_variance(
        sum_of_labels,
        sum_of_squared_labels,
        data.labels.len() as f64,
    );

    let number_of_features = data.features[0].len();
    for i in 0..number_of_features {
        let best_threshold_for_feature = threshold_finder_variance::determine_best_threshold(
            data,
            i as u32,
            sum_of_squared_labels,
            sum_of_labels,
        );

        let information_gain = total_variance - best_threshold_for_feature.loss;
        if information_gain > best_gain {
            best_gain = information_gain;
            best_question.column = i as u32;
            best_question.value = best_threshold_for_feature.threshold_value;
        }
    }

    BestSplitResult {
        gain: best_gain,
        question: best_question,
    }
}

fn get_total_variance(
    sum_of_labels: f64,
    sum_of_squared_labels: f64,
    number_of_labels: f64,
) -> f64 {
    let mean_of_labels = sum_of_labels / number_of_labels;
    calculate_variance(sum_of_squared_labels, mean_of_labels, number_of_labels)
}
