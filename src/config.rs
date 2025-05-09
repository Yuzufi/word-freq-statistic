use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
    /// 输入语料文件名，应位于程序所在目录
    input_file: String,
    /// 输出结果文件名，将输出到程序所在目录；若文件已存在，则覆盖
    output_file: String,
    /// 词的字数（范围为1-255）
    pub(crate) word_length: u8,
    /// 词频阈值：低于此值的词将被忽略
    pub(crate) freq_threshold: usize,
    /// 字符过滤方式：false则使用UTF-8值范围和额外字符，true则使用正则表达式
    pub(crate) use_regex: bool,
    /// UTF-8值范围下限：19968即\u4e00，即"一"字，会被包含
    pub(crate) lower_limit: usize,
    /// UTF-8值范围上限：40959即\u9fff，即"鿿"字，会被包含
    pub(crate) upper_limit: usize,
    /// UTF-8值范围外的额外字符，如逗号、句号、空格、生僻字等
    extra_chars: String,
    /// 正则表达式；若use_regex为false，则忽略此项
    regex: String,
}
