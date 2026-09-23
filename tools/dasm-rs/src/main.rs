/*
 * Copyright (C) 2026 The AOSP and FrizkOS.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::env;
use std::fs;
use std::process;
use dasm_rs::DasmParser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("dasm-rs: Modern Rust Dalvik assembler & opcode validator");
        eprintln!("Usage: dasm-rs <file.d>");
        process::exit(1);
    }

    let path = &args[1];
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading {}: {}", path, e);
            process::exit(1);
        }
    };

    let mut parser = DasmParser::new();
    match parser.parse_str(&content) {
        Ok(()) => {
            println!("Successfully parsed {} classes from {}", parser.classes.len(), path);
            for cls in &parser.classes {
                let kind = if cls.is_interface { "interface" } else { "class" };
                println!("  {} {} ({} methods, {} fields)", kind, cls.name, cls.methods.len(), cls.fields.len());
                for m in &cls.methods {
                    println!("    method {}{} ({} statements)", m.name, m.signature, m.statements.len());
                }
            }
        }
        Err(e) => {
            eprintln!("Parsing failed: {}", e);
            process::exit(2);
        }
    }
}
