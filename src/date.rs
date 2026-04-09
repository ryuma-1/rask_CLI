use anyhow::{ensure, Result};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::task::FromString;

// --- 型定義 ---

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(transparent)]
struct Year(u16);

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(transparent)]
struct Month(u8);

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(transparent)]
struct Day(u8);

#[derive(Debug, Clone)] // 自動の Serialize/Deserialize は外す
pub struct Date {
    year: Year,
    month: Month,
    day: Day,
}

// --- 手動シリアライズ (Date -> "YYYY-MM-DD") ---

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // to_string() を利用して文字列に変換し、シリアライズする
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

// --- 手動デシリアライズ ("YYYY-MM-DD" -> Date { ... }) ---

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // 1. まず文字列として読み込む
        let s = String::deserialize(deserializer)?;
        
        // 2. FromString トレイトの new メソッドを呼び出す
        // map_err で String のエラーメッセージを Serde のエラー型に変換
        <Self as FromString>::new(&s).map_err(de::Error::custom)
    }
}

// --- 既存のロジック ---

impl Date {
    pub fn new(y: u16, m: u8, d: u8) -> Result<Self> {
        ensure!(m >= 1 && m <= 12, "月は1〜12の間である必要があります: {}", m);
        ensure!(d >= 1 && d <= 31, "日は1〜31の間である必要があります: {}", d);
        
        Ok(Self {
            year: Year(y),
            month: Month(m),
            day: Day(d),
        })
    }
}

impl FromString for Date {
    fn new(s: &String) -> Result<Self, String> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 {
            return Err("日付は YYYY-MM-DD 形式で入力してください。".to_string());
        }
        let year: u16 = parts[0].parse().map_err(|_| "年は数値で入力してください。".to_string())?;
        let month: u8 = parts[1].parse().map_err(|_| "月は数値で入力してください。".to_string())?;
        let day: u8 = parts[2].parse().map_err(|_| "日は数値で入力してください。".to_string())?;
        
        Self::new(year, month, day).map_err(|e| e.to_string())  
    }

    fn to_string(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year.0, self.month.0, self.day.0)
    }  
}
