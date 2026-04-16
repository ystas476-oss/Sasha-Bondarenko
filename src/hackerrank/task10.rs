pub fn sock_merchant(arr: &[i32]) -> i32 {
    use std::collections::HashMap;

    let mut counts = HashMap::new();

    for &sock in arr {
        *counts.entry(sock).or_insert(0) += 1;
    }

    counts.values().map(|c| c / 2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let arr = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant(&arr), 3);
    }

    #[test]
    fn test_simple() {
        let arr = vec![1, 1, 2, 2, 3];
        assert_eq!(sock_merchant(&arr), 2);
    }
}