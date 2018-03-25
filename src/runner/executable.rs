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

use runner::command::{Command as Cmd, Method};

use std::ffi::OsStr;
use std::process::Command;
use std::ops::Deref;

pub fn get_run_process() -> Method {
    return run_process;
}

pub fn run_process(cmd: &Cmd) -> Option<i32> {
    return run_process_impl(cmd.command(),
                            cmd.args.iter().map(|item| item.deref()).skip(1));
}

fn run_process_impl<I, S>(process_name: &str, args: I) -> Option<i32>
    where I: IntoIterator<Item=S>,
          S: AsRef<OsStr> {
    let res = Command::new(process_name)
        .args(args)
        .spawn();
    match res {
        Ok(mut child) => {
            let res = child.wait();
            match res {
                Ok(exit_status) => {
                    println!("{0} finished", process_name);
                    return exit_status.code();
                }
                Err(e) => {
                    eprintln!("Cannot wait for {0}: {1}", process_name, e);
                    return Some(-2);
                }
            }
        }
        Err(e) => {
            eprintln!("{0} failed to start: {1}", process_name, e);
            return Some(-1);
        }
    }
}