use crate::highlight;

pub struct FileType {
    name: String,
    hl_opts: highlight::Options,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("plain"),
            hl_opts: highlight::Options::default(),
        }
    }
}

impl FileType {
    pub fn from(file_name: &str) -> Self {
        let file_ext = file_name.split('.').last().unwrap_or("plain");
        let name = match file_ext {
            "rs" => "rust",
            _ => "plain",
        };

        Self {
            name: name.to_string(),
            hl_opts: highlight::Options::from(name),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn highlight_options(&self) -> &highlight::Options {
        &self.hl_opts
    }
}
