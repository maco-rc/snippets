struct Snail {
    row: usize,
    col: usize,
}

impl Snail {
    fn new(row: usize, col: usize) -> Snail {
        Snail { row: row, col: col }
    }

    fn snail_that(mut self, matrix: &mut [Vec<i32>]) -> Vec<i32> {
        let mut vec = Vec::new();
        let x = self.row * self.col;

        match matrix.len() {
            0 => vec![],
            1 => {
                if matrix.first().unwrap().is_empty() {
                    vec![]
                } else {
                    vec![*matrix.first().unwrap().first().unwrap()]
                }
            }
            _ => {
                while vec.len() != x {
                    for _n in 0..self.row {
                        vec.push(matrix[0].remove(0));
                    }
                    self.col -= 1;
                    matrix.rotate_left(1);

                    for n in 0..self.col {
                        vec.push(matrix[n].pop().unwrap())
                    }
                    for n in 0..matrix.len() {
                        matrix[n].reverse();
                    }
                    self.row -= 1;
                    if matrix[0].len() > 1 {
                        matrix.reverse();

                        while matrix[0].len() == 0 {
                            matrix.rotate_left(1);
                        }
                    }
                }
                println!("{:?}", vec);
                vec
            }
        }
    }
}

fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    //I prefer not to comment on how awful it is to manipulate a slice of VECTOR, well in a way works so its fine

    let col = matrix.len();
    let row = if col != 0 { matrix.first().unwrap().len() } else { 0 };

    let mut copy = vec![Vec::new(); col];
    copy.clone_from_slice(matrix);

    Snail::new(row, col).snail_that(&mut copy)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test1() {
        let square = &[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(snail(square), expected);
    }
    #[test]
    fn sample_test2() {
        let square = &[vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(snail(square), expected);
    }
    #[test]
    fn sample_test3() {
        let square: &[Vec<i32>; 1] = &[Vec::new()];
        let expected = Vec::new();
        assert_eq!(snail(square), expected, "Failed with empty input");
    }
    #[test]
    fn sample_test4() {
        let square = &[vec![1]];
        let expected = vec![1];
        assert_eq!(snail(square), expected);
    }
}
