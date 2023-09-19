use crate::block_on;
use crate::terminal::TERMINAL;
use crossterm::cursor::MoveToColumn;
use crossterm::execute;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use std::io::stdout;

pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(Logger { level })).map(|_| log::set_max_level(level))
}

struct Logger {
    level: LevelFilter,
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let message = format!("[{}] {}", record.level(), record.args());
        let input = block_on(async {
            let terminal = TERMINAL.read().await;

            format!("> {}", terminal.input)
        });

        let color = match record.level() {
            Level::Error => Color::Red,
            Level::Warn => Color::Yellow,
            Level::Info => Color::Reset,
            Level::Debug => Color::Reset,
            Level::Trace => Color::Reset,
        };

        execute!(
            stdout(),
            MoveToColumn(0),
            Clear(ClearType::CurrentLine),
            SetForegroundColor(color),
            Print(message),
            ResetColor,
            Print("\n"),
            MoveToColumn(0),
            Print(input),
        )
        .unwrap();
    }

    fn flush(&self) {}
}
