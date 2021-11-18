use anchor_lang::prelude::*;

declare_id!("BdbfmrSnJoatrQcUdKjYUCWj2eTxA5jacSVBKN5rpVac");

#[program]
pub mod build_a_board_solana {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;

        base_account.total_pics = 0;

        Ok(())
    }

    pub fn add_pic(ctx: Context<AddPic>, pic_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct {
            pic_link: pic_link.to_string(),
            user_address: *user.to_account_info().key,
            vote_count: 0
        };

        base_account.pic_list.push(item);
        base_account.total_pics += 1;
        
        Ok(())
    }

    pub fn upvote_pic(ctx: Context<UpvotePic>, pic_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;

        for item in base_account.pic_list.iter_mut() {
            if item.pic_link == pic_link {
                item.vote_count += 1;
                return Ok(());
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddPic<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct UpvotePic<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub pic_link: String,
    pub user_address: Pubkey,
    pub vote_count: u64,
}

#[account]
pub struct BaseAccount {
    pub total_pics: u64,
    pub pic_list: Vec<ItemStruct>,
}