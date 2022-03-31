use csv::StringRecord;
use std::error::Error;
use std::fs::File;

use crate::dataset::DataSet;

pub fn read_csv_data(file_path: &str) -> DataSet {
    let data_set_read = read_data(file_path).unwrap();
    parse_data_into_features_and_labels(data_set_read)
}

pub fn get_feature_names(file_path: &str) -> Vec<String> {
    let feature_names = get_header_record(file_path).unwrap();
    let mut names_vec = vec![];
    for i in 0..feature_names.len() {
        names_vec.push(feature_names.get(i).unwrap().to_owned());
    }
    names_vec
}

fn get_header_record(file_path: &str) -> Result<StringRecord, Box<dyn Error>> {
    //feature names should be in the header of the csv file
    let file = File::open(file_path)?;
    let mut reader = csv::Reader::from_reader(file);
    let headers = reader.headers()?;
    Ok(headers.to_owned())
}

//reading in data from csv, presume header included and label is at the end of each record
fn read_data(file_path: &str) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut data = vec![];
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize() {
        let record: Vec<f64> = result?;
        data.push(record);
    }
    Ok(data)
}

fn parse_data_into_features_and_labels(data_set: Vec<Vec<f64>>) -> DataSet {
    let mut features = vec![];
    let mut labels = vec![];
    data_set.iter().for_each(|row| {
        let mut copy = row.clone();
        let label = copy.pop().unwrap();
        labels.push(label);
        features.push(copy);
    });
    DataSet { features, labels }
}

pub fn create_feature_columns(data_set_features: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut columns = vec![];
    for col in 0..data_set_features[0].len() {
        let mut column = vec![];
        for row in 0..data_set_features.len() {
            column.push(data_set_features[row][col]);
        }
        columns.push(column);
    }
    columns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_csv_reading() {
        let data_set = read_csv_data("./data-files/iris.csv");
        println!("{:?}", data_set);
    }

    #[test]
    fn print_get_feature_names() {
        let feature_names = get_header_record("./data-files/iris.csv");
        let names_unwrapped = feature_names.unwrap();
        let mut names_vec = vec![];
        for i in 0..names_unwrapped.len() - 1 {
            names_vec.push(names_unwrapped.get(i).unwrap());
        }
        println!("{:?}", names_vec)
    }
}
