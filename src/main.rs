mod config;
mod core;

fn main() {
    println!(
        "欢迎使用 Word Freq Statistic 词频统计工具！\n\
        版本号：0.1.0 (2025-05-10)\n\
        作者：Garth TB <g-art-h@outlook.com>\n\
        仓库地址：https://github.com/GarthTB/word-freq-statistic"
    );

    match core::run() {
        Ok(_) => println!("程序成功执行完毕！"),
        Err(e) => println!("程序出错，已中断！错误：{:?}", e),
    }

    println!("按回车键退出...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
