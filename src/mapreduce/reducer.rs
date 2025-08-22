
pub fn reduce(_word: &str, counts: Vec<i32>) -> i32 {
    counts.iter().sum()
}


// --- 在测试模块 `mod tests` 中，添加一个新的测试函数 ---
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_reduce_function() {
        let word = "hello";
        let counts = vec![1, 1, 1, 1]; // 假设 hello 出现了 4 次
        let expected_sum = 4;

        let result = reduce(word, counts);

        assert_eq!(result, expected_sum);
    }
}