pub fn staircase(n: i32) -> Vec<String> {
    let mut result = Vec::new();

    for i in 1..=n {
        let spaces = " ".repeat((n - i) as usize);
        let hashes = "#".repeat(i as usize);
        result.push(format!("{}{}", spaces, hashes));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase_6() {
        let expected = vec![
            "     #",
            "    ##",
            "   ###",
            "  ####",
            " #####",
            "######",
        ];

        let result = staircase(6);
        assert_eq!(result, expected);
    }
}