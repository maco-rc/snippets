fn parse(code: &str) -> Vec<i32> {
    let mut init = 0;
    let mut vec = Vec::new();

    for letter in code.chars() {
        match letter {
            'i' => {
                init = init + 1;
            }
            'd' => {
                init = init - 1;
            }
            's' => {
                init = i32::pow(init, 2);
            }
            'o' => {
                vec.push(init);
            }
            _ => continue,
        }
    }

    return vec;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_tests() {
        assert_eq!(parse("iiisdoso"), vec![8, 64]);
        assert_eq!(parse("iiisdosodddddiso"), vec![8, 64, 3600]);
    }
}

fn main() {}
