use bs58;
use std::collections::HashSet;
use types::{
    InnerInstruction, MyCompiledInstruction, MyHeader, MyInnerInstruction, MyInnerInstructions,
    MyMessage, MyMeta, MyTokenBalance, MyTransaction, MyTransactionInner, MyUiTokenAmount,
    StructuredInstruction,
};

pub fn get_account_keys(transaction: &MyTransaction) -> Vec<String> {
    let static_account_keys = &transaction.transaction.message.account_keys;
    let loaded_addresses_readonly = &transaction.meta.loaded_readonly_addresses;
    let loaded_addresses_writable = &transaction.meta.loaded_writable_addresses;
    let mut account_keys = static_account_keys.clone();
    account_keys.extend(loaded_addresses_writable.clone());
    account_keys.extend(loaded_addresses_readonly.clone());
    account_keys
        .into_iter()
        .map(|key| bs58::encode(key).into_string())
        .collect()
}

pub fn get_filtered_instructions(
    account_keys: &Vec<String>,
    transaction: &MyTransaction,
    program_id: &str,
) -> Vec<StructuredInstruction> {
    let program_index = account_keys.iter().position(|key| *key == *program_id);
    match program_index {
        Some(_) => {}
        None => {
            panic!("Program index could not be found");
        }
    }
    let structured_instructions = structure_all_instructions(transaction);
    let filtered_instructions =
        filter_instructions(&structured_instructions, account_keys, program_id);
    filtered_instructions
}

pub fn structure_all_instructions(transaction: &MyTransaction) -> Vec<StructuredInstruction> {
    let max_depth = 3;
    let compiled_instructions = &transaction.transaction.message.instructions;
    let inner_instructions = &transaction.meta.inner_instructions;

    if inner_instructions.is_empty() {
        return compiled_instructions
            .iter()
            .map(|instruction| {
                return StructuredInstruction {
                    account_key_indexes: instruction.accounts.clone(),
                    program_id_index: instruction.program_id_index.clone() as u8,
                    data: instruction.data.clone(),
                    inner_instructions: Vec::new(),
                };
            })
            .collect();
    }

    let mut formatted: Vec<StructuredInstruction> = Vec::new();

    inner_instructions.iter().for_each(|instruction| {
        let parent_ix = &compiled_instructions[instruction.index as usize];
        let mut parent: StructuredInstruction = StructuredInstruction {
            account_key_indexes: parent_ix.accounts.clone(),
            program_id_index: parent_ix.program_id_index.clone() as u8,
            data: parent_ix.data.clone(),
            inner_instructions: Vec::new(),
        };
        let mut tree: Vec<InnerInstruction> = Vec::new();
        let mut stack: Vec<StructuredInstruction> = Vec::new();
        let mut promoted: HashSet<StructuredInstruction> = HashSet::new();

        instruction
            .instructions
            .iter()
            .for_each(|inner_instruction| {
                let depth: u8 = inner_instruction
                    .stack_height
                    .unwrap_or(0)
                    .try_into()
                    .unwrap();
                let mut new_ix: StructuredInstruction = StructuredInstruction {
                    account_key_indexes: inner_instruction.accounts.clone(),
                    program_id_index: inner_instruction.program_id_index.clone() as u8,
                    data: inner_instruction.data.clone(),
                    inner_instructions: Vec::new(),
                };

                if depth == 0 {
                    tree.push(InnerInstruction {
                        accounts: new_ix.account_key_indexes.clone(),
                        data: new_ix.data.clone(),
                        program_id_index: new_ix.program_id_index.clone(),
                        stack_height: depth,
                        inner_instructions: Vec::new(),
                    });
                } else {
                    if depth > max_depth {
                        let prev = &stack[max_depth as usize];
                        if !promoted.contains(&prev) {
                            tree.push(InnerInstruction {
                                accounts: prev.account_key_indexes.clone(),
                                data: prev.data.clone(),
                                program_id_index: prev.program_id_index.clone(),
                                stack_height: depth,
                                inner_instructions: Vec::new(),
                            });
                            promoted.insert(prev.clone());
                        }
                    }

                    let parent_depth = if depth > max_depth {
                        max_depth
                    } else {
                        depth - 1
                    };
                    let mut p = stack[parent_depth as usize].clone();
                    p.inner_instructions.push(InnerInstruction {
                        accounts: new_ix.account_key_indexes.clone(),
                        data: new_ix.data.clone(),
                        program_id_index: new_ix.program_id_index.clone(),
                        stack_height: depth,
                        inner_instructions: Vec::new(),
                    });
                }
                stack[depth as usize] = new_ix.clone();
            });
        parent.inner_instructions = tree.clone();
        formatted.push(parent.clone());
    });
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
                } else {
                    instruction
                        .inner_instructions
                        .iter()
                        .for_each(|inner_instruction| {
                            if inner_instruction.program_id_index as usize == index {
                                return_ixs.push(instruction.clone());
                            }
                        })
                }
            });
            return_ixs
        }
        None => panic!("Program index could not be found"),
    }
}
