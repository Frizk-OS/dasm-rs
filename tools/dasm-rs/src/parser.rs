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

use crate::opcode::DalvikOpcode;

#[derive(Debug, Clone, PartialEq)]
pub struct DalvikClass {
    pub source: Option<String>,
    pub is_interface: bool,
    pub access_flags: Vec<String>,
    pub name: String,
    pub super_class: Option<String>,
    pub interfaces: Vec<String>,
    pub fields: Vec<DalvikField>,
    pub methods: Vec<DalvikMethod>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DalvikField {
    pub access_flags: Vec<String>,
    pub name: String,
    pub descriptor: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DalvikMethod {
    pub access_flags: Vec<String>,
    pub name: String,
    pub signature: String,
    pub limit_regs: Option<u32>,
    pub throws: Vec<String>,
    pub statements: Vec<DalvikStatement>,
    pub catches: Vec<DalvikCatch>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DalvikCatch {
    pub exception: String,
    pub from: String,
    pub to: String,
    pub target: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DalvikStatement {
    Label(String),
    Instruction(DalvikInstruction),
    FillArrayData {
        reg: String,
        element_type: String,
        values: Vec<String>,
    },
    PackedSwitch {
        reg: String,
        first_key: i32,
        targets: Vec<String>,
    },
    SparseSwitch {
        reg: String,
        cases: Vec<(i32, String)>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct DalvikInstruction {
    pub opcode: DalvikOpcode,
    pub raw_operands: String,
}

#[derive(Debug, Default)]
pub struct DasmParser {
    pub classes: Vec<DalvikClass>,
}

impl DasmParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse_str(&mut self, content: &str) -> Result<(), String> {
        let mut current_class: Option<DalvikClass> = None;
        let mut current_method: Option<DalvikMethod> = None;
        let mut current_source: Option<String> = None;

        let mut lines = content.lines().enumerate().peekable();

        while let Some((line_idx, raw_line)) = lines.next() {
            let line_num = line_idx + 1;
            let line = if let Some(idx) = raw_line.find(';') {
                &raw_line[..idx]
            } else {
                raw_line
            }.trim();

            if line.is_empty() {
                continue;
            }

            if line.starts_with(".source") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    current_source = Some(parts[1].to_string());
                }
            } else if line.starts_with(".class") || line.starts_with(".interface") {
                if let Some(method) = current_method.take() {
                    if let Some(cls) = &mut current_class {
                        cls.methods.push(method);
                    }
                }
                if let Some(cls) = current_class.take() {
                    self.classes.push(cls);
                }

                let is_interface = line.starts_with(".interface");
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 2 {
                    return Err(format!("Line {}: invalid class/interface declaration", line_num));
                }

                let name = parts.last().unwrap().to_string();
                let access_flags = parts[1..parts.len() - 1].iter().map(|s| s.to_string()).collect();

                current_class = Some(DalvikClass {
                    source: current_source.clone(),
                    is_interface,
                    access_flags,
                    name,
                    super_class: None,
                    interfaces: Vec::new(),
                    fields: Vec::new(),
                    methods: Vec::new(),
                });
            } else if line.starts_with(".super") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Some(cls) = &mut current_class {
                        cls.super_class = Some(parts[1].to_string());
                    }
                }
            } else if line.starts_with(".implements") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Some(cls) = &mut current_class {
                        cls.interfaces.push(parts[1].to_string());
                    }
                }
            } else if line.starts_with(".field") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let descriptor = parts.last().unwrap().to_string();
                    let name = parts[parts.len() - 2].to_string();
                    let access_flags = parts[1..parts.len() - 2].iter().map(|s| s.to_string()).collect();
                    if let Some(cls) = &mut current_class {
                        cls.fields.push(DalvikField {
                            access_flags,
                            name,
                            descriptor,
                        });
                    }
                }
            } else if line.starts_with(".method") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 2 {
                    return Err(format!("Line {}: invalid method declaration", line_num));
                }
                let full_name_sig = parts.last().unwrap();
                let (name, sig) = if let Some(sig_idx) = full_name_sig.find('(') {
                    (&full_name_sig[..sig_idx], &full_name_sig[sig_idx..])
                } else {
                    (*full_name_sig, "")
                };
                let access_flags = parts[1..parts.len() - 1].iter().map(|s| s.to_string()).collect();
                current_method = Some(DalvikMethod {
                    access_flags,
                    name: name.to_string(),
                    signature: sig.to_string(),
                    limit_regs: None,
                    throws: Vec::new(),
                    statements: Vec::new(),
                    catches: Vec::new(),
                });
            } else if line.starts_with(".limit") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 && parts[1] == "regs" {
                    if let Ok(regs) = parts[2].parse::<u32>() {
                        if let Some(method) = &mut current_method {
                            method.limit_regs = Some(regs);
                        }
                    }
                }
            } else if line.starts_with(".throws") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Some(method) = &mut current_method {
                        method.throws.push(parts[1].to_string());
                    }
                }
            } else if line.starts_with(".catch") {
                // .catch <ex> from <L1> to <L2> using <L3>
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 7 && parts[2] == "from" && parts[4] == "to" && parts[6] == "using" {
                    if let Some(method) = &mut current_method {
                        method.catches.push(DalvikCatch {
                            exception: parts[1].to_string(),
                            from: parts[3].to_string(),
                            to: parts[5].to_string(),
                            target: parts[7].to_string(),
                        });
                    }
                }
            } else if line.starts_with(".end") {
                if line.contains("method") {
                    if let Some(method) = current_method.take() {
                        if let Some(cls) = &mut current_class {
                            cls.methods.push(method);
                        }
                    }
                }
            } else if line.starts_with("fill-array-data") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let reg = parts[1].to_string();
                    let elem_type = parts[2].to_string();
                    let mut values = Vec::new();
                    while let Some((_, next_line)) = lines.peek() {
                        let trimmed = next_line.trim();
                        if trimmed == "fill-array-data-end" {
                            lines.next();
                            break;
                        }
                        if !trimmed.is_empty() {
                            values.push(trimmed.to_string());
                        }
                        lines.next();
                    }
                    if let Some(method) = &mut current_method {
                        method.statements.push(DalvikStatement::FillArrayData {
                            reg,
                            element_type: elem_type,
                            values,
                        });
                    }
                }
            } else if line.starts_with("packed-switch") {
                // packed-switch v11, 1
                let comma_idx = line.find(',').unwrap_or(line.len());
                let reg_part = &line["packed-switch".len()..comma_idx].trim();
                let key_str = if comma_idx < line.len() { line[comma_idx + 1..].trim() } else { "0" };
                let first_key = key_str.parse::<i32>().unwrap_or(0);
                let mut targets = Vec::new();
                while let Some((_, next_line)) = lines.peek() {
                    let trimmed = next_line.trim();
                    if trimmed == "packed-switch-end" {
                        lines.next();
                        break;
                    }
                    if !trimmed.is_empty() {
                        targets.push(trimmed.to_string());
                    }
                    lines.next();
                }
                if let Some(method) = &mut current_method {
                    method.statements.push(DalvikStatement::PackedSwitch {
                        reg: reg_part.to_string(),
                        first_key,
                        targets,
                    });
                }
            } else if line.starts_with("sparse-switch") {
                let reg_part = line["sparse-switch".len()..].trim();
                let mut cases = Vec::new();
                while let Some((_, next_line)) = lines.peek() {
                    let trimmed = next_line.trim();
                    if trimmed == "sparse-switch-end" {
                        lines.next();
                        break;
                    }
                    if let Some(colon_idx) = trimmed.find(':') {
                        let key_str = trimmed[..colon_idx].trim();
                        let target = trimmed[colon_idx + 1..].trim().to_string();
                        if let Ok(key) = key_str.parse::<i32>() {
                            cases.push((key, target));
                        }
                    }
                    lines.next();
                }
                if let Some(method) = &mut current_method {
                    method.statements.push(DalvikStatement::SparseSwitch {
                        reg: reg_part.to_string(),
                        cases,
                    });
                }
            } else if line.ends_with(':') && !line.contains(' ') {
                // Label definition
                let label_name = line[..line.len() - 1].to_string();
                if let Some(method) = &mut current_method {
                    method.statements.push(DalvikStatement::Label(label_name));
                }
            } else {
                // Instruction
                let mut parts = line.splitn(2, char::is_whitespace);
                let mnemonic = parts.next().unwrap();
                let raw_operands = parts.next().unwrap_or("").trim().to_string();

                if let Some(opcode) = DalvikOpcode::from_mnemonic(mnemonic) {
                    if let Some(method) = &mut current_method {
                        method.statements.push(DalvikStatement::Instruction(DalvikInstruction {
                            opcode,
                            raw_operands,
                        }));
                    }
                } else {
                    return Err(format!("Line {}: unknown opcode or directive '{}'", line_num, mnemonic));
                }
            }
        }

        if let Some(method) = current_method.take() {
            if let Some(cls) = &mut current_class {
                cls.methods.push(method);
            }
        }
        if let Some(cls) = current_class.take() {
            self.classes.push(cls);
        }

        Ok(())
    }
}
