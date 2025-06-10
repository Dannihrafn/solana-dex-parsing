use bs58;
use std::collections::HashSet;
use types::{
    InnerInstruction, MyCompiledInstruction, MyInnerInstructions, MyTransaction,
    StructuredInstruction,
};

pub fn get_account_keys(transaction: MyTransaction) -> Vec<String> {
    let static_account_keys = transaction.transaction.message.accountKeys;
    let loaded_addresses_readonly = transaction.meta.loadedReadonlyAddresses;
    let loaded_addresses_writable = transaction.meta.loadedWritableAddresses;
    let mut account_keys = static_account_keys.clone();
    account_keys.extend(loaded_addresses_writable);
    account_keys.extend(loaded_addresses_readonly);
    account_keys
        .into_iter()
        .map(|key| bs58::encode(key).into_string())
        .collect()
}

pub fn get_filtered_instructions(
    account_keys: Vec<String>,
    transaction: MyTransaction,
    program_id: String,
) -> Vec<StructuredInstruction> {
    let program_index = account_keys.iter().position(|key| *key == program_id);
    match program_index {
        Some(_) => {}
        None => {
            panic!("Program index could not be found");
        }
    }
    let structured_instructions = structure_all_instructions(transaction);
    let filtered_instructions =
        filter_instructions(structured_instructions, account_keys, program_id);
    filtered_instructions
}

pub fn structure_all_instructions(transaction: MyTransaction) -> Vec<StructuredInstruction> {
    let compiled_instructions = transaction.transaction.message.instructions;
    let inner_instructions = transaction.meta.innerInstructions;

    if inner_instructions.is_empty() {
        return compiled_instructions
            .iter()
            .map(|instruction| {
                return StructuredInstruction {
                    account_key_indexes: instruction.accounts.clone(),
                    program_id_index: instruction.programIdIndex.clone() as u8,
                    data: instruction.data.clone(),
                    inner_instructions: Vec::new(),
                };
            })
            .collect();
    }

    let mut formatted: Vec<StructuredInstruction> = Vec::new();

    inner_instructions.iter().for_each(|instruction| {
        let parent = &compiled_instructions[instruction.index as usize];
    });
    todo!();
}

pub fn filter_instructions(
    structured_instructions: Vec<StructuredInstruction>,
    account_keys: Vec<String>,
    program_id: String,
) -> Vec<StructuredInstruction> {
    let program_index = account_keys.iter().position(|key| *key == program_id);
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
