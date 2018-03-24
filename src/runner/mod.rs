/*
 * Project: rusth
 * File: runner/mod.rs
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

mod command;
use self::command::*;

mod executable;
mod builtin;

pub fn run_command(c: &Command) {
    if c.empty() {
        return;
    }
    match c.command() {
        "echo" => { builtin::echo(c); }
        "cd" => { builtin::cd(c); }
        _ => { executable::run_process(c); }
    };
}

pub fn run(args: Vec<&str>) {
    let command = Command::new(args);
    run_command(&command);
}