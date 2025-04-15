use anchor_lang::prelude::*;

// program id
declare_id!("");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program] // with anchor
pub mod favorites {
    use super::*;

    pub fn set_favorites() -> Return<()> {}
}

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}
