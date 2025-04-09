use mollusk_svm::{program, Mollusk};
use mollusk_svm_bencher::MolluskComputeUnitBencher;
use solana_pinocchio_starter::{
    instruction::DepositIxData,
    state::{to_bytes, DataLen},
    ID,
};
use solana_sdk::pubkey;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
};

pub const PROGRAM: Pubkey = Pubkey::new_from_array(ID);

pub const RENT: Pubkey = pubkey!("SysvarRent111111111111111111111111111111111");

pub const PAYER: Pubkey = pubkey!("41LzznNicELmc5iCR9Jxke62a3v1VhzpBYodQF5AQwHX");

fn main() {
    let mollusk = Mollusk::new(&PROGRAM, "target/deploy/solana_pinocchio_starter");

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
    let ix_accounts0 = vec![
        AccountMeta::new(PAYER, true),
        AccountMeta::new(vault_pda, false),
        AccountMeta::new_readonly(system_program, false),
    ];

    // Create the instruction data
    let ix_data = DepositIxData {
        amount: 1,
        bump: bump,
    };

    // Ix discriminator = 0
    let mut ser_ix_data = vec![0];

    // Serialize the instruction data
    ser_ix_data.extend_from_slice(to_bytes(&ix_data));

    // Create instruction
    let instruction0 = Instruction::new_with_bytes(PROGRAM, &ser_ix_data, ix_accounts0);

    // Create tx_accounts vec
    let tx_accounts0 = &vec![
        (PAYER, payer_account.clone()),
        (vault_pda, vault_account.clone()),
        (system_program, system_account.clone()),
    ];

    //Withdraw
    let vault_account2 = Account::new(1, 0, &system_program);

    //Push the accounts in to the instruction_accounts vec!
    let ix_accounts1 = vec![
        AccountMeta::new(PAYER, true),
        AccountMeta::new(vault_pda, false),
        AccountMeta::new_readonly(system_program, false),
    ];

    // Ix discriminator = 1
    let mut ser_ix_data = vec![1];

    // Serialize the instruction data
    ser_ix_data.push(bump);

    // Create instruction
    let instruction1 = Instruction::new_with_bytes(PROGRAM, &ser_ix_data, ix_accounts1);
    // Create tx_accounts vec
    let tx_accounts1 = &vec![
        (PAYER, payer_account.clone()),
        (vault_pda, vault_account2.clone()),
        (system_program, system_account.clone()),
    ];

    MolluskComputeUnitBencher::new(mollusk)
        .bench(("Deposit", &instruction0, tx_accounts0))
        .bench(("Withdraw", &instruction1, tx_accounts1))
        .must_pass(true)
        .out_dir("benches/")
        .execute();
}
