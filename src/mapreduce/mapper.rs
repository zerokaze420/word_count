
pub fn map(contents: &str) -> Vec<(String,i32)> {
    contents
    .split_whitespace()
    .map(|word | word.to_lowercase().chars().filter(| c| c.is_alphabetic()).collect::<String>())
    .filter(|word| !word.is_empty())
    .map(| word | (word,1))
    .collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_map_function() {
        let text = "Hello, world! Hello Rust.";
        let mut expected = vec![
            ("hello".to_string(), 1),
            ("world".to_string(), 1),
            ("hello".to_string(), 1),
            ("rust".to_string(), 1),
        ];

        let mut result = map(text);

        result.sort();
        expected.sort();
        
        assert_eq!(result, expected);
    }

  
}