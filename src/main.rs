// 在 src/main.rs 文件中

// 1. 引入我们自己库中的 mapreduce 模块
use word_count::mapreduce;
use std::collections::HashMap;
use std::thread;

fn main() {
    // === 准备数据 ===
    // 假设这是我们要处理的海量数据，我们把它预先分成了 5 个数据块（chunk）
    // 在真实世界里，这些数据块可能位于不同的机器上
    let data_chunks = vec![
        "Rust is a systems programming language that runs blazingly fast,",
        "prevents segfaults, and guarantees thread safety.",
        "Hello world, hello Rust community!",
        "A journey of a thousand miles begins with a single step.",
        "Fast, reliable, and productive: pick three.",
    ];
    println!("--- 任务开始：待处理的数据块 ---");
    for (i, chunk) in data_chunks.iter().enumerate() {
        println!("块 {}: \"{}\"", i + 1, chunk);
    }
    println!("-------------------------------------\n");

    // === MAP 阶段 ===
    // 我们将使用 Rust 的“作用域线程” (scoped threads) 来安全地并行处理数据
    // 这是一种非常现代和安全的并发编程方式
    let map_results_from_threads = thread::scope(|s| {
        let mut handles = vec![];

        // 为每一个数据块都创建一个新的线程去处理
        for chunk in &data_chunks {
            let handle = s.spawn(move || {
                // 在这个线程里，调用我们的 map 函数！
                mapreduce::map(chunk)
            });
            handles.push(handle);
        }

        // 收集所有 mapper 线程的处理结果
        handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .collect::<Vec<_>>()
    });

    println!("--- MAP 阶段完成：所有线程已返回结果 ---\n");

    // === SHUFFLE & GROUP 阶段 ===
    // 此刻，`map_results_from_threads` 是一个 Vec<Vec<(String, i32)>>
    // 也就是 [[("rust", 1), ("is", 1), ...], [("hello", 1), ("world", 1), ...]]
    // 我们需要先把这个“列表的列表”压平成一个“长列表”
    let flattened_map_results: Vec<(String, i32)> =
        map_results_from_threads.into_iter().flatten().collect();

    // 调用我们写好的分组函数
    let grouped_data = mapreduce::shuffle_and_group(flattened_map_results);
    println!("--- SHUFFLE & GROUP 阶段完成：数据已按单词分组 ---\n");

    // === REDUCE 阶段 ===
    let mut final_word_counts = HashMap::new();
    for (word, counts) in grouped_data {
        // 调用 reduce 函数计算最终总数
        let final_count = mapreduce::reduce(&word, counts);
        final_word_counts.insert(word, final_count);
    }
    println!("--- REDUCE 阶段完成：最终词频已计算 ---\n");


    // === 展示结果 ===
    // 为了美观，我们按词频从高到低排序后输出
    let mut sorted_counts: Vec<_> = final_word_counts.into_iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(&a.1));

    println!("--- 最终词频统计结果 (降序) ---");
    for (word, count) in sorted_counts {
        println!("{:>12}: {}", word, count);
    }
}