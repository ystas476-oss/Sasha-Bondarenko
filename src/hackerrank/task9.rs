pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = [0; 6]; // індекси 1..5

    for &bird in arr {
        counts[bird as usize] += 1;
    }

    let mut max_count = 0;
    let mut result = 0;

    for i in 1..=5 {
        if counts[i] > max_count {
            max_count = counts[i];
            result = i as i32;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds() {
        let arr = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(&arr), 4);
    }

    #[test]
    fn test_tie() {
        let arr = vec![1, 1, 2, 2, 3];
        assert_eq!(migratory_birds(&arr), 1);
    }
}