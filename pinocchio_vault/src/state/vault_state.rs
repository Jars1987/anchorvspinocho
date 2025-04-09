use super::{load_acc_mut_unchecked, DataLen};
use pinocchio::{account_info::AccountInfo, program_error::ProgramError};

#[repr(C)]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl DataLen for VaultState {
    const LEN: usize = core::mem::size_of::<Self>();
}

impl VaultState {
    pub fn initialize(state_acc: &AccountInfo, vault: u8, state: u8) -> Result<(), ProgramError> {
        let vault_state =
            unsafe { load_acc_mut_unchecked::<VaultState>(state_acc.borrow_mut_data_unchecked())? };
        vault_state.vault_bump = vault;
        vault_state.state_bump = state;

        Ok(())
    }
}
