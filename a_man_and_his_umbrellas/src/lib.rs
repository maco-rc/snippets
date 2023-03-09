mod solution;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(solution::min_umbrellas(&["cloudy"]), 0);
        assert_eq!(solution::min_umbrellas(&["rainy", "rainy", "rainy", "rainy"]), 1);
        assert_eq!(solution::min_umbrellas(&["overcast", "rainy", "clear", "thunderstorms"]), 2);
    }
}
