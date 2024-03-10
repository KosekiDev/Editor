pub struct Viewport {
    pub width: u16,
    pub height: u16,
    pub start_column: u16,
    pub start_rows: u16,
}

impl Viewport {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            start_column: 0,
            start_rows: 10000,
        }
    }

    pub fn resize(&mut self, columns: u16, rows: u16) {
        self.width = columns;
        self.height = rows;
    }
}
