use crate::config::Config;
use anyhow::Error;
use dashmap::DashMap;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};

pub(crate) fn run() -> Result<(), Error> {
    println!("载入配置文件...");
    let config = Config::load()?;
    let is_valid_char = config.get_judge_char_func()?;
    println!("配置文件载入成功！");

    println!("第一轮统计...");
    let comb_freq: DashMap<String, AtomicUsize> = DashMap::with_capacity(65536);
    config.par_for_each_input_line(|line| {
        let mut heads: VecDeque<usize> = VecDeque::with_capacity(config.word_length);
        for (tail, c) in line.char_indices() {
            if heads.len() == config.word_length {
                let head = heads.pop_front().unwrap();
                count_comb(&comb_freq, &line, head, tail);
            }
            if is_valid_char(c) {
                heads.push_back(tail);
            } else {
                heads.clear();
            }
        }
        if heads.len() == config.word_length {
            let head = heads.pop_front().unwrap();
            count_comb(&comb_freq, &line, head, line.len());
        }
    })?;
    if config.freq_threshold > 0 {
        comb_freq.retain(|_, freq| freq.load(Ordering::Relaxed) >= config.freq_threshold);
    }
    println!(
        "第一轮统计完成！筛选后共有 {} 个可能是词的字符组合。",
        comb_freq.len()
    );

    println!("第二轮统计...");
    config.par_for_each_input_line(|line| {})?;
    println!("第二轮统计完成！");

    println!("整理并输出结果...");

    println!("结果输出完成！");

    Ok(())
}

fn count_comb(comb_freq: &DashMap<String, AtomicUsize>, line: &str, head: usize, tail: usize) {
    match comb_freq.get_mut(&line[head..tail]) {
        Some(freq) => _ = freq.fetch_add(1, Ordering::Relaxed),
        None => _ = comb_freq.insert(line[head..tail].to_string(), AtomicUsize::new(1)),
    }
}
