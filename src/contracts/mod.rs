pub mod etherdelta;

use std::str;
use std::string::ToString;

use fixed_hash::clean_0x;

use error::{Error, ErrorKind};

pub trait NamedFunction {
    fn get_function(&self) -> ContractFunction;
}

pub enum ContractFunction {
    Immutable(String),
    Mutable(String)
}

impl ToString for ContractFunction {
    fn to_string(&self) -> String {
        match *self {
            ContractFunction::Immutable(ref s) => s.to_string(),
            ContractFunction::Mutable(ref s) => s.to_string()
        }
    }
}

pub fn clean_method_id(data: &str, num_params: usize) -> &str {
    let data = clean_0x(data);
    let expected_length = 64 * num_params;

    if data.len() == expected_length + 8 {
        &data[8..]
    } else {
        &data
    }
}

pub fn normalize_data(data: &str, num_params: usize) -> Result<Vec<String>, Error> {
    let data = clean_method_id(&data, num_params);
    let expected_length = 64 * num_params;

    if data.len() != expected_length {
        Err(ErrorKind::Decoder(format!("Couldn't normalize input: {}", &data)).into())
    } else {
        let args = data.as_bytes()
            .chunks(64)
            .map(|buf| unsafe { str::from_utf8_unchecked(buf).to_string() })
            .collect::<Vec<String>>();

        Ok(args)
    }
}

#[cfg(test)]
mod tests {
    use super::{normalize_data};

    #[test]
    fn cleans_and_normalizes_prefixed_data() {
        let data = include_str!("../../test_data/input_data_0x.txt");
        let cleaned_and_normalized_data = normalize_data(&data, 11).unwrap();

        assert_eq!(cleaned_and_normalized_data.len(), 11);
        assert!(cleaned_and_normalized_data.iter().all(|chunk| chunk.len() == 64));
    }

    #[test]
    fn cleans_and_normalizes_data_without_prefix() {
        let data = include_str!("../../test_data/input_data_no_prefix.txt");
        let cleaned_and_normalized_data = normalize_data(&data, 11).unwrap();

        assert_eq!(cleaned_and_normalized_data.len(), 11);
        assert!(cleaned_and_normalized_data.iter().all(|chunk| chunk.len() == 64));
    }
}