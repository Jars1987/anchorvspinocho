use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{self},
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_system::instructions::{CreateAccount, Transfer};

pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

use crate::{
    error::MyProgramError,
    state::{load_ix_data, DataLen, VaultState},
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DepositIxData {
    pub amount: u16,
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl DataLen for DepositIxData {
    const LEN: usize = core::mem::size_of::<DepositIxData>();
}

pub fn process_deposit(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // checks for accounts
    let [deposit_acc, vault_acc, state_acc, _sysvar_rent_acc, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !deposit_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !vault_acc.data_is_empty() {
        return Err(MyProgramError::InvalidAccount.into());
    }

    if !state_acc.data_is_empty() || unsafe { state_acc.owner().eq(&crate::ID) } {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let ix_data = load_ix_data::<DepositIxData>(data)?;

    let rent = Rent::get()?;

    let bump = [ix_data.state_bump];
    // Signer seeds
    let signer_seeds = [
        Seed::from("state".as_bytes()),
        Seed::from(deposit_acc.key()),
        Seed::from(&bump),
    ];
    let signers = [Signer::from(&signer_seeds[..])];
    // Create the governance config account
    CreateAccount {
        from: deposit_acc,
        to: state_acc,
        space: VaultState::LEN as u64,
        owner: &crate::ID,
        lamports: rent.minimum_balance(VaultState::LEN),
    }
    .invoke_signed(&signers)?;

    VaultState::initialize(state_acc, ix_data.vault_bump, ix_data.state_bump)?;

    let vault_pda = pubkey::create_program_address(
        &[
            "pinocchio_vault_pda".as_bytes(),
            deposit_acc.key(),
            &[ix_data.vault_bump],
        ],
        &crate::ID,
    )?;

    if vault_acc.key() != &vault_pda {
        return Err(ProgramError::InvalidAccountData);
    }

    Transfer {
        from: deposit_acc,
        to: vault_acc,
        lamports: ix_data.amount as u64 * LAMPORTS_PER_SOL,
    }
    .invoke()?;

    Ok(())
}
