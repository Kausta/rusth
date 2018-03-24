/*
 * Project: rusth
 * File: runner/builtin.rs
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

use runner::command::Command;

use std::env::{set_current_dir, current_dir};

pub fn echo(cmd: &Command) {
    for s in cmd.skip(1) {
        print!("{} ", *s);
    }
    println!();
}

pub fn cd(cmd: &Command) {
    let c = cmd.args.len();
    if c < 2 {
        eprintln!("Not enough arguments to cd!");
        return;
    }
    cd_impl(cmd.args[1]);
}

fn cd_impl(dir: &str) {
    let res = set_current_dir(dir);
    if let Err(e) = res {
        eprintln!("Cannot change directory: {0}", e);
    }
}

pub fn pwd(_cmd: &Command){
    let cd = current_dir();
    match cd {
        Ok(dir) => println!("{}", dir.display()),
        Err(e) => eprintln!("Cannot obtain active directory: {}", e)
    }
}