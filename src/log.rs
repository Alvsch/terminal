use std::io::stdout;
use crossterm::cursor::MoveToColumn;
use crossterm::execute;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use crate::block_on;
use crate::terminal::TERMINAL;

static LOGGER: Logger = Logger;

pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|_| log::set_max_level(level))
}


struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let message = format!(
            "[{}] {}",
            record.level(), record.args()
        );
        let input = block_on(async {
            let terminal = TERMINAL.read().await;

            format!(
                "\n> {}",
                terminal.input
            )
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

            Print(input),
        ).unwrap();
    }

    fn flush(&self) {}
}