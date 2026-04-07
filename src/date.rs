use anyhow::{ensure, Result};

// メモリ効率の良い型を選択
#[derive(Clone, Copy)]
struct Year(u16);
#[derive(Clone, Copy)]
struct Month(u8);
#[derive(Clone, Copy)]
struct Day(u8);

#[derive(Clone)]
pub struct Date {
    year: Year,
    month: Month,
    day: Day,
}

impl Date {
    // 生成時にバリデーションを行う
    pub fn new(y: u16, m: u8, d: u8) -> Result<Self> {
        ensure!(m >= 1 && m <= 12, "月は1〜12の間である必要があります: {}", m);
        ensure!(d >= 1 && d <= 31, "日は1〜31の間である必要があります: {}", d);
        // 本来はうるう年などのロジックもここへ
        
        Ok(Self {
            year: Year(y),
            month: Month(m),
            day: Day(d),
        })
    }

    pub fn to_string(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year.0, self.month.0, self.day.0)
    }
}
