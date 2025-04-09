use pinocchio::program_error::ProgramError;

pub mod deposit;
pub mod withdraw;

pub use deposit::*;
pub use withdraw::*;

#[repr(u8)]
pub enum MyProgramInstrution {
    Deposit,
    Withdraw,
}

impl TryFrom<&u8> for MyProgramInstrution {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(MyProgramInstrution::Deposit),
            1 => Ok(MyProgramInstrution::Withdraw),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
