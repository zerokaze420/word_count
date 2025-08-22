use std::collections::HashMap;


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

        // 为了确保测试的稳定性，我们对结果进行排序，因为 hashmap 或并发处理可能不保证顺序
        result.sort();
        expected.sort();
        
        // 比较结果是否和预期一致
        assert_eq!(result, expected);
    }
}