/*
 * Project: rusth
 * File: runner/command.rs
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
use std::{
    process::Stdio,
    borrow::Cow,
    io::Write,
    fs::OpenOptions,
};

use super::{
    builtin::get_builtin,
    executable::{run_process, spawn_process},
};

pub type Method = fn(&Command) -> Option<i32>;

pub trait RunnableCmd {
    fn run(&mut self, conf: RunConfig) -> Option<i32>;
}

pub enum Runnable<'a> {
    Cmd(Command<'a>),
    Pipe(Pipe<'a>),
    Insert(Insert<'a>),
    From,
    Append(Append<'a>),
}

impl<'a> RunnableCmd for Runnable<'a> {
    fn run(&mut self, conf: RunConfig) -> Option<i32> {
        match *self {
            Runnable::Cmd(ref mut cmd) => cmd.run(conf),
            Runnable::Pipe(ref mut p) => p.run(conf),
            Runnable::Insert(ref mut i) => i.run(conf),
            Runnable::From => None,
            Runnable::Append(ref mut a) => a.run(conf)
        }
    }
}

pub struct RunConfig {
    pub input: Option<Stdio>,
    pub output: Option<Stdio>,
}

impl RunConfig {
    pub fn build() -> RunConfigBuilder {
        RunConfigBuilder::new()
    }
}

impl Default for RunConfig {
    fn default() -> RunConfig {
        RunConfig {
            input: None,
            output: None,
        }
    }
}

pub struct RunConfigBuilder {
    conf: RunConfig
}

impl RunConfigBuilder {
    pub fn new() -> RunConfigBuilder {
        RunConfigBuilder {
            conf: RunConfig::default()
        }
    }

    pub fn input(mut self, input: Stdio) -> RunConfigBuilder {
        self.conf.input = Some(input);
        self
    }

    pub fn output(mut self, output: Stdio) -> RunConfigBuilder {
        self.conf.output = Some(output);
        self
    }

    pub fn build(self) -> RunConfig {
        self.conf
    }
}

#[derive(Debug)]
pub struct Command<'a> {
    pub args: Vec<Cow<'a, str>>
}

impl<'a> Command<'a> {
    pub fn new(args: Vec<Cow<'a, str>>) -> Command<'a> {
        Command {
            args
        }
    }

    pub fn empty(&self) -> bool {
        self.args.is_empty()
    }

    pub fn command(&self) -> &str {
        self.args[0].as_ref()
    }

    pub fn is_builtin(&self) -> bool {
        get_builtin(self.command()).is_some()
    }
}

impl<'a> RunnableCmd for Command<'a> {
    fn run(&mut self, conf: RunConfig) -> Option<i32> {
        if self.empty() {
            return Some(0);
        }
        match get_builtin(self.command()) {
            Some(builtin) => builtin(self),
            None => run_process(self, conf)
        }
    }
}

#[derive(Debug)]
pub struct Pipe<'a> {
    pub from: Command<'a>,
    pub to: Command<'a>,
}

impl<'a> Pipe<'a> {
    pub fn new(from: Command<'a>, to: Command<'a>) -> Pipe<'a> {
        Pipe {
            from,
            to,
        }
    }
}

impl<'a> RunnableCmd for Pipe<'a> {
    fn run(&mut self, _conf: RunConfig) -> Option<i32> {
        if self.from.empty() {
            eprintln!("No command before pipe, ignoring it!");
            return self.to.run(RunConfig::default());
        }
        if self.from.is_builtin() || self.to.is_builtin() {
            eprintln!("Builtin redirection not supported for now!");
            return Some(12);
        }
        let to_conf = RunConfig::build().output(Stdio::piped()).build();
        let child = spawn_process(&self.from, to_conf);
        if child.is_none() {
            return Some(10);
        }
        let child = child.unwrap();
        let output = match child.wait_with_output() {
            Ok(output) => output,
            Err(e) => {
                eprintln!("Error occured in first program: {}", e);
                return Some(12);
            }
        };
        let to_conf = RunConfig::build().input(Stdio::piped()).build();
        let child = spawn_process(&self.to, to_conf);
        if child.is_none() {
            return Some(11);
        }
        let mut child = child.unwrap();
        {
            let stdin = child.stdin.as_mut().unwrap();
            let res = stdin.write_all(&output.stdout);
            if let Err(e) = res {
                eprintln!("Error occured: {}", e);
                return Some(13);
            }
        }
        match child.wait() {
            Ok(_res) => {
                Some(0)
            }
            Err(e) => {
                eprintln!("Error occured: {}", e);
                Some(14)
            }
        }
    }
}

#[derive(Debug)]
pub struct Insert<'a> {
    pub cmd: Command<'a>,
    pub file_name: Cow<'a, str>,
    append: bool,
}

impl<'a> Insert<'a> {
    pub fn new(cmd: Command<'a>, file_name: Cow<'a, str>) -> Self {
        Self {
            cmd,
            file_name,
            append: false,
        }
    }
}

// TODO: Abstract Insert, Append, From logic instead of sharing most of the code
impl<'a> RunnableCmd for Insert<'a> {
    fn run(&mut self, _conf: RunConfig) -> Option<i32> {
        if self.cmd.empty() {
            eprintln!("No command before pipe, ignoring it!");
            return None;
        }
        if self.cmd.is_builtin() {
            eprintln!("Builtin redirection not supported for now!");
            return Some(12);
        }
        let cmd_conf = RunConfig::build().output(Stdio::piped()).build();
        let child = spawn_process(&self.cmd, cmd_conf);
        if child.is_none() {
            return Some(10);
        }
        let child = child.unwrap();
        let output = match child.wait_with_output() {
            Ok(output) => output,
            Err(e) => {
                eprintln!("Error occured in command program: {}", e);
                return Some(12);
            }
        };
        let mut options = OpenOptions::new();
        if self.append {
            options.append(true).create(true);
        } else {
            options.write(true).create(true).truncate(true);
        }
        let mut file = match options.open(self.file_name.as_ref()) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Cannot create/open the file: {}", e);
                return Some(13);
            }
        };
        match file.write_all(&output.stdout) {
            Ok(()) => Some(0),
            Err(e) => {
                eprintln!("Cannot write the text: {}", e);
                Some(14)
            }
        }
    }
}


#[derive(Debug)]
pub struct Append<'a> {
    insert: Insert<'a>
}

impl<'a> Append<'a> {
    pub fn new(cmd: Command<'a>, file_name: Cow<'a, str>) -> Self {
        Self {
            insert: Insert {
                cmd,
                file_name,
                append: true,
            }
        }
    }
}

impl<'a> RunnableCmd for Append<'a> {
    fn run(&mut self, conf: RunConfig) -> Option<i32> {
        self.insert.run(conf)
    }
}
