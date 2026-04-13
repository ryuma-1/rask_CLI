pub enum MinuteType {
    GN,
    New,
    Other,
}

pub struct Minute {
    m_type: MinuteType,
    url: String,
    number: i32,
}

impl FromString for Minute {
    fn new(s: &String) -> Result<Self, String> {
        let m_type = if s.contains("GN") {
             MinuteType::GN
        } else if s.contains("New") {
             MinuteType::New
        } else {
            // デフォルトはNewとする
             MinuteType::Other
        };

        Ok(Self {
            m_type,
            url: s.clone(),
            number: 0, // 仮の値、実際にはURLから番号を抽出するロジックが必要
        })
    }

    fn to_string(&self) -> String {
        self.url.clone()
    }
}
