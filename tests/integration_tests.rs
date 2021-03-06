use std::time::Instant;

use decision_tree::{
    classifier::calculate_accuracy,
    data_reader::{get_feature_names, read_csv_data},
    decision_tree::{print_tree, DecisionTree},
    split_finder::{SplitFinder, SplitMetric},
};

#[test]
fn test_decision_tree_for_iris() {
    let data_set = read_csv_data("./data-files/iris.csv");
    let split_finder = SplitFinder::new(SplitMetric::Variance);
    let tree = DecisionTree::new(data_set, split_finder, 3, false);
    let boxed_tree = Box::new(tree.root);
    let test_set = read_csv_data("./data-files/iris_test.csv");
    let accuracy = calculate_accuracy(&test_set, &boxed_tree);
    assert_eq!(accuracy, 1.0);
}

#[test]
fn test_decision_tree_for_synthetic() {
    let data_set = read_csv_data("./data-files/synthetic_1.csv");
    let split_finder = SplitFinder::new(SplitMetric::Variance);
    let tree = DecisionTree::new(data_set, split_finder, 3, false);
    let boxed_tree = Box::new(tree.root);
    let test_set = read_csv_data("./data-files/synthetic_1.csv");
    let accuracy = calculate_accuracy(&test_set, &boxed_tree);
    assert_eq!(accuracy, 1.0);
}

#[test]
fn test_decision_tree_for_digits() {
    let data_set = read_csv_data("./data-files/digits_train.csv");
    let split_finder = SplitFinder::new(SplitMetric::Variance);
    let tree = DecisionTree::new(data_set, split_finder, 10, false);
    let boxed_tree = Box::new(tree.root);
    let test_set = read_csv_data("./data-files/digits_test.csv");
    let accuracy = calculate_accuracy(&test_set, &boxed_tree);
    assert!(accuracy > 0.75)
}

#[test]
fn test_decision_tree_for_wine() {
    let data_set = read_csv_data("./data-files/wine_train.csv");
    let split_finder = SplitFinder::new(SplitMetric::Variance);
    let tree = DecisionTree::new(data_set, split_finder, 3, false);
    let boxed_tree = Box::new(tree.root);
    let test_set = read_csv_data("./data-files/wine_test.csv");
    let accuracy = calculate_accuracy(&test_set, &boxed_tree);
    assert!(accuracy > 0.90)
}

#[test]
fn test_decision_tree_for_covtype() {
    let data_set = read_csv_data("./data-files/covtype_train.csv");
    let split_finder = SplitFinder::new(SplitMetric::Variance);
    let before = Instant::now();
    let tree = DecisionTree::new(data_set, split_finder, 7, false);
    println!("Elapsed time: {:.2?}", before.elapsed());
    let boxed_tree = Box::new(tree.root);
    let test_set = read_csv_data("./data-files/covtype_test.csv");
    let accuracy = calculate_accuracy(&test_set, &boxed_tree);
    println!("{}", accuracy);
    assert!(accuracy > 0.90)
}

#[test]
fn test_decision_tree_for_covtype_multi_threaded() {
    let data_set = read_csv_data("./data-files/covtype_train.csv");
    let split_finder = SplitFinder::new(SplitMetric::Variance);
    let before = Instant::now();
    let tree = DecisionTree::new(data_set, split_finder, 7, true);
    println!("Elapsed time: {:.2?}", before.elapsed());
    let boxed_tree = Box::new(tree.root);
    let test_set = read_csv_data("./data-files/covtype_test.csv");
    let accuracy = calculate_accuracy(&test_set, &boxed_tree);
    assert!(accuracy > 0.90)
}

#[test]
fn print_tree_for_wine() {
    let data_set = read_csv_data("./data-files/wine_train.csv");
    let split_finder = SplitFinder::new(SplitMetric::Variance);
    let tree = DecisionTree::new(data_set, split_finder, 3, false);
    let feature_names = get_feature_names("./data-files/wine_train.csv");
    print_tree(Box::new(tree.root), "".to_string(), &feature_names);
}

#[test]
fn print_tree_for_synthetic() {
    let data_set = read_csv_data("./data-files/synthetic_1.csv");
    let split_finder = SplitFinder::new(SplitMetric::Variance);
    let tree = DecisionTree::new(data_set, split_finder, 2, false);
    let feature_names = get_feature_names("./data-files/synthetic_1.csv");
    print_tree(Box::new(tree.root), "".to_string(), &feature_names);
}
