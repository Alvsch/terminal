use crate::block_on;
use crate::command::Command;
use crate::command_dispatcher::CommandDispatcher;
use crossterm::cursor::{MoveLeft, MoveRight, MoveToColumn, position};
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use futures::{future::FutureExt, select, StreamExt};
use log::{info, warn};
use once_cell::sync::Lazy;
use std::io::stdout;
use std::process::exit;
use tokio::io;
use tokio::sync::RwLock;

pub(crate) static INPUT: Lazy<RwLock<String>> = Lazy::new(|| {
    RwLock::new(String::new())
});

pub static TERMINAL: Lazy<RwLock<Terminal>> = Lazy::new(|| {
    let terminal = RwLock::new(Terminal::new());

    block_on(async {
        let mut lock = terminal.write().await;

        lock.dispatcher.add_command(Command::new(
            "help".into(),
            Some("Command for help".into()),
            Some("How do you fail to use the help command".into()),
            |_| {
                let lock = block_on(async { TERMINAL.read().await });

                let mut message = String::from("Help message:");
                for command in lock.dispatcher.get_command_names() {
                    let name = command.get_name();
                    let description = command.get_description().unwrap_or("".into());

                    message.push_str(format!("\n       {name} - {description}").as_str());
                }

                info!("{}", message.as_str());

                true
            },
        ));
    });

    // Start terminal event listener
    tokio::spawn(Terminal::event_listener());
    // Prepare terminal text
    execute!(
        stdout(),
        Print("> "),
    ).unwrap();

    terminal
});

pub struct Terminal {
    pub dispatcher: CommandDispatcher,
}

impl Terminal {
    fn new() -> Self {
        enable_raw_mode().unwrap();

        Self {
            dispatcher: CommandDispatcher::new(),
        }
    }

    async fn handle_event(event: KeyEvent) -> io::Result<()> {
        if event.kind != KeyEventKind::Press {
            return Ok(());
        }

        if event.modifiers == KeyModifiers::CONTROL && event.code == KeyCode::Char('c') {
            disable_raw_mode().unwrap();
            exit(-1073741510);
        }

        let mut input = INPUT.write().await;
        match event.code {
            KeyCode::Backspace => {
                if input.is_empty() {
                    return Ok(());
                }

                execute!(stdout(), MoveLeft(1), Clear(ClearType::UntilNewLine),)?;
                input.pop();
            }
            KeyCode::Char(char) => {
                execute!(stdout(), Print(char),)?;
                input.push(char);
            }
            KeyCode::Enter => {
                execute!(stdout(), Print("\n"), MoveToColumn(0), Print("> "),)?;

                if input.is_empty() {
                    return Ok(());
                }
                drop(input);

                let terminal = TERMINAL.read().await;
                let (command, args) = terminal.prepare_command().await;

                if command.is_none() {
                    warn!("Command not found!");
                    return Ok(());
                }
                let command = command.unwrap();

                let success = command.execute(args);

                let usage = command.get_usage();
                if !success && usage.is_some() {
                    warn!("{}", usage.unwrap().as_str());
                }
            }
            _ => {}
        };
        Ok(())
    }

    async fn prepare_command(&self) -> (Option<Command>, Vec<String>) {
        let mut lock = INPUT.write().await;

        let cloned_input = lock.clone();
        lock.clear();

        let mut args: Vec<String> = cloned_input.split_whitespace().map(|s| s.to_string()).collect();

        let name: String = args.remove(0);

        let command = self.dispatcher.get_command(&name).cloned();

        (command, args)
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
