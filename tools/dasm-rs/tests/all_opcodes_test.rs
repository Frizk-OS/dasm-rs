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

use dasm_rs::DasmParser;
use dasm_rs::DalvikStatement;
use std::fs;
use std::path::Path;

#[test]
fn test_parse_all_opcodes_d() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let test_file = Path::new(manifest_dir)
        .parent()
        .unwrap()
        .join("dasm")
        .join("test")
        .join("all_opcodes.d");

    assert!(test_file.exists(), "all_opcodes.d must exist at {:?}", test_file);

    let content = fs::read_to_string(&test_file).expect("Failed to read all_opcodes.d");
    let mut parser = DasmParser::new();
    let result = parser.parse_str(&content);
    assert!(result.is_ok(), "Failed to parse all_opcodes.d: {:?}", result.err());

    // Expect 2 classes/interfaces: dasm.test.all_opcodes and dasm.test.interface.test_interface
    assert_eq!(parser.classes.len(), 2, "Expected 2 classes parsed");

    let class1 = &parser.classes[0];
    assert_eq!(class1.name, "dasm.test.all_opcodes");
    assert_eq!(class1.super_class.as_deref(), Some("java/lang/Object"));
    assert_eq!(class1.interfaces, vec!["java/lang/Runnable"]);
    assert_eq!(class1.fields.len(), 2);
    assert_eq!(class1.methods.len(), 2);

    // Method 0: <init>()V
    let m0 = &class1.methods[0];
    assert_eq!(m0.name, "<init>");
    assert_eq!(m0.signature, "()V");

    // Method 1: run()I
    let m1 = &class1.methods[1];
    assert_eq!(m1.name, "run");
    assert_eq!(m1.signature, "()I");
    assert_eq!(m1.limit_regs, Some(4));
    assert_eq!(m1.throws, vec!["java/lang/NullPointerException"]);
    assert_eq!(m1.catches.len(), 1);
    assert_eq!(m1.catches[0].exception, "java/lang/Exception");
    assert_eq!(m1.catches[0].from, "Label1");
    assert_eq!(m1.catches[0].to, "Label2");
    assert_eq!(m1.catches[0].target, "Label3");

    // Check count of statements in run()
    let instructions: Vec<_> = m1.statements.iter().filter_map(|s| {
        if let DalvikStatement::Instruction(insn) = s {
            Some(insn)
        } else {
            None
        }
    }).collect();

    // Verify there are over 100 instructions tested
    assert!(instructions.len() > 100, "Expected > 100 Dalvik instructions in all_opcodes.d, got {}", instructions.len());

    // Class 2: interface
    let class2 = &parser.classes[1];
    assert!(class2.is_interface);
    assert_eq!(class2.name, "dasm.test.interface.test_interface");
    assert_eq!(class2.methods.len(), 1);
    assert_eq!(class2.methods[0].name, "test");
    assert_eq!(class2.methods[0].signature, "()V");
}
