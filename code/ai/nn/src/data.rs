use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, io::Read};

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    // Training inputs, consisting in a vector of two numbers
    pub training_inputs: Vec<[f64; 2]>,

    // Training outputs, consisting on the sum of the two numbers
    pub training_outputs: Vec<f64>,

    // Test inputs, consisting in a vector of two numbers
    pub test_inputs: Vec<[f64; 2]>,
}

// Get data from json, convert it to a data struct
pub fn get_data() -> Result<Data, Box<dyn Error>> {
    let mut file: File = File::open("./resources/data.json")?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;

    let data: Data = serde_json::from_str(&contents)?;
    Ok(data)
}
