use crate::viewport::Viewport;

pub struct Buffer {
    path: Option<String>,
    pub lines: Vec<String>,
    pub viewport: Viewport,
}

impl Buffer {
    pub fn new(path: Option<String>) -> Self {
        let lines = match &path {
            Some(file_path) => std::fs::read_to_string(file_path)
                .unwrap()
                .lines()
                .map(|line| line.to_owned())
                .collect(),
            None => vec!["".to_string()],
        };

        Self {
            path,
            lines,
            viewport: Viewport::new(),
        }
    }
}
