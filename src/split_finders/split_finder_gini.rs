#[path = "threshold_finders/threshold_finder_gini.rs"]
mod threshold_finder_gini;
use crate::{
    calculations_gini::calculate_gini, class_counter::get_class_counts, dataset::DataSet,
    question::Question, results::BestSplitResult,
};

pub fn find_best_split(data: &DataSet, number_of_classes: u32) -> BestSplitResult {
    let mut best_gain = 0.0;
    let mut best_question = Question::new(0, 0.);

    let class_counts_all = get_class_counts(&data.labels, number_of_classes);
    let gini_all = calculate_gini(&class_counts_all, data.features.len() as f64);

    let number_of_features = data.features[0].len();
    for i in 0..number_of_features {
        let best_threshold_for_feature =
            threshold_finder_gini::determine_best_threshold(data, i as u32, &class_counts_all);

        // Information gain from splitting S into S1 and S2
        // IG(S) = Gini(S) - (|S1|/|S|)*Gini(S1) - (|S2|/|S|)*Gini(S2)
        let information_gain = gini_all - best_threshold_for_feature.loss;
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

#[cfg(test)]
mod tests {
    use crate::{data_reader::read_csv_data, dataset::DataSet};

    #[test]
    fn test_find_best_split_dummy() {
        let features = vec![vec![10., 2., 0.], vec![6., 2., 0.], vec![1., 2., 1.]];
        let labels = vec![0., 0., 1.];
        let data = DataSet { features, labels };
        let result = super::find_best_split(&data, 2);
        assert_eq!(result.question.value, 6.);
    }

    #[test]
    fn test_find_first_best_split_iris() {
        let iris = read_csv_data("./data-files/iris.csv");
        let result = super::find_best_split(&iris, 3);
        assert_eq!(result.question.column, 2);
        assert_eq!(result.question.value, 30.);
    }
}
