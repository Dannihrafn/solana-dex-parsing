use bs58;
use std::collections::{HashMap, HashSet};
use types::{
    InnerInstruction, StructuredInstruction,
};
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

pub fn get_filtered_instructions(transaction: &SubscribeUpdateTransaction, account_keys: &Vec<String>, program_id: &str) -> Vec<StructuredInstruction> {
    let program_index = account_keys.iter().position(|key| *key == *program_id);
    match program_index {
        Some(_) => {}
        None => {
            panic!("Program index could not be found");
        }
    }
    let structured_instructions = structure_all_instructions(&transaction);
    let filtered_instructions = filter_instructions(&structured_instructions, account_keys, program_id);
    filtered_instructions
}

pub fn structure_all_instructions(transaction: &SubscribeUpdateTransaction) -> Vec<StructuredInstruction> {
    let max_depth: usize = 3;
    let txn = &transaction.transaction.clone().unwrap();
    let compiled_instructions =
        txn.transaction.clone().unwrap().message.unwrap().instructions;
    let inner_instructions = txn.meta.clone().unwrap().inner_instructions;

    if inner_instructions.is_empty() {
        return compiled_instructions
            .iter()
            .map(
                |instruction| StructuredInstruction {
                    account_key_indexes: instruction.accounts.clone(),
                    program_id_index: instruction.program_id_index as u8,
                    data: instruction.data.clone(),
                    inner_instructions: Vec::new(),
                },
            )
            .collect();
    }

    let mut formatted: Vec<StructuredInstruction> = Vec::new();

    for inner_instruction_group in inner_instructions.iter() {
        let parent_ix =
            &compiled_instructions[inner_instruction_group.index as usize];

        let mut parent = StructuredInstruction {
            account_key_indexes: parent_ix.accounts.clone(),
            program_id_index: parent_ix.program_id_index as u8,
            data: parent_ix.data.clone(),
            inner_instructions: Vec::new(),
        };

        // Working tree of depth-0 children
        let mut tree: Vec<InnerInstruction> = Vec::new();
        // Stack to track most recent instruction at each depth
        let mut stack: Vec<Option<InnerInstruction>> = Vec::new();
        // Track promoted instructions to avoid duplicates
        let mut promoted: HashSet<usize> = HashSet::new(); // Using index instead of the struct

        for (_, inner_instruction) in
            inner_instruction_group.instructions.iter().enumerate()
        {
            let depth: usize = inner_instruction
                .stack_height
                .unwrap_or(0)
                .try_into()
                .unwrap_or(0);

            let mut new_instruction = InnerInstruction {
                accounts: inner_instruction.accounts.clone(),
                data: inner_instruction.data.clone(),
                program_id_index: inner_instruction.program_id_index as u8,
                stack_height: depth as u8,
                inner_instructions: Vec::new(),
            };

            // Ensure stack is large enough
            while stack.len() <= depth {
                stack.push(None);
            }

            if depth == 0 {
                // True top-level under this parent
                tree.push(new_instruction.clone());
            } else {
                // Handle depth > maxDepth by promoting
                if depth > max_depth {
                    if let Some(Some(prev)) = stack.get(max_depth) {
                        if !promoted.contains(&max_depth) {
                            tree.push(prev.clone());
                            promoted.insert(max_depth);
                        }
                    }
                }

                // Pick the right parent in the stack
                let parent_depth = if depth > max_depth {
                    max_depth
                } else {
                    depth - 1
                };

                if let Some(Some(parent_in_stack)) = stack.get_mut(parent_depth) {
                    parent_in_stack
                        .inner_instructions
                        .push(new_instruction.clone());
                } else {
                    // Fallback if stack was missing
                    tree.push(new_instruction.clone());
                }
            }

            // Update stack at current depth
            stack[depth] = Some(new_instruction);
        }

        // Attach the assembled tree to parent
        parent.inner_instructions = tree;
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

pub fn filter_instructions_new(
    structured_instructions: &Vec<StructuredInstruction>,
    account_keys: &Vec<String>,
    program_ids: HashSet<String>,
) -> HashMap<String, Vec<StructuredInstruction>> {
    let mut ret: HashMap<String, Vec<StructuredInstruction>> = HashMap::new();
    for instruction in structured_instructions {
        let program_id = &account_keys[instruction.program_id_index as usize];
        if program_ids.contains(program_id) {
            ret.entry(program_id.into()).or_default().push(instruction.clone());
        }
    }
    ret
}
