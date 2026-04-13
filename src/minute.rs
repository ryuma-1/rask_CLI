use regex::Regex;
use anyhow::{Context, Result, anyhow};
use strum::Display;
use clap::ValueEnum;

use crate::doc::DocUrl;


#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Display)]
pub enum MinuteType {
    GN,
    New,
    Other,
}

#[derive(Debug, Clone, Copy)]
pub struct MinuteNum {
    pub num: i32,
}

pub struct Minute {
    m_type: MinuteType,
    number: MinuteNum,
    url: DocUrl,
}

// トレイトの戻り値を anyhow::Result に変更
pub trait FromString {
    fn new(s: &str) -> Result<Self> where Self: Sized;
    fn to_string(&self) -> String;
}



impl FromString for MinuteNum {
    fn new(s: &str) -> Result<Self> {
        // 1. 正規表現の定義
        let re = Regex::new(r"第(\d+)回").unwrap();

        // 2. キャプチャの実行
        // context() を使うことで、失敗時のメッセージを簡単に追加できます
        let caps = re.captures(s)
            .with_context(|| format!("「第{{数字}}回」の形式で見つかりませんでした: {}", s))?;

        // 3. 数字の抽出とパース
        // caps[1] は &str なので parse() が必要
        let num: i32 = caps[1]
            .parse()
            .with_context(|| format!("数字部分 '{}' を数値に変換できませんでした", &caps[1]))?;

        Ok(MinuteNum { num })
    }

    fn to_string(&self) -> String {
        format!("第{}回", self.num)
    }
}

impl MinuteNum{
    pub fn value(&self) -> i32 {
        self.num
    }
}

impl Minute {

    pub fn new(m_type: MinuteType, number: MinuteNum, url: DocUrl) -> Self {

        Self { m_type, number, url }
    }

    pub fn m_type(&self) -> MinuteType {
        self.m_type
    }

    pub fn num(&self) -> MinuteNum {
        self.number.clone()
    }

    pub fn url(&self) -> DocUrl {
        self.url.clone()
    }
}
