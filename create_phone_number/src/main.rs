fn create_phone_number(numbers: &[u8]) -> String {
    let ddd = &numbers[0..3];
    let n = &numbers[3..10];
    let current_number = format!("({:?}{:?}{:?}) {:?}{:?}{:?}-{:?}{:?}{:?}{:?}", ddd[0], ddd[1], ddd[2], n[0], n[1], n[2], n[3], n[4], n[5], n[6]);
    current_number
}


fn main() {
    create_phone_number(&[1,2,3,4,5,6,7,8,9,0]);
}
