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

use rustyline::completion::FilenameCompleter;
use rustyline::{Config, Editor, CompletionType, EditMode};

mod parser;
mod runner;

fn main() {
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build();

    let completer = FilenameCompleter::new();
    let mut rl = Editor::with_config(config);
    rl.set_completer(Some(completer));
    let history_path = util::get_history_text_path();
    if let Err(_) = rl.load_history(&history_path) {
        println!("No previous history, creating history in {}", history_path.display());
    }
    'read_loop: loop {
        let readline = rl.readline(&util::get_prompt());
        match readline {
            Ok(line_raw) => {
                if line_raw.trim() == "exit" {
                    break 'read_loop;
                }
                let line = line_raw.trim().to_owned();
                rl.add_history_entry(line.as_ref());
                let parsed = parser::parse(&line);
                runner::run(parsed);
            }
            Err(err) => {
                use rustyline::error::ReadlineError::*;
                match err {
                    Interrupted => println!("CTRL-C"),
                    Eof => println!("CTRL-D"),
                    _ => println!("Error occured: {:?}", err)
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

    #[cfg(unix)]
    static PROMPT: &'static str = "\x1b[1;32m>>\x1b[0m ";

    #[cfg(windows)]
    static PROMPT: &'static str = ">> ";

    pub fn get_history_text_path() -> PathBuf {
        match home_dir() {
            Some(path) => path.join(".rusth.history"),
            None => PathBuf::from(".rusth.history")
        }
    }

    pub fn get_prompt<'a>() -> Cow<'a, str> {
        let cd = current_dir();
        match cd {
            Ok(dir) => {
                let mut dir_str = dir.into_os_string().into_string().unwrap();
                dir_str.push(' ');
                dir_str.push_str(PROMPT);
                return dir_str.into();
            }
            Err(_) => PROMPT.into()
        }
    }
}
