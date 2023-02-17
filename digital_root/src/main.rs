pub fn digital_root(n: i64) -> i64 {
    let mut rng = 0;
    let mut number = n;
    while rng != 1 {
        let arr = spliting(number);
        rng = arr.len();
        number = 0;

        for num in 0..rng {
            number = number + arr[num];
        }

        println!("{:?}", arr);
        println!("{}", number);
    }
    number
}

//function to split a integer into a array
pub fn spliting(x: i64) -> Vec<i64> {
    let vec = x.to_string().chars().map(|c| c as i64 - 0x30).collect();

    vec
}

fn main() {
    digital_root(1872);
}
