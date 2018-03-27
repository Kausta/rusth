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

#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

extern crate rustyline;
extern crate ansi_term;

use rustyline::completion::FilenameCompleter;
use rustyline::{Config, Editor, CompletionType, EditMode};

mod parser;
mod runner;

use runner::command::Runnable;

fn main() {
    let mut prompt = util::Prompt::new();

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
    if rl.load_history(&history_path).is_err() {
        println!("No previous history, creating history in {}", history_path.display());
    }
    'read_loop: loop {
        let readline = rl.readline(&prompt.make_prompt());
        match readline {
            Ok(line) => {
                // Parse the line into command / arguments
                let parsed = parser::parse(&line);
                match parsed {
                    Ok(mut parsed) => {
                        // Exit if exit is entered
                        if let Runnable::Cmd(ref mut cmd) = parsed {
                            if !cmd.empty() && cmd.command() == "exit" {
                                break 'read_loop;
                            }
                        }
                        // Add to history if not exit
                        rl.add_history_entry(line.as_ref());
                        // Run the parsed input
                        let code = runner::run_command(&mut parsed);
                        prompt.set_return_code(code);
                    }
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
    use std::fmt::Write;

    #[allow(unused_imports)]
    use ansi_term;

    static PROMPT_ANSI: &'static str = "\x1b[1;32m>>\x1b[0m ";
    static PROMPT_NORMAL: &'static str = ">> ";

    pub struct Prompt<'a> {
        pub prompt_base: &'a str,
        pub have_ansi: bool,
        return_code: Option<i32>,
    }

    impl<'a> Prompt<'a> {
        pub fn new() -> Prompt<'a> {
            // TODO: Config options etc.
            let have_ansi = Prompt::init_ansi();
            let prompt = if have_ansi { PROMPT_ANSI } else { PROMPT_NORMAL };
            Prompt {
                prompt_base: prompt,
                have_ansi,
                return_code: None,
            }
        }

        /// Tries to initialize ansi support on Windows and return accordingly
        /// Always returns true on linux
        #[cfg(windows)]
        fn init_ansi() -> bool {
            ansi_term::enable_ansi_support().is_ok()
        }
        #[cfg(not(windows))]
        fn init_ansi() -> bool {
            true
        }

        pub fn set_return_code(&mut self, code: Option<i32>) {
            self.return_code = code;
        }

        pub fn make_prompt(&mut self) -> String {
            let cd = current_dir();
            let mut prompt = String::new();
            if let Some(code) = self.return_code {
                write!(&mut prompt, "({}) ", code).unwrap(); // TODO: Not unwrap here
            }
            if let Ok(dir) = cd {
                write!(&mut prompt, "{} ", dir.into_os_string().into_string().unwrap()).unwrap(); // TODO: Not unwrap here
            };
            prompt.push_str(self.prompt_base);
            self.reset_state();
            prompt
        }

        fn reset_state(&mut self) {
            self.return_code = None;
        }
    }

    pub fn get_history_text_path() -> PathBuf {
        match home_dir() {
            Some(path) => path.join(".rusth.history"),
            None => PathBuf::from(".rusth.history")
        }
    }
}
