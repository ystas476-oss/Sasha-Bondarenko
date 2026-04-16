pub fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: Vec<i32>,
    oranges: Vec<i32>,
) -> (i32, i32) {
    let apples_count = apples
        .into_iter()
        .map(|d| a + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    let oranges_count = oranges
        .into_iter()
        .map(|d| b + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    (apples_count, oranges_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apples_and_oranges() {
        let s = 7;
        let t = 11;
        let a = 5;
        let b = 15;
        let apples = vec![-2, 2, 1];
        let oranges = vec![5, -6];

        let result = count_apples_and_oranges(s, t, a, b, apples, oranges);
        assert_eq!(result, (1, 1));
    }
}