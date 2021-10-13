use std::convert::TryInto;
use solana_program::program_error::ProgramError;

use crate::error::EscrowError::InvalidInstruction;
pub enum EscrowInstruction {
    /// Using for starting the trade
    /// Accountss
    /// [signer] -> account of the person who starts the escrow
    /// [writable] -> temporary token that needs to be created for the transactions
    /// [] -> token accoutn for token they will receive
    /// [writable] -> the escrow account, it holds all necessary information
    /// [] the rent sysvar
    /// [] the token program
    InitEscrow {
        /// the amourt party A expects to receive of token Y
        amount: u64
    }
}

impl EscrowInstruction{
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag,rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitEscrow{
                amount:Self::unpack_amount(rest)?
            },
            _=> return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input:&[u8]) -> Result<u64, ProgramError>{
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}
