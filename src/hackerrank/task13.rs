pub fn divisible_sum_pairs(k: i32, ar: &[i32]) -> i32 {
    let mut count = 0;

    for i in 0..ar.len() {
        for j in i + 1..ar.len() {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let arr = vec![1, 3, 2, 6, 1, 2];
        assert_eq!(divisible_sum_pairs(3, &arr), 5);
    }

    #[test]
    fn test_simple() {
        let arr = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(divisible_sum_pairs(5, &arr), 3);
    }
}