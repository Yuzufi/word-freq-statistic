use crate::config::Config;
use anyhow::Error;
use dashmap::DashMap;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};

pub(crate) fn run() -> Result<(), Error> {
    println!("载入配置文件...");
    let config = Config::load()?;
    let word_length = config.word_length;
    let is_valid_char = config.get_judge_char_func()?;
    println!("配置文件载入成功！");

    println!("第一轮统计...");
    let comb_freq: DashMap<String, AtomicUsize> = DashMap::with_capacity(65536);
    config.par_for_each_input_line(|line| {
        let mut heads: VecDeque<usize> = VecDeque::with_capacity(word_length);
        for (tail, c) in line.char_indices() {
            if heads.len() == word_length {
                let head = heads.pop_front().unwrap();
                count_comb(&comb_freq, &line, head, tail);
            }
            if is_valid_char(c) {
                heads.push_back(tail);
            } else {
                heads.clear();
            }
        }
        if heads.len() == word_length {
            let head = heads.pop_front().unwrap();
            let tail = line.len();
            count_comb(&comb_freq, &line, head, tail);
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
    let word_freq: DashMap<String, AtomicUsize> = DashMap::with_capacity(comb_freq.len());
    config.par_for_each_input_line(|line| {
        let mut heads: VecDeque<usize> = VecDeque::with_capacity(word_length);
        for (tail, c) in line.char_indices() {
            if heads.len() == 2 * word_length - 1 {
                count_word(&word_freq, &comb_freq, &line, &mut heads, tail, word_length);
            }
            if is_valid_char(c) {
                heads.push_back(tail);
            } else {
                if heads.len() >= word_length {
                    count_word(&word_freq, &comb_freq, &line, &mut heads, tail, word_length);
                }
                heads.clear();
            }
        }
        if heads.len() >= word_length {
            let tail = line.len();
            count_word(&word_freq, &comb_freq, &line, &mut heads, tail, word_length);
        }
    })?;
    if config.freq_threshold > 0 {
        word_freq.retain(|_, freq| freq.load(Ordering::Relaxed) >= config.freq_threshold);
    }
    println!("第二轮统计完成！筛选后共有 {} 个词。", word_freq.len());

    println!("整理并输出结果...");
    let mut result: Vec<(String, usize)> = word_freq
        .into_iter()
        .map(|(word, freq)| (word, freq.load(Ordering::Relaxed)))
        .collect();
    result.sort_by(|(_, a_freq), (_, b_freq)| b_freq.cmp(a_freq));
    let report = result
        .into_iter()
        .map(|(word, freq)| format!("{word}\t{freq}"))
        .collect::<Vec<_>>()
        .join("\n");
    config.output_bytes(report.as_bytes())?;
    println!("结果输出完成！");

    Ok(())
}

fn count_comb(comb_freq: &DashMap<String, AtomicUsize>, line: &str, head: usize, tail: usize) {
    match comb_freq.get_mut(&line[head..tail]) {
        Some(freq) => _ = freq.fetch_add(1, Ordering::Relaxed),
        None => _ = comb_freq.insert(line[head..tail].to_string(), AtomicUsize::new(1)),
    }
}

fn count_word(
    word_freq: &DashMap<String, AtomicUsize>,
    comb_freq: &DashMap<String, AtomicUsize>,
    line: &str,
    heads: &mut VecDeque<usize>,
    tail: usize,
    word_length: usize,
) {
    let mut max_word: &str = "";
    let mut max_freq: usize = 0;
    for _ in 0..(heads.len() - word_length + 1) {
        let head = heads.pop_front().unwrap();
        let comb_tail = if heads.len() < word_length {
            tail
        } else {
            heads[word_length - 1]
        };
        if let Some(freq) = comb_freq.get(&line[head..comb_tail]) {
            let freq = freq.load(Ordering::Relaxed);
            if freq > max_freq {
                max_word = &line[head..comb_tail];
                max_freq = freq;
            }
        }
    }
    if max_freq > 0 {
        match word_freq.get_mut(max_word) {
            Some(freq) => _ = freq.fetch_add(1, Ordering::Relaxed),
            None => _ = word_freq.insert(max_word.to_string(), AtomicUsize::new(1)),
        }
    }
}
