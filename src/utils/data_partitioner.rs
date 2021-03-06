use crate::{dataset::DataSet, question::Question};

pub fn partition(data: &DataSet, question: &Question) -> (DataSet, DataSet) {
    let mut true_rows = vec![];
    let mut false_rows = vec![];
    let mut true_labels = vec![];
    let mut false_labels = vec![];

    let mut index: usize = 0;
    data.features.iter().for_each(|row| {
        let current_label = *data.labels.get(index).unwrap();
        index += 1;
        if question.solve(row) {
            true_rows.push(row.clone());
            true_labels.push(current_label);
        } else {
            false_rows.push(row.clone());
            false_labels.push(current_label);
        }
    });

    let false_data = DataSet {
        features: false_rows,
        labels: false_labels,
    };

    let true_data = DataSet {
        features: true_rows,
        labels: true_labels,
    };

    (false_data, true_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_all_false() {
        let features = vec![vec![2., 2., 0.], vec![2., 2., 1.], vec![2., 2., 2.]];
        let labels = vec![0., 1., 2.];
        let data = DataSet { features, labels };
        let question = Question::new(0, 2.);
        let partitioned_data = partition(&data, &question);
        println!("{:?}", partitioned_data);
        assert_eq!(partitioned_data.0.features.len(), 0);
        assert_eq!(partitioned_data.1.features.len(), 3);
    }

    #[test]
    fn test_partition_all_true() {
        let features = vec![vec![2., 2., 0.], vec![2., 2., 1.], vec![2., 2., 2.]];
        let labels = vec![0., 1., 2.];
        let data = DataSet { features, labels };
        let question = Question::new(0, 0.);
        let partitioned_data = partition(&data, &question);
        println!("{:?}", partitioned_data);
        assert_eq!(partitioned_data.0.features.len(), 0);
        assert_eq!(partitioned_data.1.features.len(), 3);
    }

    #[test]
    fn test_partition_even() {
        let features = vec![
            vec![1., 2., 0.],
            vec![2., 2., 1.],
            vec![4., 2., 2.],
            vec![5., 2., 2.],
        ];
        let labels = vec![0., 1., 2., 3.];
        let data = DataSet { features, labels };
        let question = Question::new(0, 3.);
        let partitioned_data = partition(&data, &question);
        println!("{:?}", partitioned_data);
        assert_eq!(partitioned_data.0.features.len(), 2);
        assert_eq!(partitioned_data.1.features.len(), 2);
    }
}
