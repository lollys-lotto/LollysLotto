use crate::{
    error::Result,
    utils::type_conversions::{byte_array_to_u64, pubkey, u64_to_postgres_types},
    Database, TypedQuery,
};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use sqlx::{query_as, types::Decimal};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct LottoGameVaultInfoRaw {
    pub address: String,
    pub lotto_game: String,
    pub authority: String,
    pub round_num: Decimal,
    pub round_arr: Vec<u8>,
}

impl LottoGameVaultInfoRaw {
    fn select_all_by_authority(authority: &Pubkey) -> TypedQuery<Self> {
        query_as::<_, LottoGameVaultInfoRaw>(
            "SELECT * FROM lotto_game_vault_info WHERE authority = $1",
        )
        .bind(authority.to_string())
    }

    fn select_by_lotto_game(lotto_game: &Pubkey) -> TypedQuery<Self> {
        query_as::<_, LottoGameVaultInfoRaw>(
            "SELECT * FROM lotto_game_vault_info WHERE lotto_game = $1",
        )
        .bind(lotto_game.to_string())
    }

    fn select_by_round_num(round_num: Decimal) -> TypedQuery<Self> {
        query_as::<_, LottoGameVaultInfoRaw>(
            "SELECT * FROM lotto_game_vault_info WHERE round_num = $1",
        )
        .bind(round_num)
    }

    fn insert(
        address: String,
        lotto_game: String,
        authority: String,
        round_num: Decimal,
        round_arr: Vec<u8>,
    ) -> TypedQuery<Self> {
        query_as::<_, LottoGameVaultInfoRaw>(
            r#"INSERT INTO lotto_game_vault_info
            (address, lotto_game, authority, round_num, round_arr)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING address, lotto_game, authority, round_num"#,
        )
        .bind(address)
        .bind(lotto_game)
        .bind(authority)
        .bind(round_num)
        .bind(round_arr)
    }
}

impl Into<LottoGameVaultInfo> for LottoGameVaultInfoRaw {
    fn into(self) -> LottoGameVaultInfo {
        let round = byte_array_to_u64(&self.round_arr).unwrap();
        LottoGameVaultInfo {
            address: Pubkey::from_str(&self.address).unwrap(),
            lotto_game: Pubkey::from_str(&self.lotto_game).unwrap(),
            authority: Pubkey::from_str(&self.authority).unwrap(),
            round,
        }
    }
}

/// Type-converted for ease-of-use

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LottoGameVaultInfo {
    #[serde(with = "pubkey")]
    pub address: Pubkey,
    #[serde(with = "pubkey")]
    pub lotto_game: Pubkey,
    #[serde(with = "pubkey")]
    pub authority: Pubkey,
    pub round: u64,
}

impl Database {
    pub async fn select_lotto_game_vault_info_by_lotto_game(
        &self,
        lotto_game: &Pubkey,
    ) -> Result<LottoGameVaultInfo> {
        self.fetch_one(LottoGameVaultInfoRaw::select_by_lotto_game(lotto_game))
            .await
    }

    pub async fn select_lotto_game_vault_info_by_authority(
        &self,
        authority: &Pubkey,
    ) -> Result<Vec<LottoGameVaultInfo>> {
        self.fetch_all(LottoGameVaultInfoRaw::select_all_by_authority(authority))
            .await
    }

    pub async fn select_lotto_game_vault_info_by_round_num(
        &self,
        round: u64,
    ) -> Result<LottoGameVaultInfo> {
        let (round_num, _round_arr) = u64_to_postgres_types(round);
        self.fetch_one(LottoGameVaultInfoRaw::select_by_round_num(round_num))
            .await
    }

    pub async fn insert_lotto_game_vault_info(
        &self,
        address: &Pubkey,
        lotto_game: &Pubkey,
        authority: &Pubkey,
        round: u64,
    ) -> Result<LottoGameVaultInfo> {
        let (round_num, round_arr) = u64_to_postgres_types(round);
        self.fetch_one(LottoGameVaultInfoRaw::insert(
            address.to_string(),
            lotto_game.to_string(),
            authority.to_string(),
            round_num,
            round_arr,
        ))
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_helpers::{connect_to_test_db, random_pubkey};

    #[tokio::test]
    async fn test_lotto_game_vault_info() {
        let db = connect_to_test_db().await;

        let address1 = random_pubkey();
        let lotto_game1 = random_pubkey();
        let authority1 = random_pubkey();
        let round_num1 = 1;

        let _info1 = db
            .insert_lotto_game_vault_info(&address1, &lotto_game1, &authority1, round_num1)
            .await
            .unwrap();

        let address2 = random_pubkey();
        let lotto_game2 = random_pubkey();
        let round_num2 = 2;

        let _info2 = db
            .insert_lotto_game_vault_info(&address2, &lotto_game2, &authority1, round_num2)
            .await
            .unwrap();

        let result1 = db
            .select_lotto_game_vault_info_by_authority(&authority1)
            .await
            .unwrap();

        assert!(result1.contains(&_info1));
        assert!(result1.contains(&_info2));

        let result2 = db
            .select_lotto_game_vault_info_by_round_num(round_num1)
            .await
            .unwrap();
        assert!(result2 == _info1);

        let result3 = db
            .select_lotto_game_vault_info_by_lotto_game(&lotto_game2)
            .await
            .unwrap();
        assert!(result3 == _info2);
    }
}
