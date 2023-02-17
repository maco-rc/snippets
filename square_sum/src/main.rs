fn square_sum(vec: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;

    for num in vec {
        sum += num * num;
    }

    sum
}

fn main() {
    assert_eq!(square_sum(vec![1, 2]), 5);
}
