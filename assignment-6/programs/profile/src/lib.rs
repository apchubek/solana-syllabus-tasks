use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWqk1zXg7Z4jzq7N4Qx6gsJx");

pub const MAX_HANDLE: usize = 32;
pub const MAX_NAME: usize = 64;
pub const MAX_BIO: usize = 160;

#[program]
pub mod profile {
    use super::*;

    pub fn initialize_profile(
        ctx: Context<InitializeProfile>,
        handle: String,
        name: String,
        bio: String,
    ) -> Result<()> {
        validate_profile_fields(&handle, &name, &bio)?;

        let profile = &mut ctx.accounts.profile;
        profile.authority = ctx.accounts.authority.key();
        profile.bump = ctx.bumps.profile;
        profile.handle = handle;
        profile.name = name;
        profile.bio = bio;

        Ok(())
    }

    pub fn update_profile(
        ctx: Context<UpdateProfile>,
        handle: String,
        name: String,
        bio: String,
    ) -> Result<()> {
        validate_profile_fields(&handle, &name, &bio)?;

        let profile = &mut ctx.accounts.profile;
        profile.handle = handle;
        profile.name = name;
        profile.bio = bio;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeProfile<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = ProfileAccount::SPACE,
        seeds = [b"profile", authority.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, ProfileAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
        seeds = [b"profile", authority.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, ProfileAccount>,
}

#[account]
pub struct ProfileAccount {
    pub authority: Pubkey,
    pub bump: u8,
    pub handle: String,
    pub name: String,
    pub bio: String,
}

impl ProfileAccount {
    pub const SPACE: usize = 8 + 32 + 1 + 4 + MAX_HANDLE + 4 + MAX_NAME + 4 + MAX_BIO;
}

fn validate_profile_fields(handle: &str, name: &str, bio: &str) -> Result<()> {
    validate_handle(handle)?;
    validate_name(name)?;
    validate_bio(bio)?;
    Ok(())
}

fn validate_handle(handle: &str) -> Result<()> {
    let trimmed = handle.trim();

    require!(!trimmed.is_empty(), ProfileError::HandleEmpty);
    require!(trimmed.len() <= MAX_HANDLE, ProfileError::HandleTooLong);
    require!(trimmed == handle, ProfileError::HandleContainsWhitespace);
    require!(
        trimmed
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '_' || character == '-'),
        ProfileError::HandleInvalidCharacters
    );

    Ok(())
}

fn validate_name(name: &str) -> Result<()> {
    require!(!name.trim().is_empty(), ProfileError::NameEmpty);
    require!(name.len() <= MAX_NAME, ProfileError::NameTooLong);
    Ok(())
}

fn validate_bio(bio: &str) -> Result<()> {
    require!(bio.len() <= MAX_BIO, ProfileError::BioTooLong);
    Ok(())
}

#[error_code]
pub enum ProfileError {
    #[msg("handle is empty")]
    HandleEmpty,
    #[msg("handle is too long")]
    HandleTooLong,
    #[msg("handle contains whitespace")]
    HandleContainsWhitespace,
    #[msg("handle contains invalid characters")]
    HandleInvalidCharacters,
    #[msg("name is empty")]
    NameEmpty,
    #[msg("name is too long")]
    NameTooLong,
    #[msg("bio is too long")]
    BioTooLong,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_account_space_matches_string_layout() {
        assert_eq!(ProfileAccount::SPACE, 8 + 32 + 1 + 4 + MAX_HANDLE + 4 + MAX_NAME + 4 + MAX_BIO);
    }

    #[test]
    fn accepts_valid_profile_fields() {
        assert!(validate_profile_fields("builder-1", "Web3 Builder", "Hello from Solana").is_ok());
    }

    #[test]
    fn rejects_empty_handle() {
        assert!(validate_profile_fields("", "Web3 Builder", "Hello from Solana").is_err());
    }

    #[test]
    fn rejects_handle_with_spaces() {
        assert!(validate_profile_fields("bad handle", "Web3 Builder", "Hello from Solana").is_err());
    }

    #[test]
    fn rejects_empty_name() {
        assert!(validate_profile_fields("builder", "   ", "Hello from Solana").is_err());
    }

    #[test]
    fn rejects_too_long_bio() {
        let bio = "x".repeat(MAX_BIO + 1);
        assert!(validate_profile_fields("builder", "Web3 Builder", &bio).is_err());
    }
}
