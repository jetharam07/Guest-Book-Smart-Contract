use anchor_lang::prelude::*;

declare_id!("Cf2bnoyUYDxN5e6jAJzKXgvQpFrxhwSe8LLKjvPchPXR");

#[program]

pub mod guestbook {
   use super::*;

   pub fn init_counter (ctx:Context<InitCounter>)  -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    counter.owner = *ctx.accounts.owner.key;
    counter.count = 0;
    Ok(())
   }

   pub fn write_entry (ctx: Context<WriteEntry>, name: String, message: String)-> Result<()> {
    let entry = &mut ctx.accounts.entry;
    let counter = &mut ctx.accounts.counter;

    entry.name = name;
    entry.message = message;
    entry.author = *ctx.accounts.author.key;

    // increment counter
   counter.count += 1;
Ok(())
   }

}

#[derive(Accounts)]
pub struct InitCounter<'info>{
   #[account(mut)]
   pub owner: Signer<'info>,

   #[account(
      init,
      payer = owner,
      space = 8 + 32 + 8,
      seeds = [b"counter", owner.key.as_ref()],
      bump
   )]

   pub counter: Account<'info, EntryCounter>,

   pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WriteEntry<'info>{
   #[account(mut)]
   pub author: Signer<'info>,


   #[account(
      mut,
      seeds = [b"counter", author.key.as_ref()],
      bump
   )]
   pub counter: Account<'info, EntryCounter>,



   #[account(
      init,
      payer = author,
      space = 8 + 32 + 64 + 128,
      seeds = [b"entry", author.key.as_ref(), &counter.count.to_le_bytes()],
      bump
   )]

   pub entry: Account<'info, Entry>,

   pub system_program: Program<'info, System>,
}



#[account]
pub struct Entry {
pub author: Pubkey,
pub name: String,
pub message: String,
}

#[account]
pub struct EntryCounter {
    pub owner: Pubkey,
    pub count: u64,
}
