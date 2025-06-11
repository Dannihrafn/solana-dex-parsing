use types::{
    MyCompiledInstruction, MyHeader, MyInnerInstruction, MyInnerInstructions, MyMessage, MyMeta,
    MyTokenBalance, MyTransaction, MyTransactionInner, MyUiTokenAmount,
};

use parser_pump_amm::PumpAmmInstructionParser;
use utils::{get_account_keys, structure_all_instructions};

fn main() {
    let transaction = MyTransaction {
        slot: 345090268,
        transaction: MyTransactionInner {
            message: MyMessage {
                header: MyHeader {
                    num_readonly_signed_accounts: 0,
                    num_readonly_unsigned_accounts: 12,
                    num_required_signatures: 2,
                },
                account_keys: [
                    [
                        111, 108, 103, 129, 162, 122, 1, 216, 29, 117, 247, 38, 85, 122, 112, 161,
                        4, 85, 228, 241, 168, 54, 198, 172, 224, 61, 20, 43, 251, 163, 135, 234,
                    ],
                    [
                        122, 230, 230, 13, 250, 81, 13, 29, 164, 77, 213, 255, 117, 198, 243, 172,
                        14, 165, 58, 117, 188, 132, 3, 219, 255, 222, 34, 135, 68, 91, 77, 86,
                    ],
                    [
                        202, 95, 156, 241, 24, 144, 35, 109, 30, 133, 190, 208, 158, 33, 154, 188,
                        29, 221, 87, 125, 207, 172, 250, 219, 23, 209, 156, 39, 127, 254, 162, 13,
                    ],
                    [
                        169, 107, 219, 216, 134, 217, 63, 228, 244, 102, 233, 107, 60, 39, 17, 253,
                        203, 33, 78, 153, 252, 190, 153, 156, 179, 102, 215, 224, 15, 101, 242, 87,
                    ],
                    [
                        210, 196, 76, 88, 231, 181, 175, 106, 162, 166, 26, 39, 8, 143, 51, 37,
                        157, 118, 136, 20, 46, 128, 174, 224, 187, 81, 144, 232, 48, 84, 119, 52,
                    ],
                    [
                        97, 180, 178, 228, 225, 95, 20, 186, 158, 89, 80, 158, 195, 0, 174, 100,
                        194, 116, 236, 24, 203, 198, 184, 97, 10, 110, 118, 174, 107, 209, 67, 230,
                    ],
                    [
                        137, 213, 69, 28, 9, 195, 73, 148, 148, 183, 119, 136, 223, 197, 39, 109,
                        249, 187, 154, 12, 13, 46, 100, 51, 109, 60, 12, 136, 34, 224, 243, 254,
                    ],
                    [
                        222, 67, 251, 129, 220, 64, 173, 242, 108, 112, 11, 93, 207, 201, 248, 95,
                        3, 173, 133, 14, 22, 70, 83, 238, 234, 96, 124, 251, 239, 92, 67, 214,
                    ],
                    [
                        87, 21, 87, 5, 90, 2, 209, 36, 81, 171, 57, 95, 119, 123, 221, 138, 62,
                        225, 174, 104, 189, 185, 242, 82, 150, 60, 215, 141, 238, 128, 181, 224,
                    ],
                    [
                        6, 137, 24, 247, 105, 5, 1, 177, 110, 82, 108, 9, 50, 49, 206, 155, 58, 32,
                        67, 85, 30, 128, 86, 154, 249, 90, 97, 166, 14, 195, 197, 64,
                    ],
                    [
                        3, 6, 70, 111, 229, 33, 23, 50, 255, 236, 173, 186, 114, 195, 155, 231,
                        188, 140, 229, 187, 197, 247, 18, 107, 44, 67, 155, 58, 64, 0, 0, 0,
                    ],
                    [
                        140, 151, 37, 143, 78, 36, 137, 241, 187, 61, 16, 41, 20, 142, 13, 131, 11,
                        90, 19, 153, 218, 255, 16, 132, 4, 142, 123, 216, 219, 233, 248, 89,
                    ],
                    [
                        6, 155, 136, 87, 254, 171, 129, 132, 251, 104, 127, 99, 70, 24, 192, 53,
                        218, 196, 57, 220, 26, 235, 59, 85, 152, 160, 240, 0, 0, 0, 0, 1,
                    ],
                    [
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0,
                    ],
                    [
                        6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121,
                        172, 28, 180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0,
                        169,
                    ],
                    [
                        12, 20, 222, 252, 130, 94, 198, 118, 148, 37, 8, 24, 187, 101, 64, 101,
                        244, 41, 141, 49, 86, 213, 113, 180, 212, 248, 9, 12, 24, 233, 168, 99,
                    ],
                    [
                        223, 138, 194, 188, 62, 128, 126, 238, 107, 100, 97, 68, 88, 221, 39, 76,
                        182, 222, 147, 220, 1, 103, 147, 26, 61, 96, 134, 84, 205, 124, 174, 57,
                    ],
                    [
                        137, 11, 166, 68, 254, 31, 85, 170, 25, 241, 28, 210, 210, 236, 20, 211,
                        35, 59, 110, 10, 75, 234, 238, 247, 43, 105, 133, 142, 33, 225, 112, 214,
                    ],
                    [
                        92, 58, 126, 51, 43, 179, 73, 224, 10, 121, 143, 40, 248, 181, 52, 25, 193,
                        5, 19, 29, 158, 145, 72, 254, 195, 248, 130, 96, 40, 222, 196, 20,
                    ],
                    [
                        96, 140, 204, 29, 252, 233, 97, 180, 59, 119, 156, 25, 21, 5, 166, 226,
                        211, 191, 69, 213, 164, 219, 70, 24, 173, 118, 200, 45, 97, 117, 69, 53,
                    ],
                    [
                        229, 74, 112, 149, 40, 131, 159, 97, 192, 185, 184, 96, 121, 137, 28, 19,
                        146, 22, 228, 122, 113, 182, 47, 183, 59, 236, 114, 22, 148, 88, 116, 94,
                    ],
                    [
                        109, 101, 144, 67, 186, 165, 42, 24, 27, 88, 68, 35, 158, 141, 84, 45, 31,
                        222, 220, 214, 129, 206, 126, 113, 208, 117, 41, 218, 201, 110, 38, 173,
                    ],
                ]
                .to_vec(),
                address_table_lookups: vec![],
                versioned: true,
                instructions: [
                    MyCompiledInstruction {
                        accounts: vec![],
                        data: [2, 144, 208, 3, 0].to_vec(),
                        program_id_index: 10,
                    },
                    MyCompiledInstruction {
                        accounts: vec![],
                        data: [3, 16, 39, 0, 0, 0, 0, 0, 0].to_vec(),
                        program_id_index: 10,
                    },
                    MyCompiledInstruction {
                        accounts: [0, 2, 0, 12, 13, 14].to_vec(),
                        data: [].to_vec(),
                        program_id_index: 11,
                    },
                    MyCompiledInstruction {
                        accounts: [0, 2].to_vec(),
                        data: [2, 0, 0, 0, 64, 34, 151, 175, 1, 0, 0, 0].to_vec(),
                        program_id_index: 13,
                    },
                    MyCompiledInstruction {
                        accounts: [2].to_vec(),
                        data: [17].to_vec(),
                        program_id_index: 14,
                    },
                    MyCompiledInstruction {
                        accounts: [
                            16, 0, 17, 12, 18, 2, 3, 4, 5, 19, 6, 14, 14, 13, 11, 20, 15, 7, 21,
                        ]
                        .to_vec(),
                        data: [
                            51, 230, 133, 164, 1, 127, 131, 173, 64, 34, 151, 175, 1, 0, 0, 0, 75,
                            118, 85, 46, 146, 10, 0, 0,
                        ]
                        .to_vec(),
                        program_id_index: 15,
                    },
                    MyCompiledInstruction {
                        accounts: [2, 0, 0].to_vec(),
                        data: [9].to_vec(),
                        program_id_index: 14,
                    },
                    MyCompiledInstruction {
                        accounts: [1, 8, 1, 12, 13, 14].to_vec(),
                        data: [].to_vec(),
                        program_id_index: 11,
                    },
                    MyCompiledInstruction {
                        accounts: [
                            16, 1, 17, 12, 18, 8, 9, 4, 5, 19, 6, 14, 14, 13, 11, 20, 15, 7, 21,
                        ]
                        .to_vec(),
                        data: [
                            102, 6, 61, 18, 1, 218, 235, 234, 80, 129, 116, 54, 2, 0, 0, 0, 217,
                            56, 55, 88, 219, 14, 0, 0,
                        ]
                        .to_vec(),
                        program_id_index: 15,
                    },
                    MyCompiledInstruction {
                        accounts: [8, 1, 1].to_vec(),
                        data: [9].to_vec(),
                        program_id_index: 14,
                    },
                ]
                .to_vec(),
            },
            signatures: [
                [
                    130, 62, 205, 248, 212, 246, 2, 171, 232, 210, 49, 229, 159, 68, 72, 0, 180,
                    182, 58, 38, 84, 122, 80, 249, 191, 128, 99, 52, 88, 69, 40, 245, 86, 105, 46,
                    7, 198, 21, 0, 4, 21, 224, 110, 240, 79, 235, 33, 219, 11, 4, 144, 137, 19,
                    115, 156, 131, 97, 148, 84, 45, 137, 7, 142, 5,
                ]
                .to_vec(),
                [
                    148, 76, 87, 134, 14, 23, 71, 205, 171, 242, 233, 247, 23, 197, 104, 118, 129,
                    72, 72, 175, 224, 167, 155, 23, 129, 103, 189, 152, 9, 182, 87, 249, 86, 85,
                    101, 43, 22, 103, 174, 93, 209, 145, 194, 189, 125, 16, 98, 170, 243, 254, 247,
                    161, 96, 21, 247, 78, 1, 58, 29, 54, 232, 6, 0, 7,
                ]
                .to_vec(),
            ]
            .to_vec(),
        },
        meta: MyMeta {
            inner_instructions: [
                MyInnerInstructions {
                    index: 2,
                    instructions: [
                        MyInnerInstruction {
                            accounts: [12].to_vec(),
                            data: [21, 7, 0].to_vec(),
                            program_id_index: 14,
                            stack_height: Some(2),
                        },
                        MyInnerInstruction {
                            accounts: [0, 2].to_vec(),
                            data: [
                                0, 0, 0, 0, 240, 29, 31, 0, 0, 0, 0, 0, 165, 0, 0, 0, 0, 0, 0, 0,
                                6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235,
                                121, 172, 28, 180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133,
                                126, 255, 0, 169,
                            ]
                            .to_vec(),
                            program_id_index: 13,
                            stack_height: Some(2),
                        },
                        MyInnerInstruction {
                            accounts: [2].to_vec(),
                            data: [22].to_vec(),
                            program_id_index: 14,
                            stack_height: Some(2),
                        },
                        MyInnerInstruction {
                            accounts: [2, 12].to_vec(),
                            data: [
                                18, 111, 108, 103, 129, 162, 122, 1, 216, 29, 117, 247, 38, 85,
                                122, 112, 161, 4, 85, 228, 241, 168, 54, 198, 172, 224, 61, 20, 43,
                                251, 163, 135, 234,
                            ]
                            .to_vec(),
                            program_id_index: 14,
                            stack_height: Some(2),
                        },
                    ]
                    .to_vec(),
                },
                MyInnerInstructions {
                    index: 5,
                    instructions: [
                        MyInnerInstruction {
                            accounts: [2, 12, 4, 0].to_vec(),
                            data: [12, 64, 34, 151, 175, 1, 0, 0, 0, 9].to_vec(),
                            program_id_index: 14,
                            stack_height: Some(2),
                        },
                        MyInnerInstruction {
                            accounts: [5, 18, 3, 16].to_vec(),
                            data: [12, 253, 175, 133, 57, 194, 10, 0, 0, 6].to_vec(),
                            program_id_index: 14,
                            stack_height: Some(2),
                        },
                        MyInnerInstruction {
                            accounts: [5, 18, 6, 16].to_vec(),
                            data: [12, 93, 160, 108, 97, 1, 0, 0, 0, 6].to_vec(),
                            program_id_index: 14,
                            stack_height: Some(2),
                        },
                        MyInnerInstruction {
                            accounts: [20].to_vec(),
                            data: [
                                228, 69, 165, 46, 81, 203, 154, 29, 62, 47, 55, 10, 165, 3, 220,
                                42, 98, 104, 67, 104, 0, 0, 0, 0, 64, 34, 151, 175, 1, 0, 0, 0, 75,
                                118, 85, 46, 146, 10, 0, 0, 64, 34, 151, 175, 1, 0, 0, 0, 164, 161,
                                82, 167, 152, 2, 0, 0, 82, 15, 7, 130, 71, 0, 0, 0, 218, 28, 253,
                                33, 67, 212, 1, 0, 204, 209, 164, 32, 201, 10, 0, 0, 20, 0, 0, 0,
                                0, 0, 0, 0, 114, 129, 178, 133, 5, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0,
                                93, 160, 108, 97, 1, 0, 0, 0, 90, 80, 242, 154, 195, 10, 0, 0, 253,
                                175, 133, 57, 194, 10, 0, 0, 223, 138, 194, 188, 62, 128, 126, 238,
                                107, 100, 97, 68, 88, 221, 39, 76, 182, 222, 147, 220, 1, 103, 147,
                                26, 61, 96, 134, 84, 205, 124, 174, 57, 111, 108, 103, 129, 162,
                                122, 1, 216, 29, 117, 247, 38, 85, 122, 112, 161, 4, 85, 228, 241,
                                168, 54, 198, 172, 224, 61, 20, 43, 251, 163, 135, 234, 202, 95,
                                156, 241, 24, 144, 35, 109, 30, 133, 190, 208, 158, 33, 154, 188,
                                29, 221, 87, 125, 207, 172, 250, 219, 23, 209, 156, 39, 127, 254,
                                162, 13, 169, 107, 219, 216, 134, 217, 63, 228, 244, 102, 233, 107,
                                60, 39, 17, 253, 203, 33, 78, 153, 252, 190, 153, 156, 179, 102,
                                215, 224, 15, 101, 242, 87, 96, 140, 204, 29, 252, 233, 97, 180,
                                59, 119, 156, 25, 21, 5, 166, 226, 211, 191, 69, 213, 164, 219, 70,
                                24, 173, 118, 200, 45, 97, 117, 69, 53, 137, 213, 69, 28, 9, 195,
                                73, 148, 148, 183, 119, 136, 223, 197, 39, 109, 249, 187, 154, 12,
                                13, 46, 100, 51, 109, 60, 12, 136, 34, 224, 243, 254, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            ]
                            .to_vec(),
                            program_id_index: 15,
                            stack_height: Some(2),
                        },
                    ]
                    .to_vec(),
                },
                MyInnerInstructions {
                    index: 7,
                    instructions: [
                        MyInnerInstruction {
                            accounts: [12].to_vec(),
                            data: [21, 7, 0].to_vec(),
                            program_id_index: 14,
                            stack_height: Some(2),
                        },
                        MyInnerInstruction {
                            accounts: [1, 8].to_vec(),
                            data: [
                                0, 0, 0, 0, 240, 29, 31, 0, 0, 0, 0, 0, 165, 0, 0, 0, 0, 0, 0, 0,
                                6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235,
                                121, 172, 28, 180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133,
                                126, 255, 0, 169,
                            ]
                            .to_vec(),
                            program_id_index: 13,
                            stack_height: Some(2),
                        },
                        MyInnerInstruction {
                            accounts: [8].to_vec(),
                            data: [22].to_vec(),
                            program_id_index: 14,
                            stack_height: Some(2),
                        },
                        MyInnerInstruction {
                            accounts: [8, 12].to_vec(),
                            data: [
                                18, 122, 230, 230, 13, 250, 81, 13, 29, 164, 77, 213, 255, 117,
                                198, 243, 172, 14, 165, 58, 117, 188, 132, 3, 219, 255, 222, 34,
                                135, 68, 91, 77, 86,
                            ]
                            .to_vec(),
                            program_id_index: 14,
                            stack_height: Some(2),
                        },
                    ]
                    .to_vec(),
                },
                MyInnerInstructions {
                    index: 8,
                    instructions: [
                        MyInnerInstruction {
                            accounts: [4, 12, 8, 16].to_vec(),
                            data: [12, 80, 129, 116, 54, 2, 0, 0, 0, 9].to_vec(),
                            program_id_index: 14,
                            stack_height: Some(2),
                        },
                        MyInnerInstruction {
                            accounts: [9, 18, 5, 1].to_vec(),
                            data: [12, 103, 214, 216, 77, 74, 14, 0, 0, 6].to_vec(),
                            program_id_index: 14,
                            stack_height: Some(2),
                        },
                        MyInnerInstruction {
                            accounts: [9, 18, 6, 1].to_vec(),
                            data: [12, 78, 9, 84, 211, 1, 0, 0, 0, 6].to_vec(),
                            program_id_index: 14,
                            stack_height: Some(2),
                        },
                        MyInnerInstruction {
                            accounts: [20].to_vec(),
                            data: [
                                228, 69, 165, 46, 81, 203, 154, 29, 103, 244, 82, 31, 44, 245, 119,
                                119, 98, 104, 67, 104, 0, 0, 0, 0, 80, 129, 116, 54, 2, 0, 0, 0,
                                217, 56, 55, 88, 219, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 166, 113,
                                23, 5, 108, 44, 0, 0, 146, 49, 158, 49, 73, 0, 0, 0, 128, 204, 10,
                                135, 127, 201, 1, 0, 47, 177, 136, 0, 67, 14, 0, 0, 20, 0, 0, 0, 0,
                                0, 0, 0, 56, 37, 80, 77, 7, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 78, 9,
                                84, 211, 1, 0, 0, 0, 103, 214, 216, 77, 74, 14, 0, 0, 181, 223, 44,
                                33, 76, 14, 0, 0, 223, 138, 194, 188, 62, 128, 126, 238, 107, 100,
                                97, 68, 88, 221, 39, 76, 182, 222, 147, 220, 1, 103, 147, 26, 61,
                                96, 134, 84, 205, 124, 174, 57, 122, 230, 230, 13, 250, 81, 13, 29,
                                164, 77, 213, 255, 117, 198, 243, 172, 14, 165, 58, 117, 188, 132,
                                3, 219, 255, 222, 34, 135, 68, 91, 77, 86, 87, 21, 87, 5, 90, 2,
                                209, 36, 81, 171, 57, 95, 119, 123, 221, 138, 62, 225, 174, 104,
                                189, 185, 242, 82, 150, 60, 215, 141, 238, 128, 181, 224, 6, 137,
                                24, 247, 105, 5, 1, 177, 110, 82, 108, 9, 50, 49, 206, 155, 58, 32,
                                67, 85, 30, 128, 86, 154, 249, 90, 97, 166, 14, 195, 197, 64, 96,
                                140, 204, 29, 252, 233, 97, 180, 59, 119, 156, 25, 21, 5, 166, 226,
                                211, 191, 69, 213, 164, 219, 70, 24, 173, 118, 200, 45, 97, 117,
                                69, 53, 137, 213, 69, 28, 9, 195, 73, 148, 148, 183, 119, 136, 223,
                                197, 39, 109, 249, 187, 154, 12, 13, 46, 100, 51, 109, 60, 12, 136,
                                34, 224, 243, 254, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            ]
                            .to_vec(),
                            program_id_index: 15,
                            stack_height: Some(2),
                        },
                    ]
                    .to_vec(),
                },
            ]
            .to_vec(),
            loaded_readonly_addresses: [].to_vec(),
            loaded_writable_addresses: [].to_vec(),
            pre_balances: [
                18797493027,
                1944689572,
                0,
                2039280,
                307126218050,
                2039280,
                2039280,
                2039280,
                0,
                2039280,
                1,
                731913600,
                1058282318278,
                1,
                934087680,
                1141440,
                2582160,
                4454454,
                1461600,
                10702543686917,
                0,
                0,
            ]
            .to_vec(),
            post_balances: [
                11556595727,
                11448229108,
                0,
                2039280,
                304863563314,
                2039280,
                2039280,
                2039280,
                0,
                2039280,
                1,
                731913600,
                1058282318278,
                1,
                934087680,
                1141440,
                2582160,
                4454454,
                1461600,
                10702543686917,
                0,
                0,
            ]
            .to_vec(),
            pre_token_balances: [
                MyTokenBalance {
                    mint: "7D2EuFq7rMQkamGnGaTHfp7Rr67KcdS8gnM6hVYPwtb5".to_string(),
                    owner: "8Vx8Qs3ii5GYFu9FVWmpS76AeuAXRvehxSSvZo15WCAD".to_string(),
                    account_index: 3,
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
                    ui_token_amount: MyUiTokenAmount {
                        ui_amount: 2854665.494948,
                        ui_amount_string: "2854665.494948".to_string(),
                        decimals: 6,
                        amount: "2854665494948".to_string(),
                    },
                },
                MyTokenBalance {
                    mint: "So11111111111111111111111111111111111111112".to_string(),
                    owner: "G3cdxit9pwSCAY6ZKvp3KYcks2ZBDcTUKAeyARyakyzg".to_string(),
                    account_index: 4,
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
                    ui_token_amount: MyUiTokenAmount {
                        ui_amount: 307.12417877,
                        ui_amount_string: "307.12417877".to_string(),
                        decimals: 9,
                        amount: "307124178770".to_string(),
                    },
                },
                MyTokenBalance {
                    mint: "7D2EuFq7rMQkamGnGaTHfp7Rr67KcdS8gnM6hVYPwtb5".to_string(),
                    owner: "G3cdxit9pwSCAY6ZKvp3KYcks2ZBDcTUKAeyARyakyzg".to_string(),
                    account_index: 5,
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
                    ui_token_amount: MyUiTokenAmount {
                        ui_amount: 514859774.844122,
                        ui_amount_string: "514859774.844122".to_string(),
                        decimals: 6,
                        amount: "514859774844122".to_string(),
                    },
                },
                MyTokenBalance {
                    mint: "7D2EuFq7rMQkamGnGaTHfp7Rr67KcdS8gnM6hVYPwtb5".to_string(),
                    owner: "7VtfL8fvgNfhz17qKRMjzQEXgbdpnHHHQRh54R9jP2RJ".to_string(),
                    account_index: 6,
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
                    ui_token_amount: MyUiTokenAmount {
                        ui_amount: 3484707.54004,
                        ui_amount_string: "3484707.54004".to_string(),
                        decimals: 6,
                        amount: "3484707540040".to_string(),
                    },
                },
                MyTokenBalance {
                    mint: "7D2EuFq7rMQkamGnGaTHfp7Rr67KcdS8gnM6hVYPwtb5".to_string(),
                    owner: "8N3GDaZ2iwN65oxVatKTLPNooAVUJTbfiVJ1ahyqwjSk".to_string(),
                    account_index: 7,
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
                    ui_token_amount: MyUiTokenAmount {
                        ui_amount: 0.0,
                        ui_amount_string: "0".to_string(),
                        decimals: 6,
                        amount: "0".to_string(),
                    },
                },
                MyTokenBalance {
                    mint: "7D2EuFq7rMQkamGnGaTHfp7Rr67KcdS8gnM6hVYPwtb5".to_string(),
                    owner: "9GkwvvaYHe5jGD6ybzJFVzp4ukxFvPK8Tr5L5UKs4NWy".to_string(),
                    account_index: 9,
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
                    ui_token_amount: MyUiTokenAmount {
                        ui_amount: 48842453.512614,
                        ui_amount_string: "48842453.512614".to_string(),
                        decimals: 6,
                        amount: "48842453512614".to_string(),
                    },
                },
            ]
            .to_vec(),
            post_token_balances: [
                MyTokenBalance {
                    mint: "7D2EuFq7rMQkamGnGaTHfp7Rr67KcdS8gnM6hVYPwtb5".to_string(),
                    owner: "8Vx8Qs3ii5GYFu9FVWmpS76AeuAXRvehxSSvZo15WCAD".to_string(),
                    account_index: 3,
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
                    ui_token_amount: MyUiTokenAmount {
                        ui_amount: 14683970.490785,
                        ui_amount_string: "14683970.490785".to_string(),
                        decimals: 6,
                        amount: "14683970490785".to_string(),
                    },
                },
                MyTokenBalance {
                    mint: "So11111111111111111111111111111111111111112".to_string(),
                    owner: "G3cdxit9pwSCAY6ZKvp3KYcks2ZBDcTUKAeyARyakyzg".to_string(),
                    account_index: 4,
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
                    ui_token_amount: MyUiTokenAmount {
                        ui_amount: 304.861524034,
                        ui_amount_string: "304.861524034".to_string(),
                        decimals: 9,
                        amount: "304861524034".to_string(),
                    },
                },
                MyTokenBalance {
                    mint: "7D2EuFq7rMQkamGnGaTHfp7Rr67KcdS8gnM6hVYPwtb5".to_string(),
                    owner: "G3cdxit9pwSCAY6ZKvp3KYcks2ZBDcTUKAeyARyakyzg".to_string(),
                    account_index: 5,
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
                    ui_token_amount: MyUiTokenAmount {
                        ui_amount: 518736836.797159,
                        ui_amount_string: "518736836.797159".to_string(),
                        decimals: 6,
                        amount: "518736836797159".to_string(),
                    },
                },
                MyTokenBalance {
                    mint: "7D2EuFq7rMQkamGnGaTHfp7Rr67KcdS8gnM6hVYPwtb5".to_string(),
                    owner: "7VtfL8fvgNfhz17qKRMjzQEXgbdpnHHHQRh54R9jP2RJ".to_string(),
                    account_index: 6,
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
                    ui_token_amount: MyUiTokenAmount {
                        ui_amount: 3498477.483507,
                        ui_amount_string: "3498477.483507".to_string(),
                        decimals: 6,
                        amount: "3498477483507".to_string(),
                    },
                },
                MyTokenBalance {
                    mint: "7D2EuFq7rMQkamGnGaTHfp7Rr67KcdS8gnM6hVYPwtb5".to_string(),
                    owner: "8N3GDaZ2iwN65oxVatKTLPNooAVUJTbfiVJ1ahyqwjSk".to_string(),
                    account_index: 7,
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
                    ui_token_amount: MyUiTokenAmount {
                        ui_amount: 0.0,
                        ui_amount_string: "0".to_string(),
                        decimals: 6,
                        amount: "0".to_string(),
                    },
                },
                MyTokenBalance {
                    mint: "7D2EuFq7rMQkamGnGaTHfp7Rr67KcdS8gnM6hVYPwtb5".to_string(),
                    owner: "9GkwvvaYHe5jGD6ybzJFVzp4ukxFvPK8Tr5L5UKs4NWy".to_string(),
                    account_index: 9,
                    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
                    ui_token_amount: MyUiTokenAmount {
                        ui_amount: 33122316.620273,
                        ui_amount_string: "33122316.620273".to_string(),
                        decimals: 6,
                        amount: "33122316620273".to_string(),
                    },
                },
            ]
            .to_vec(),
        },
    };

    let parser = PumpAmmInstructionParser::new();
    let decoded_tx = parser.decode_transaction(&transaction);
    println!("{:?}", decoded_tx);
}
