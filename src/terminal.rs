use std::io::{BufWriter, Stdout, Write};

pub struct TerminalBuilder<T: Write> {
    writer: T,
    title: String,
    raw_mode: bool,
    alternative_mode: bool,
    line_wrap: bool,
    hide_cursor: bool,
    size: (u16,u16)
}

pub struct Terminal<T: Write> {
    writer: T,
}

impl<T: Write> Terminal<T> {
}

impl<T: Write> TerminalBuilder<T> {
    pub fn build(self) -> Terminal<T> {
        Terminal { writer: self.writer }
    }

    pub fn writer<N: Write>(self, writer: N) -> TerminalBuilder<N> {
        TerminalBuilder {
            writer,
            title: self.title,
            raw_mode: self.raw_mode,
            line_wrap: self.line_wrap,
            hide_cursor: self.hide_cursor,
            alternative_mode: self.alternative_mode,
            size: self.size
        }
    }

    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = title.into();
        self
    }

    pub fn raw_mode(mut self) -> Self {
        self.raw_mode = true;
        self
    }

    pub fn alternative_mode(mut self) -> Self {
        self.alternative_mode = true;
        self
    }

    pub fn line_wrap(mut self) -> Self {
        self.line_wrap = true;
        self
    }

    pub fn hide_cursor(mut self) -> Self {
        self.hide_cursor = true;
        self
    }

    pub fn set_size(mut self, columns: u16, rows: u16) -> Self {
        self.size = (columns, rows);
        self
    }
}

impl Default for TerminalBuilder<BufWriter<Stdout>> {
    fn default() -> Self {
        TerminalBuilder {
            writer: BufWriter::new(std::io::stdout()),
            title: String::new(),
            raw_mode: false,
            alternative_mode: false,
            line_wrap: false,
            hide_cursor: false,
            size: (0, 0)
        }
    }
}