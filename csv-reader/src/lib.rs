use std::{error::Error, fs};

pub struct CSVReader {
    filename: String,
    headers: Vec<String>,
    records: Vec<Vec<String>>,
}

impl CSVReader {
    pub fn new(filename: String, use_headers: bool) -> Result<CSVReader, Box<dyn Error>> {
        let file = fs::read_to_string(filename.clone())?;

        let mut headers = Vec::new();
        let mut records = Vec::new();

        let mut lines = file.lines();

        if use_headers {
            if let Some(first_row) = lines.next() {
                first_row
                    .split(',')
                    .map(|v| v.trim())
                    .for_each(|column| headers.push(String::from(column)));
            }
        }

        lines.for_each(|line| {
            records.push(Vec::from_iter(
                line.split(',')
                    .map(|v| v.trim())
                    .map(|val| String::from(val)),
            ));
        });

        Ok(CSVReader {
            filename,
            headers,
            records,
        })
    }

    pub fn filename(&self) -> &String {
        return &self.filename;
    }
    pub fn headers(&self) -> &Vec<String> {
        &self.headers
    }
    pub fn records(&self) -> &Vec<Vec<String>> {
        &self.records
    }
}
