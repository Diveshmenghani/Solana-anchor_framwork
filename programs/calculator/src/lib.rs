use anchor_lang::prelude::*;

declare_id!("Bx1T1k9Z3X8BQV4LfxkBPCPvK8SkvTbtPirxM8HviyXq");

#[program]
pub mod calculator {
    use super::*;


    pub fn initialize_result(ctx: Context<InitializeResult>) -> Result<()> {
        ctx.accounts.output.result = 0;
        Ok(())
    }
    pub fn addition(ctx: Context<Addition>, a: i64, b: i64) -> Result<()> {
        ctx.accounts.output.result = a + b;
        Ok(())
    }
    pub fn subtraction(ctx: Context<Subtraction>, a: i64, b: i64) -> Result<()> {
        ctx.accounts.output.result = a - b;
        Ok(())
    }
    pub fn divison(ctx: Context<Divison>, a: i64, b: i64) -> Result<()> {
        ctx.accounts.output.result = a / b;
        Ok(())
    }
    pub fn multiplication(ctx: Context<Multiply>, a: i64, b: i64) -> Result<()> {
        ctx.accounts.output.result = a * b;
        Ok(())
    }
}


#[account]
#[derive(InitSpace)]
pub struct ResultCalci {
    result: i64,
}


#[derive(Accounts)]
pub struct InitializeResult<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,


    #[account(init,payer=payer,space=8+ResultCalci::INIT_SPACE)]
    pub output: Account<'info, ResultCalci>,


    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Multiply<'info> {
    #[account(mut)]
    pub output: Account<'info, ResultCalci>,
}
#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub output: Account<'info, ResultCalci>,
}
#[derive(Accounts)]
pub struct Subtraction<'info> {
    #[account(mut)]
    pub output: Account<'info, ResultCalci>,
}
#[derive(Accounts)]
pub struct Divison<'info> {
    #[account(mut)]
    pub output: Account<'info, ResultCalci>,
}