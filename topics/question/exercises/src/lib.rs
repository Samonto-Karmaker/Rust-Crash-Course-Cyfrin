fn parse(s: &str) -> Result<u32, String> {
    match s.parse() {
        Ok(val) => Ok(val),
        Err(_) => Err("Failed to parse string into u32".to_string()),
    }
}

pub fn sum(num: &[&str]) -> Result<u32, String> {
    let mut total: u32 = 0;
    for s in num {
        total += parse(s)?;
    }
    Ok(total)
}
