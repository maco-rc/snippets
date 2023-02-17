fn what_century(year: &str) -> String {
    let century = year.parse::<f32>().unwrap() / 100.0;
    let rounded = century.ceil() as i32;

    let vec: Vec<u32> = rounded
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    if vec[0] == 1 {
        return format!("{}th", rounded)
    }
    match vec[1] {
        1 => return format!("{}st", rounded),
        2 => return format!("{}nd", rounded),
        3 => return format!("{}rd", rounded),
        _ => return format!("{}th", rounded),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_what_century() {
        assert_eq!(what_century("1234"), "13th")
    }
}
