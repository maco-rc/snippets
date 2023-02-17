pub fn solution(num: i32) -> i32 {
    let nums = [3, 5];
    let mut mult = Vec::new();
    let mut res;
    for i in 0..2 {
        for x in 1..num {
            res = nums[i] * x;
            if !mult.contains(&res) && res < num {
                mult.push(res);
            }
        }
    }
    //sum the entire vector
    let sum: i32 = mult.iter().sum();
    sum
}


fn main() {
    assert_eq!(solution(10), 23);
}
