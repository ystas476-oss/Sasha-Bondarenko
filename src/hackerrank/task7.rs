pub fn get_total_x(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let mut count = 0;

    let start = *a.iter().max().unwrap();
    let end = *b.iter().min().unwrap();

    for x in start..=end {
        let valid_a = a.iter().all(|&i| x % i == 0);
        let valid_b = b.iter().all(|&j| j % x == 0);

        if valid_a && valid_b {
            count += 1;
        }
    }

    count
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(get_total_x(vec![2, 4], vec![16, 32, 96]), 3);
    }
}