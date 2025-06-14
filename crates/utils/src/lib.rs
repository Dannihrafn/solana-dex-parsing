use bs58;
use std::collections::{HashMap, HashSet};
use types::{StructuredInstruction};
use yellowstone_grpc_proto::prelude::SubscribeUpdateTransaction;

pub fn get_account_keys(transaction: &SubscribeUpdateTransaction) -> Vec<String> {
    let txn = transaction.transaction.clone().unwrap();
    let mut account_keys = txn.transaction.unwrap().message.unwrap().account_keys;
    let meta = txn.meta.unwrap();
    let loaded_addresses_readonly = meta.loaded_readonly_addresses;
    let loaded_addresses_writable = meta.loaded_writable_addresses;
    account_keys.extend(loaded_addresses_writable);
    account_keys.extend(loaded_addresses_readonly);
    account_keys
        .into_iter()
        .map(|key| bs58::encode(key).into_string())
        .collect()
}

pub fn get_filtered_instructions(
    transaction: &SubscribeUpdateTransaction,
    account_keys: &Vec<String>,
    program_id: &str,
) -> Vec<StructuredInstruction> {
    let program_index = account_keys.iter().position(|key| *key == *program_id);
    match program_index {
        Some(_) => {}
        None => {
            panic!("Program index could not be found");
        }
    }
    let structured_instructions = structure_all_instructions(&transaction);
    let filtered_instructions =
        filter_instructions(&structured_instructions, account_keys, program_id);
    filtered_instructions
}

pub fn structure_all_instructions(
    transaction: &SubscribeUpdateTransaction,
) -> Vec<StructuredInstruction> {
    let txn = &transaction.transaction.clone().unwrap();
    let compiled_instructions = txn
        .transaction
        .clone()
        .unwrap()
        .message
        .unwrap()
        .instructions;
    let inner_instructions = txn.meta.clone().unwrap().inner_instructions;

    if inner_instructions.is_empty() {
        return compiled_instructions
            .iter()
            .map(|instruction| StructuredInstruction {
                account_key_indexes: instruction.accounts.clone(),
                program_id_index: instruction.program_id_index as u8,
                data: instruction.data.clone(),
                inner_instructions: Vec::new(),
                stack_height: 0,
            })
            .collect();
    }

    let mut formatted: Vec<StructuredInstruction> = Vec::new();

    for inner_instruction_group in inner_instructions.iter() {
        let parent_ix = &compiled_instructions[inner_instruction_group.index as usize];

        let mut parent = StructuredInstruction {
            account_key_indexes: parent_ix.accounts.clone(),
            program_id_index: parent_ix.program_id_index as u8,
            data: parent_ix.data.clone(),
            inner_instructions: Vec::new(),
            stack_height: 1,
        };

        for inner_instruction in inner_instruction_group.instructions.iter() {
            match inner_instruction.stack_height {
                Some(stack_height) => {
                    if stack_height == 2 {
                        parent.inner_instructions.push(StructuredInstruction {
                            account_key_indexes: inner_instruction.accounts.clone(),
                            program_id_index: inner_instruction.program_id_index.clone() as u8,
                            data: inner_instruction.data.clone(),
                            inner_instructions: Vec::new(),
                            stack_height: 2,
                        });
                    } else if stack_height == 3 {
                        if let Some(last) = parent.inner_instructions.last_mut() {
                            last.inner_instructions.push(StructuredInstruction {
                                account_key_indexes: inner_instruction.accounts.clone(),
                                program_id_index: inner_instruction.program_id_index.clone() as u8,
                                data: inner_instruction.data.clone(),
                                inner_instructions: Vec::new(),
                                stack_height: 3,
                            });
                        }
                    } else if stack_height == 4 {
                        if let Some(last) = parent.inner_instructions.last_mut() {
                            if let Some(last_last) = last.inner_instructions.last_mut() {
                                last_last.inner_instructions.push(StructuredInstruction {
                                    account_key_indexes: inner_instruction.accounts.clone(),
                                    program_id_index: inner_instruction.program_id_index.clone()
                                        as u8,
                                    data: inner_instruction.data.clone(),
                                    inner_instructions: Vec::new(),
                                    stack_height: 4,
                                });
                            }
                        }
                    } else if stack_height == 5 {
                        if let Some(last) = parent.inner_instructions.last_mut() {
                            if let Some(last_last) = last.inner_instructions.last_mut() {
                                if let Some(last_last_last) =
                                    last_last.inner_instructions.last_mut()
                                {
                                    last_last_last
                                        .inner_instructions
                                        .push(StructuredInstruction {
                                            account_key_indexes: inner_instruction.accounts.clone(),
                                            program_id_index: inner_instruction
                                                .program_id_index
                                                .clone()
                                                as u8,
                                            data: inner_instruction.data.clone(),
                                            inner_instructions: Vec::new(),
                                            stack_height: 5,
                                        });
                                }
                            }
                        }
                    }
                }
                None => {
                    panic!("Stack height is None");
                }
            }
        }
        formatted.push(parent);
    }
    formatted
}

pub fn filter_instructions(
    structured_instructions: &Vec<StructuredInstruction>,
    account_keys: &Vec<String>,
    program_id: &str,
) -> Vec<StructuredInstruction> {
    let program_index = account_keys.iter().position(|key| *key == *program_id);
    match program_index {
        Some(index) => {
            let mut return_ixs: Vec<StructuredInstruction> = Vec::new();
            structured_instructions.iter().for_each(|instruction| {
                if instruction.program_id_index as usize == index {
                    return_ixs.push(instruction.clone());
                }
            });
            return_ixs
        }
        None => panic!("Program index could not be found"),
    }
}

pub fn filter_instructions_new(
    roots: &[StructuredInstruction],          // &[T] is idiomatic read-only
    account_keys: &[String],
    program_ids: &HashSet<String>,
) -> HashMap<String, Vec<StructuredInstruction>> {
    let mut out: HashMap<String, Vec<StructuredInstruction>> = HashMap::new();

    // depth-first walk
    fn walk(
        ix: &StructuredInstruction,
        account_keys: &[String],
        program_ids: &HashSet<String>,
        out: &mut HashMap<String, Vec<StructuredInstruction>>,
    ) {
        // 1. resolve program-ID
        let pid = &account_keys[ix.program_id_index as usize];

        // 2. keep if caller asked for it
        if program_ids.contains(pid) {
            out.entry(pid.clone()).or_default().push(ix.clone());
        }

        // 3. recurse into children (no depth limit)
        for child in &ix.inner_instructions {
            walk(child, account_keys, program_ids, out);
        }
    }

    // kick off the walk for each root instruction
    for ix in roots {
        walk(ix, account_keys, program_ids, &mut out);
    }

    out
}
