use std::collections::HashMap;

fn count_frequency(content: &str) -> HashMap<char, usize> {
    let mut frequencies = HashMap::new();

    for c in content.chars() {
        *frequencies.entry(c).or_default() += 1;
    }

    frequencies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_frequency() {
        let content = "hello world";
        let frequencies = count_frequency(content);
        assert_eq!(frequencies.len(), 8);
        assert_eq!(frequencies[&'l'], 3);
        assert_eq!(frequencies[&'o'], 2);
    }

    #[test]
    fn test_count_frequency_les_miserables() {
        let content =
            std::fs::read_to_string("../test/test.txt").expect("Failed to read test file");

        let frequencies = count_frequency(&content);

        assert_eq!(frequencies[&'X'], 333, "uppercase X count mismatch");
        assert_eq!(frequencies[&'t'], 223_000, "lowercase t count mismatch");
    }
}
