/*
 * Project: rusth
 * File: 
 * Copyright 2018 Caner Korkmaz (Kausta) [info@canerkorkmaz.com]
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

mod dir;

use runner::command::{Command, Method};

use std::env::current_dir;

#[cfg(not(windows))]
pub fn get_builtin(cmd_name: &str) -> Option<Method> {
    None
}

#[cfg(windows)]
pub fn get_builtin(cmd_name: &str) -> Option<Method> {
    match cmd_name {
        "ls" => Some(ls),
        _ => None
    }
}

pub fn ls(_cmd: &Command) -> Option<i32> {
    let curr_dir = current_dir();
    match curr_dir {
        Ok(curr_dir) => {
            dir::ls_dir(&curr_dir)
        },
        Err(e) => {
            eprintln!("Error occured: {}", e);
            Some(1)
        }
    }
}


