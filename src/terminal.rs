use std::io::{stdout};
use std::ops::AddAssign;
use std::sync::Mutex;
use crossterm::cursor::{MoveLeft, MoveToColumn};
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind};
use crossterm::execute;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType, enable_raw_mode};
use futures::{future::FutureExt, select, StreamExt};
use once_cell::sync::Lazy;
use tokio::io;
use tokio::sync::RwLock;
use crate::command_dispatcher::CommandDispatcher;
use crate::log::Level;

pub static TIMES: Mutex<i32> = Mutex::new(0);

pub static TERMINAL: Lazy<RwLock<Terminal>> = Lazy::new(|| {
    let terminal = RwLock::new(Terminal::new(Level::Info));

    let mut lock = TIMES.lock().unwrap();
    lock.add_assign(1);

    // Start terminal event listener
    tokio::spawn(Terminal::event_listener());
    // Prepare terminal text
    execute!(
        stdout(),
        Print("> ")
    ).unwrap();

    terminal
});

pub struct Terminal {
    input: String,
    pub level: Level,
    pub dispatcher: CommandDispatcher,
}

impl Terminal {
    fn new(level: Level) -> Self {
        enable_raw_mode().unwrap();

        Self {
            input: String::new(),
            level,
            dispatcher: CommandDispatcher::new(),
        }
    }

    pub fn log(&self, level: Level, message: &str) {
        if self.level > level {
            return;
        }

        let message = format!(
            "[{}] {}",
            level, message
        );

        let input = format!(
            "\n> {}",
            self.input
        );

        execute!(
            stdout(),
            MoveToColumn(0),
            Clear(ClearType::CurrentLine),

            SetForegroundColor(level.get_color()),
            Print(message),
            ResetColor,

            Print(input),
        ).unwrap();

    }

    async fn handle_event(event: KeyEvent) -> io::Result<()> {
        if event.kind != KeyEventKind::Press {
            return Ok(());
        }

        let mut terminal = TERMINAL.write().await;
        match event.code {
            KeyCode::Backspace => {
                if terminal.input.is_empty() {
                    return Ok(());
                }

                execute!(
                    stdout(),
                    MoveLeft(1),
                    Clear(ClearType::UntilNewLine),
                )?;
                terminal.input.pop();
            },
            KeyCode::Char(char) => {
                execute!(
                    stdout(),
                    Print(char),
                )?;
                terminal.input.push(char);
            },
            KeyCode::Enter => {
                execute!(
                    stdout(),
                    Print("\n> "),
                )?;

                let (name, args) = terminal.prepare_command().await;
                let command = terminal.dispatcher.get_command(&name).cloned();
                if command.is_none() {
                    terminal.log(Level::Warning, "Command not found!");
                    return Ok(());
                }
                let command = command.unwrap();

                drop(terminal);

                let success = command.execute(args);

                let terminal = TERMINAL.write().await;

                let usage = command.get_usage();
                if !success && usage.is_some() {
                    terminal.log(Level::Warning,  usage.unwrap().as_str());
                }

            },
            _ => {},
        };
        Ok(())
    }

    async fn prepare_command(&mut self) -> (String, Vec<String>) {
        let input = self.input.clone();
        self.input.clear();

        let mut args: Vec<String> = input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let name: String = args.remove(0);

        (name, args)
    }

    async fn event_listener() -> io::Result<()> {
        let mut reader = EventStream::new();

        loop {
            let mut event = reader.next().fuse();

            select! {
                maybe_event = event => {
                    match maybe_event {
                        Some(Ok(event)) => {
                            if let Event::Key(event) = event {
                                Terminal::handle_event(event).await?;
                            }
                        }
                        Some(Err(e)) => println!("Error: {:?}\n", e),
                        None => break,
                    }
                }
            }
        }
        Ok(())
    }
}

