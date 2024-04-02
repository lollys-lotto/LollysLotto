use lollys_lotto::state::UserMetadata;
use solana_devtools_localnet::GeneratedAccount;
use anchor_lang::prelude::Pubkey;
use crate::TestUser;

pub struct TestUserMetadata;
impl GeneratedAccount for TestUserMetadata {
    type Data = UserMetadata;

    fn address(&self) -> Pubkey {
        UserMetadata::address(TestUser.address())
    }

    fn generate(&self) -> Self::Data {
        let (_, bump) = UserMetadata::address_with_bump(TestUser.address());
        UserMetadata {
            bump,
            user: TestUser.address(),
            created_timestamp: 0,
            tier: lollys_lotto::state::UserTier::Bronze,
            total_tickets_purchased: 0,
            total_amount_won: 0,
            total_amount_claimed: 0,
            last_claimed_at: 0,
            referral_count: 0,
            referral_revenue: 0,
            claim_tickets: [lollys_lotto::state::ClaimTicket::default(); lollys_lotto::state::USER_CLAIM_TICKET_CAPACITY],
        }
    }

    fn owner(&self) -> Pubkey {
        lollys_lotto::ID
    }
}
