pub fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary = 0;
    let mut secondary = 0;

    for i in 0..n {
        primary += arr[i][i];
        secondary += arr[i][n - 1 - i];
    }

    (primary - secondary).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let arr = vec![
            vec![11, 2, 4],
            vec![4, 5, 6],
            vec![10, 8, -12],
        ];
        assert_eq!(diagonal_difference(&arr), 15);
    }

    #[test]
    fn test_small() {
        let arr = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        assert_eq!(diagonal_difference(&arr), 0);
    }
}