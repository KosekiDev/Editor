pub struct Viewport {
    pub width: u16,
    pub height: u16,
    pub start_column: u16,
    pub start_rows: u16,

    pub cursor_x: u16,
    pub cursor_y: u16,

    pub buffer_id: usize,
}

impl Viewport {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            start_column: 0,
            start_rows: 10000,

            cursor_x: 0,
            cursor_y: 0,

            buffer_id: 0,
        }
    }

    pub fn resize(&mut self, columns: u16, rows: u16) {
        self.width = columns;
        self.height = rows;
    }
}
