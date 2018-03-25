/*
 * Project: rusth
 * File: main.rs
 * Copyright 2018 Caner Korkmaz (Kausta) <info@canerkorkmaz.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/

extern crate rustyline;
extern crate ansi_term;

use rustyline::completion::FilenameCompleter;
use rustyline::{Config, Editor, CompletionType, EditMode};

mod parser;
mod runner;

fn main() {
    let prompt = util::Prompt::new();

    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build();

    // Create a basic filename completer for now
    let completer = FilenameCompleter::new();
    let mut rl = Editor::with_config(config);
    rl.set_completer(Some(completer));
    let history_path = util::get_history_text_path();
    // Inform the user if there is no previous history file
    if let Err(_) = rl.load_history(&history_path) {
        println!("No previous history, creating history in {}", history_path.display());
    }
    'read_loop: loop {
        let readline = rl.readline(&prompt.get_str());
        match readline {
            Ok(line) => {
                // Parse the line into command / arguments
                let parsed = parser::parse(&line);
                match parsed {
                    Ok(parsed) => {
                        // Exit if exit is entered
                        if parsed.len() != 0 && parsed[0] == "exit" {
                            break 'read_loop;
                        }
                        // Add to history if not exit
                        rl.add_history_entry(line.as_ref());
                        // Run the parsed input
                        runner::run(parsed);
                    },
                    Err(e) => {
                        println!("Error occured in command: {}", e);
                    }
                }
            }
            Err(err) => {
                use rustyline::error::ReadlineError::*;
                match err {
                    Interrupted => println!("CTRL-C"),
                    Eof => println!("CTRL-D"),
                    _ => {
                        println!("Error occured: {:?}", err);
                        continue 'read_loop;
                    }
                }
                break 'read_loop;
            }
        }
    }
    rl.save_history(&history_path).unwrap();
}

mod util {
    use std::env::{home_dir, current_dir};
    use std::path::PathBuf;
    use std::borrow::Cow;

    #[allow(unused_imports)]
    use ansi_term;

    static PROMPT_ANSI: &'static str = "\x1b[1;32m>>\x1b[0m ";
    static PROMPT_NORMAL: &'static str = ">> ";

    pub struct Prompt<'a> {
        pub prompt_base: &'a str,
        pub have_ansi: bool,
    }

    impl<'a> Prompt<'a> {
        pub fn new() -> Prompt<'a> {
            // TODO: Config options etc.
            let have_ansi = Prompt::init_ansi();
            let prompt = match have_ansi {
                true => PROMPT_ANSI,
                false => PROMPT_NORMAL
            };
            return Prompt {
                prompt_base: prompt,
                have_ansi,
            };
        }

        /// Tries to initialize ansi support on Windows and return accordingly
        /// Always returns true on linux
        #[cfg(windows)]
        fn init_ansi() -> bool {
            if let Err(_) = ansi_term::enable_ansi_support() {
                return false;
            }
            return true;
        }
        #[cfg(not(windows))]
        fn init_ansi() -> bool {
            return true;
        }

        // TODO: To string instead of manual get_str
        pub fn get_str(&self) -> Cow<'a, str> {
            let cd = current_dir();
            let prompt = self.prompt_base;
            match cd {
                Ok(dir) => {
                    let mut dir_str = dir.into_os_string().into_string().unwrap();
                    dir_str.push(' ');
                    dir_str.push_str(prompt);
                    return dir_str.into();
                }
                Err(_) => prompt.into()
            }
        }
    }

    pub fn get_history_text_path() -> PathBuf {
        match home_dir() {
            Some(path) => path.join(".rusth.history"),
            None => PathBuf::from(".rusth.history")
        }
    }
}
