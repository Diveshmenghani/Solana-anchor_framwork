#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;


declare_id!("4fsMyC9cJKskb3ubrYejxk2kZ2YMJBKpwCydiEsKKSKd");


#[program]
pub mod todo {
   use super::*;


   pub fn create_todo(ctx: Context<CreateToDo>,name:String,priority:u8) -> Result<()> {
      let todo = &mut ctx.accounts.list;    
       todo.set_inner(ToDo { owner: ctx.accounts.owner.key(), name, priority});
       Ok(())
   }
     pub fn update_priority(ctx: Context<UpdatePriority>,priority:u8) -> Result<()> {
       let todo = &mut ctx.accounts.list;
       todo.priority=priority;
       Ok(())
   }


   pub fn update_name(ctx: Context<UpdateName>,_name:String,newName:String) -> Result<()> {
       let todo = &mut ctx.accounts.list;
       todo.name=newName;
       Ok(())
   }
     pub fn delete_todo(_ctx: Context<DeleteToDo>) -> Result<()> {
       Ok(())
   }
}
//Blockchain  + wallet  = abc
//Solidity + wallet = abc

// b"



#[account]
#[derive(InitSpace)]
pub struct ToDo{
 #[max_len(50)]
   name:String,
   owner:Pubkey,
   priority:u8,
}
#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateToDo<'info> {
    #[account(mut)]
   pub owner: Signer<'info>,


  #[account(
     init,
     seeds = [name.as_bytes(), owner.key().as_ref()],
     bump,
     space = 8 + ToDo::INIT_SPACE,
     payer=owner,
  )]
   pub list: Account<'info, ToDo>,


   pub system_program: Program<'info, System>,


}


#[derive(Accounts)]

pub struct UpdatePriority<'info>{
  #[account(mut)]
  pub owner:Signer<'info>,
  #[account(
   mut,
)]
 pub list:Account<'info,ToDo>,


pub sytem_program:Program<'info,System>,


}


#[derive(Accounts)]
#[instruction(name: String)]
pub struct UpdateName<'info> {
   #[account(mut)]
   pub owner: Signer<'info>,


   #[account(
       mut,
       seeds = [name.as_bytes(), owner.key().as_ref()],
       bump,
       realloc = 8 + ToDo::INIT_SPACE,
       realloc::payer = owner,
       realloc::zero = true
   )]
   pub list: Account<'info, ToDo>,


   pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct DeleteToDo<'info>{
  #[account(mut)]
  pub owner:Signer<'info>,

  #[account(
   mut,
   close=owner
)]
 pub list:Account<'info,ToDo>,

 pub sytem_program:Program<'info,System>,


}

