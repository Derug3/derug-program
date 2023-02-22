use anchor_lang::prelude::*;
pub mod constants;
pub mod instructions;
pub mod state;
use instructions::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod derug_program {
    use super::*;
}
