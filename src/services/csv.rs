use csv::Reader;
use std::io::Read;
use crate::models::ContactRecord;

pub fn process_csv<R: Read>(reader: R) -> Result<Vec<ContactRecord>, csv::Error> {
    let mut csv_reader = Reader::from_reader(reader);
    let records: Result<Vec<ContactRecord>, csv::Error> = csv_reader
        .deserialize()
        .collect();
    records
} 