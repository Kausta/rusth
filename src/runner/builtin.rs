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
#[cfg(windows)]
use super::windows;
use super::command::{Command, Method};

use std::env::{set_current_dir, current_dir, home_dir};

#[cfg(not(windows))]
fn get_builtin_os(_cmd_name: &str) -> Option<Method> {
    None
}
#[cfg(windows)]
fn get_builtin_os(cmd_name: &str) -> Option<Method> {
    windows::get_builtin(cmd_name)
}

pub fn get_builtin(cmd_name: &str) -> Option<Method> {
    if let Some(builtin) = get_builtin_os(cmd_name) {
        return Some(builtin);
    }
    match cmd_name {
        "echo" => Some(echo),
        "cd" => Some(cd),
        "pwd" => Some(pwd),
        _ => None
    }
}

pub fn echo(cmd: &Command) -> Option<i32> {
    for s in cmd.args.iter().skip(1) {
        print!("{} ", s);
    }
    println!();
    Some(0)
}

pub fn cd(cmd: &Command) -> Option<i32> {
    let c = cmd.args.len();
    if c < 2 {
        cd_impl("~")
    } else {
        cd_impl(cmd.args[1].as_ref())
    }
}

fn cd_impl(dir: &str) -> Option<i32> {
    use std::path::PathBuf;

    let dir = if dir.starts_with('~') {
        let dir = dir.replacen("~", "", 1);

        let home_dir = home_dir();
        if home_dir.is_none() {
            eprintln!("Cannot get home directory!");
            return Some(5);
        }
        let home_dir = home_dir.unwrap();
        home_dir.join(dir)
    } else {
        PathBuf::from(dir)
    };
    let res = set_current_dir(dir);
    if let Err(e) = res {
        eprintln!("Cannot change directory: {0}", e);
        return Some(2);
    }
    Some(0)
}

pub fn pwd(_cmd: &Command) -> Option<i32> {
    let cd = current_dir();
    match cd {
        Ok(dir) => {
            println!("{}", dir.display());
            Some(0)
        },
        Err(e) => {
            eprintln!("Cannot obtain active directory: {}", e);
            Some(1)
        }
    }
}