fn add(mut x : i32, mut y: i32) -> i32 {
    while y != 0 {
        let hold = x & y;

        x = x ^ y;

        y = hold << 1;
    }

    return x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_values() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(5, 19), 24);
        assert_eq!(add(23, 17), 40);
    }
    
    #[test]
    fn test_negative_values() {
        assert_eq!(add(-14, -16), -30);
        assert_eq!(add(-50, -176), -226);
        assert_eq!(add(-10, -29), -39);
    }
    
    #[test]
    fn test_mixture_values() {
        assert_eq!(add(-13, 13), 0);
        assert_eq!(add(-27, 18), -9);
        assert_eq!(add(-90, 30), -60);
    }
}
