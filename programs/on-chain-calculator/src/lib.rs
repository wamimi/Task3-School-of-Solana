// The task is to add code everywhere where you find // TODO.or todo!()
// You have to finish the implementation of methods of on-chain Calculator.
// Once you complete the TODOs, make sure that you delete all `todo!()` macros and
// you can try to run the tests using `anchor test` command and start debugging ;-)

use anchor_lang::prelude::*;

declare_id!("ARmiAGe6oAEq5BKguHydD3zt2n5PkV2Q5PLA1McuMkJT");

#[program]
pub mod on_chain_calculator {
    use super::*;
    // ------------------------------------------------------------------------------------------------
    // Functions
    //
    /// Initialize the Calculator with the corresponding operands and the corresponding update authority.
    /// Update authority is important to ensure that only privileged persons can modify calculator data.
    pub fn init_calculator(ctx: Context<InitializeCalculator>, x: i32, y: i32) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        let update_authority = ctx.accounts.update_authority.key();

        calculator.x = x;
        calculator.y = y;
        calculator.update_authority = update_authority;

        Ok(())
    }

    /// Update Operand X
    pub fn update_x(ctx: Context<ChangeInternalState>, new_x: i32) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.x = new_x;
        Ok(())
    }

    /// Update Operand Y
    pub fn update_y(ctx: Context<ChangeInternalState>, new_y: i32) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.y = new_y;
        Ok(())
    }

    /// Update Authority
    pub fn update_authority(ctx: Context<ChangeInternalState>, new_authority: Pubkey) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.update_authority = new_authority;
        Ok(())
    }

    /// This function reads data from the Calculator Account and
    /// performs an addition operation. The result, as well as the operands,
    /// are then emitted in the logs. We can then subscribe to the logs off-chain
    /// to check the correctness of the operands and the result.
    pub fn addition(ctx: Context<Compute>) -> Result<()> {
        let calculator = &ctx.accounts.calculator;

        let operand_x: i32 = calculator.x;
        let operand_y: i32 = calculator.y;

        let result: Option<i32> = calculator.addition();

        emit!(CalculatorEvent {
            x: operand_x,
            y: operand_y,
            result,
            op: Operation::Addition,
        });
        Ok(())
    }

    /// This function performs a subtraction operation.
    pub fn subtraction(ctx: Context<Compute>) -> Result<()> {
        let calculator = &ctx.accounts.calculator;

        let operand_x: i32 = calculator.x;
        let operand_y: i32 = calculator.y;

        let result: Option<i32> = calculator.subtraction();

        emit!(CalculatorEvent {
            x: operand_x,
            y: operand_y,
            result,
            op: Operation::Subtraction,
        });
        Ok(())
    }

    /// This function performs a multiplication operation.
    pub fn multiplication(ctx: Context<Compute>) -> Result<()> {
        let calculator = &ctx.accounts.calculator;

        let operand_x: i32 = calculator.x;
        let operand_y: i32 = calculator.y;

        let result: Option<i32> = calculator.multiplication();

        emit!(CalculatorEvent {
            x: operand_x,
            y: operand_y,
            result,
            op: Operation::Multiplication,
        });
        Ok(())
    }

    /// This function performs a division operation.
    pub fn division(ctx: Context<Compute>) -> Result<()> {
        let calculator = &ctx.accounts.calculator;

        let operand_x: i32 = calculator.x;
        let operand_y: i32 = calculator.y;

        let result: Option<i32> = calculator.division();

        emit!(CalculatorEvent {
            x: operand_x,
            y: operand_y,
            result,
            op: Operation::Division,
        });
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Contexts
//

#[derive(Accounts)]
pub struct InitializeCalculator<'info> {
    #[account(mut)]
    pub update_authority: Signer<'info>,
    #[account(init, payer = update_authority)]
    pub calculator: Account<'info, Calculator>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ChangeInternalState<'info> {
    pub update_authority: Signer<'info>,
    #[account(mut, has_one = update_authority @ CalculatorError::WrongPrivileges)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Compute<'info> {
    pub calculator: Account<'info, Calculator>,
}

// ------------------------------------------------------------------------------------------------
// Stored on-chain Data
//

#[account]
pub struct Calculator {
    pub x: i32,
    pub y: i32,
    pub update_authority: Pubkey,
}

impl Calculator {
    pub fn addition(&self) -> Option<i32> {
        self.x.checked_add(self.y)
    }
    pub fn subtraction(&self) -> Option<i32> {
        self.x.checked_sub(self.y)
    }
    pub fn multiplication(&self) -> Option<i32> {
        self.x.checked_mul(self.y)
    }
    pub fn division(&self) -> Option<i32> {
        self.x.checked_div(self.y)
    }
}

// ------------------------------------------------------------------------------------------------
// Error Codes
//

#[error_code]
pub enum CalculatorError {
    #[msg("You do not have sufficient privileges to updated the Calculator")]
    WrongPrivileges,
}

// ------------------------------------------------------------------------------------------------
// Predefined structure for emitting into logs
//

#[event]
pub struct CalculatorEvent {
    pub x: i32,
    pub y: i32,
    pub result: Option<i32>,
    pub op: Operation,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}
