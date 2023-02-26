#[derive(Debug)]
pub struct ConvertResult {
    pub as_array_b_nsize: Vec<bool>,
    pub as_u64: u64,
    pub as_string_hex: String,
    pub as_string_bin: String,
}

pub fn from_string_vec_binary(bin_str: String) -> Result<ConvertResult, String> {
    let b_vec = byte_string_to_b_vec(bin_str);
    let u64_conversion = u64::from_str_radix(&bin_str, 2);
    if u64_conversion.is_err() {
        let error = u64_conversion.err().unwrap();
        return Err(format!(
            "Cannot convert '{}' into u64 value because {}",
            bin_str, error
        ));
    }

    let as_u64 = u64_conversion.unwrap();

    Ok(ConvertResult {
        as_array_b_nsize: b_vec,
        as_u64,
        as_string_hex: format!("0x{:0width$X}", as_u64, width = b_vec.len() / 4),
        as_string_bin: str,
    })
}

pub fn byte_string_to_b_vec(byte_string: String) -> Vec<bool> {
    let mut result = Vec::new();
    for s in byte_string.chars().rev() {
        result.push(s == '1');
    }
    result
}

