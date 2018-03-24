/*
 * Project: rusth
 * File: runner/executable.rs
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

use runner::command::Command as Cmd;

use std::ffi::OsStr;
use std::process::{Command, Output};

pub fn run_process(cmd: &Cmd) {
    run_process_impl(cmd.command(), cmd.skip(1));
}

fn run_process_impl<I, S>(process_name: &str, args: I) -> Option<Output>
    where I: IntoIterator<Item=S>,
          S: AsRef<OsStr> {
    let res = Command::new(process_name)
        .args(args)
        .spawn();
    match res {
        Ok(child) => {
            let res = child.wait_with_output();
            match res {
                Ok(output) => {
                    println!("{0} finished", process_name);
                    return Some(output);
                },
                Err(e) => {
                    eprintln!("Cannot wait for {0}: {1}", process_name, e);
                    return None;
                }
            }
        },
        Err(e) => {
            eprintln!("{0} failed to start: {1}", process_name, e);
            return None;
        }
    }
}