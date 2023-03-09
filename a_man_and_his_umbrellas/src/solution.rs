pub fn min_umbrellas(arr: &[&str]) -> usize {
    let mut home = 0;
    let mut work = 0;

    let bad_weather = vec!["rainy", "thunderstorms"];
    arr.iter().enumerate().for_each(|(index, weater)| {
        if bad_weather.contains(weater) {
            if index % 2 == 0 {
                work += 1;
                if home > 0 {
                    home -= 1
                }
            } else {
                home += 1;
                if work > 0 {
                    work -= 1
                }
            }
        }
    });

    home + work
}
