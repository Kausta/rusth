/*
 * Project: rusth
 * File: runner/windows/dir.rs
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

use std;
use std::fs::{self, DirEntry};
use std::path::Path;

struct LsVisitor {
    id: usize,
}

impl LsVisitor {
    pub fn new() -> LsVisitor {
        LsVisitor {
            id: 0
        }
    }

    pub fn visit(&mut self, entry: &DirEntry) {
        let path = entry.path();
        let is_dir = path.is_dir();
        let file_name = entry.file_name();
        self.visit_from_str(is_dir, &file_name.to_string_lossy());
    }

    pub fn visit_from_str<T: std::fmt::Display>(&mut self, is_dir: bool, file_name: &T){
        println!("{0} :\t{1}", if is_dir { "Dir " } else { "File" }, file_name);
        self.id += 1;
    }

    pub fn visit_dir_from_str<T: std::fmt::Display>(&mut self, file_name: &T) {
        self.visit_from_str(true, file_name);
    }
}

pub fn ls_dir(dir: &Path) -> Option<i32> {
    if dir.is_dir() {
        println!("\t{} >", dir.display());
        let mut visitor = LsVisitor::new();
        visitor.visit_dir_from_str(&".");
        visitor.visit_dir_from_str(&"..");
        let read_dir = fs::read_dir(dir);
        match read_dir {
            Err(e) => {
                eprintln!("Error occured: {}", e);
                return Some(2);
            },
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            visitor.visit(&entry);
                        }
                        Err(e) => {
                            eprintln!("Error occured: {}", e);
                            return Some(3);
                        }
                    }
                }
            }
        }
        Some(0)
    } else {
        eprintln!("Given path is not a directory");
        Some(6)
    }
}