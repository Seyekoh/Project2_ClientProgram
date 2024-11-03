use base64::{encode};

/// Encodes the given string to Base64 format.
///
/// # Arguments
/// * `input` - A string clice that holds the content to be encoded.
///
/// # Returns
/// A `String` containing the Base64 encoded version of the input.
pub fn encode_to_base64(input: &str) -> String {
    encode(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encod_to_base64() {
        let sample_input = "ALBNM, PROD001, 12, 2023-01-01";
        let expected_output = "QUxCTk0sIFBST0QwMDEsIDEyLCAyMDIzLTAxLTAx0";

        asser_eq!(encode_to_base64(sample_input), expected_output);
    }
}