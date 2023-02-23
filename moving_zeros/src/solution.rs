/*
Write an algorithm that takes an array and moves all of the zeros to the end, preserving the order of the other elements.

moveZeros([false,1,0,1,2,0,1,3,"a"]) // returns[false,1,1,2,1,3,"a",0,0]
 */

pub fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut order: Vec<u8> = vec![];
    let mut zeros: Vec<u8> = vec![];

    for item in arr {
        if item > &0 {
            order.push(*item);
        } else {
            zeros.push(*item);
        }
    }

    let solve = [order, zeros].concat();

    solve
}
