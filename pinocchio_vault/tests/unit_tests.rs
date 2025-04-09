use mollusk_svm::result::{Check, ProgramResult};
use mollusk_svm::{program, Mollusk};
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
extern crate alloc;
use alloc::vec;

use solana_pinocchio_starter::instruction::DepositIxData;
use solana_pinocchio_starter::state::{to_bytes, DataLen};
use solana_pinocchio_starter::ID;

pub const PROGRAM: Pubkey = Pubkey::new_from_array(ID);

pub const RENT: Pubkey = pubkey!("SysvarRent111111111111111111111111111111111");

pub const PAYER: Pubkey = pubkey!("41LzznNicELmc5iCR9Jxke62a3v1VhzpBYodQF5AQwHX");

pub fn mollusk() -> Mollusk {
    let mollusk = Mollusk::new(&PROGRAM, "target/deploy/solana_pinocchio_starter");
    mollusk
}

#[test]
fn test_deposit() {
    let mollusk = mollusk();

    //system program and system account
    let (system_program, system_account) = program::keyed_account_for_system_program();

    // Create the PDA
    let (vault_pda, bump) = Pubkey::find_program_address(
        &["pinocchio_vault_pda".as_bytes(), &PAYER.to_bytes()],
        &PROGRAM,
    );

    //Initialize the accounts
    let payer_account = Account::new(10 * LAMPORTS_PER_SOL, 0, &system_program);
    let vault_account = Account::new(0, 0, &system_program);

    //Push the accounts in to the instruction_accounts vec!
    let ix_accounts = vec![
        AccountMeta::new(PAYER, true),
        AccountMeta::new(vault_pda, false),
        AccountMeta::new_readonly(system_program, false),
    ];

    // Create the instruction data
    let ix_data = DepositIxData { amount: 1, bump };

    // Ix discriminator = 0
    let mut ser_ix_data = vec![0];

    // Serialize the instruction data
    ser_ix_data.extend_from_slice(to_bytes(&ix_data));

    // Create instruction
    let instruction = Instruction::new_with_bytes(PROGRAM, &ser_ix_data, ix_accounts);

    // Create tx_accounts vec
    let tx_accounts = &vec![
        (PAYER, payer_account.clone()),
        (vault_pda, vault_account.clone()),
        (system_program, system_account.clone()),
    ];

    let init_res =
        mollusk.process_and_validate_instruction(&instruction, tx_accounts, &[Check::success()]);

    assert!(init_res.program_result == ProgramResult::Success);
}

#[test]
fn test_withdraw() {
    let mollusk = mollusk();

    //system program and system account
    let (system_program, system_account) = program::keyed_account_for_system_program();

    // Create the PDA
    let (vault_pda, bump) = Pubkey::find_program_address(
        &["pinocchio_vault_pda".as_bytes(), &PAYER.to_bytes()],
        &PROGRAM,
    );

    //Initialize the accounts
    let payer_account = Account::new(9 * LAMPORTS_PER_SOL, 0, &system_program);
    let vault_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

    //Push the accounts in to the instruction_accounts vec!
    let ix_accounts = vec![
        AccountMeta::new(PAYER, true),
        AccountMeta::new(vault_pda, false),
        AccountMeta::new_readonly(system_program, false),
    ];

    // Ix discriminator = 1
    let mut ix_data = vec![1];

    // Serialize the instruction data
    ix_data.push(bump);

    // Create instruction
    let instruction = Instruction::new_with_bytes(PROGRAM, &ix_data, ix_accounts);
    // Create tx_accounts vec
    let tx_accounts = &vec![
        (PAYER, payer_account.clone()),
        (vault_pda, vault_account.clone()),
        (system_program, system_account.clone()),
    ];

    let update_res =
        mollusk.process_and_validate_instruction(&instruction, tx_accounts, &[Check::success()]);

    assert!(update_res.program_result == ProgramResult::Success);
}
