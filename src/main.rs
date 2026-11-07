//! A macOS input source CLI using safe cidre APIs.
//!
//! Use `--english` to select ABC or U.S. and `--status` to print the input source ID.
//! Both commands use Text Input Sources APIs and need no Accessibility permission.
//! Run in a logged-in macOS GUI session without sudo. This tool does not read or
//! change an input method's internal mode, such as Squirrel's ascii_mode.

use std::process::ExitCode;

const HELP: &str = "im-switch — macOS input source CLI

Usage: im-switch --english
       im-switch --status
       im-switch --help

--english  Select an enabled ABC or U.S. input source and confirm it.
--status   Print the current system input source ID.

No Accessibility permission is required.
With no arguments, show this help without changing the input source.
System input sources do not reflect an input method's internal language mode.";

enum Command {
    English,
    Status,
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = match args.as_slice() {
        [] => {
            println!("{HELP}");
            return ExitCode::SUCCESS;
        }
        [arg] => match arg.as_str() {
            "--english" => Command::English,
            "--status" => Command::Status,
            "--help" | "-h" => {
                println!("{HELP}");
                return ExitCode::SUCCESS;
            }
            _ => {
                eprintln!("Unknown argument: {arg}\n\n{HELP}");
                return ExitCode::from(2);
            }
        },
        _ => {
            eprintln!("Too many arguments.\n\n{HELP}");
            return ExitCode::from(2);
        }
    };

    match platform::run(command) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("im-switch: {error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(not(target_os = "macos"))]
mod platform {
    pub fn run(_: super::Command) -> Result<(), &'static str> {
        Err("This program only supports macOS.")
    }
}

#[cfg(target_os = "macos")]
mod platform {
    use super::Command;
    use cidre::tis;
    use std::{thread, time::Duration};

    pub fn run(command: Command) -> Result<(), &'static str> {
        match command {
            Command::English => select_english(),
            Command::Status => {
                println!("{}", current_source_id()?);
                Ok(())
            }
        }
    }

    fn current_source_id() -> Result<String, &'static str> {
        let source =
            tis::InputSrc::current_kb().ok_or("Failed to read the current input source.")?;
        source
            .id()
            .map(|id| id.to_string())
            .ok_or("Failed to read the current input source ID.")
    }

    fn select_english() -> Result<(), &'static str> {
        let sources =
            tis::InputSrc::list(None, false).ok_or("Failed to list enabled input sources.")?;
        for id in ["com.apple.keylayout.ABC", "com.apple.keylayout.US"] {
            let source = sources
                .iter()
                .find(|source| source.id().is_some_and(|value| value.to_string() == id));
            let Some(source) = source else {
                continue;
            };
            if current_source_id()? == id {
                return Ok(());
            }
            // Selection is propagated through the GUI session. Confirm it
            // before returning so callers can safely start their TUI with &&.
            source
                .select()
                .map_err(|_| "Failed to select the English input source.")?;
            for _ in 0..20 {
                if current_source_id()? == id {
                    return Ok(());
                }
                thread::sleep(Duration::from_millis(10));
            }
            return Err("Could not confirm that the English input source is active.");
        }
        Err("No enabled ABC or U.S. input source found. Add one in System Settings > Keyboard.")
    }
}
