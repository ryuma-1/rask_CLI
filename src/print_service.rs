use anyhow::{Context, Result};

pub trait Printable {
    fn get_print_fields(&self) -> Vec<PrintField>;
}

pub trait PrintableList {
    fn get_printable_list(&self) -> Vec<Box<dyn Printable>>;
}

pub struct PrintField {
    pub key: String,
    pub value: String,
}

impl PrintField {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

pub struct PrintService;

impl PrintService {
    pub fn new() -> Self {
        Self
    }

    pub fn print(&self, data: &dyn Printable) -> Result<()> {
        for field in data.get_print_fields() {
            println!("{}: {}", field.key, field.value);
        }
        Ok(())
    }

    pub fn print_json(&self, data: &dyn Printable) -> Result<()> {
        let map: std::collections::HashMap<String, String> = data
            .get_print_fields()
            .into_iter()
            .map(|f| (f.key, f.value))
            .collect();

        let json =
            serde_json::to_string_pretty(&map).context("JSONのシリアライズに失敗しました")?;

        println!("{}", json);
        Ok(())
    }

    pub fn print_list(&self, data: &dyn PrintableList) -> Result<()> {
        for item in data.get_printable_list() {
            self.print(item.as_ref())?;
            println!("--------------------");
        }
        Ok(())
    }

    pub fn print_list_json(&self, data: &dyn PrintableList) -> Result<()> {
        let list: Vec<std::collections::HashMap<String, String>> = data
            .get_printable_list()
            .into_iter()
            .map(|item| {
                item.get_print_fields()
                    .into_iter()
                    .map(|f| (f.key, f.value))
                    .collect()
            })
            .collect();

        let json =
            serde_json::to_string_pretty(&list).context("JSONのシリアライズに失敗しました")?;

        println!("{}", json);
        Ok(())
    }
}
