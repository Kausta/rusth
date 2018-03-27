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

use super::command::{Command as Cmd, RunConfig};

use std::io;
use std::ffi::OsStr;
use std::process::{Command, Child};
use std::ops::Deref;

pub fn run_process(cmd: &Cmd, run_config: RunConfig) -> Option<i32> {
    match spawn_process(cmd, run_config) {
        Some(mut child) => {
            let res = child.wait();
            match res {
                Ok(exit_status) => {
                    println!("{0} finished", cmd.command());
                    exit_status.code()
                }
                Err(e) => {
                    eprintln!("Cannot wait for {0}: {1}", cmd.command(), e);
                    Some(-2)
                }
            }
        }
        None => Some(-1)
    }
}

pub fn spawn_process(cmd: &Cmd, run_config: RunConfig) -> Option<Child> {
    let res = spawn_process_impl(cmd.command(),
                                 cmd.args.iter().map(|item| item.deref()).skip(1),
                                 run_config);
    match res {
        Ok(child) => {
            Some(child)
        },
        Err(e) => {
            eprintln!("{0} failed to start: {1}", cmd.command(), e);
            None
        }
    }
}


fn spawn_process_impl<I, S>(process_name: &str, args: I, conf: RunConfig) -> io::Result<Child>
    where I: IntoIterator<Item=S>,
          S: AsRef<OsStr> {
    let mut cmd = Command::new(process_name);
    cmd.args(args);
    if let Some(stdin) = conf.input {
        cmd.stdin(stdin);
    }
    if let Some(stdout) = conf.output {
        cmd.stdout(stdout);
    }
    cmd.spawn()
}