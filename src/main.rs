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
mod util;

use runner::command::Runnable;
use util::{prompt, history};

fn main() {
    let mut prompt = prompt::Prompt::new();

    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build();

    // Create a basic filename completer for now
    let completer = FilenameCompleter::new();
    let mut rl = Editor::with_config(config);
    rl.set_completer(Some(completer));
    let history_path = history::get_history_text_path();
    // Inform the user if there is no previous history file
    if rl.load_history(&history_path).is_err() {
        println!("No previous history, creating history in {}", history_path.display());
    }
    'read_loop: loop {
        let line = match rl.readline(&prompt.make_prompt()) {
            Ok(line) => line,
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
        };

        // Parse the line into command / arguments
        let mut parsed = match parser::parse(&line) {
            Ok(mut parsed) => parsed,
            Err(e) => {
                println!("Error occured in command: {}", e);
                continue 'read_loop;
            }
        };

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
    rl.save_history(&history_path).unwrap();
}

