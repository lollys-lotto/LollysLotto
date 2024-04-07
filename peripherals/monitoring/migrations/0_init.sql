CREATE TYPE instruction_type AS ENUM (
  'process_winning_numbers',
  'request_winning_numbers',
  'test_emit_winning_numbers',
  'burn_lolly',
  'buy_lotto_ticket',
  'claim_user_rewards',
  'close_event_emitter',
  'close_lollys_lotto',
  'close_lotto_game',
  'crank_lotto_game_winner',
  'create_event_emitter',
  'create_lolly_burn_state',
  'create_lollys_lotto',
  'create_user_metadata',
  'start_lotto_game',
  'swap_usdc_lolly'
);

CREATE TABLE IF NOT EXISTS program_events (
   id BIGSERIAL PRIMARY KEY,
   transaction_signature VARCHAR(90) NOT NULL,
   seq_num int8 NOT NULL,
   version SMALLINT NOT NULL,
   instruction_type instruction_type NOT NULL,
   data VARCHAR(1500) NOT NULL,
   slot int8 NOT NULL,
   block_time timestamp NOT NULL,
   error_program VARCHAR(46),
   error_message VARCHAR(120),
   UNIQUE (seq_num, transaction_signature)
);

CREATE TABLE IF NOT EXISTS liquidity_account_info (
   address VARCHAR(46) PRIMARY KEY,
   owner VARCHAR(46),
   mint VARCHAR(46),
   pool_registry VARCHAR(46),
   created_at timestamp,
   creation_event_id int8 REFERENCES program_events(id),
   close_event_id int8 REFERENCES program_events(id)
);

CREATE TABLE IF NOT EXISTS pool_vault_info (
   address VARCHAR(46) PRIMARY KEY,
   pool_registry VARCHAR(46) NOT NULL,
   main_mint VARCHAR(46) NOT NULL,
   secondary_mint VARCHAR(46),
   UNIQUE (pool_registry, main_mint, secondary_mint)
);

CREATE TABLE IF NOT EXISTS fee_vault_info (
   address VARCHAR(46) PRIMARY KEY,
   pool_registry VARCHAR(46),
   mint VARCHAR(46),
   UNIQUE (pool_registry, mint)
);

CREATE TABLE IF NOT EXISTS pair_info (
   address VARCHAR(46) PRIMARY KEY,
   pool_registry VARCHAR(46) NOT NULL,
   mint_a VARCHAR(46) NOT NULL,
   mint_b VARCHAR(46) NOT NULL
);

CREATE TABLE IF NOT EXISTS oracle_price_history_info (
   address VARCHAR(46) PRIMARY KEY,
   pool_registry VARCHAR(46) NOT NULL,
   oracle VARCHAR(46) NOT NULL,
   oracle_type oracle_type NOT NULL,
   mint VARCHAR(46) NOT NULL
);

CREATE TABLE IF NOT EXISTS liquidity_account_activity (
   event_id int8 PRIMARY KEY REFERENCES program_events(id) NOT NULL,
   liquidity_account VARCHAR(46) NOT NULL,
   amount_deposited_num NUMERIC NOT NULL,
   amount_deposited_arr BYTEA NOT NULL,
   CHECK (octet_length(amount_deposited_arr) = 8),
   last_observed_tap_num NUMERIC NOT NULL,
   last_observed_tap_arr BYTEA NOT NULL,
   CHECK (octet_length(last_observed_tap_arr) = 8),
   total_earned_num NUMERIC NOT NULL,
   total_earned_arr BYTEA NOT NULL,
   CHECK (octet_length(total_earned_arr) = 8)
);

CREATE TABLE IF NOT EXISTS pool_vault_activity (
   event_id int8 REFERENCES program_events(id) NOT NULL,
   pool_vault VARCHAR(46) REFERENCES pool_vault_info(address) NOT NULL,
   balance_before_num NUMERIC NOT NULL,
   balance_before_arr BYTEA NOT NULL,
   CHECK (octet_length(balance_before_arr) = 8),
   balance_after_num NUMERIC NOT NULL,
   balance_after_arr BYTEA NOT NULL,
   CHECK (octet_length(balance_after_arr) = 8),
   PRIMARY KEY (event_id, pool_vault)
);

CREATE TABLE IF NOT EXISTS lp_deposit_activity (
   event_id int8 PRIMARY KEY REFERENCES program_events(id) NOT NULL,
   pool_registry VARCHAR(46) NOT NULL,
   mint VARCHAR(46) NOT NULL,
   total_liquidity_deposited_num NUMERIC NOT NULL,
   total_liquidity_deposited_arr BYTEA NOT NULL,
   CHECK (octet_length(total_liquidity_deposited_arr) = 8)
);

CREATE TABLE IF NOT EXISTS fee_vault_activity (
   event_id int8 PRIMARY KEY REFERENCES program_events(id) NOT NULL,
   fee_vault VARCHAR(46) REFERENCES fee_vault_info(address) NOT NULL,
   balance_before_num NUMERIC NOT NULL,
   balance_before_arr BYTEA NOT NULL,
   CHECK (octet_length(balance_before_arr) = 8),
   balance_after_num NUMERIC NOT NULL,
   balance_after_arr BYTEA NOT NULL,
   CHECK (octet_length(balance_after_arr) = 8)
);

CREATE TABLE IF NOT EXISTS swap_activity (
   event_id int8 PRIMARY KEY REFERENCES program_events(id) NOT NULL,
   pair VARCHAR(46) REFERENCES pair_info(address) NOT NULL,
   "user" VARCHAR(46) NOT NULL,
   mint_in VARCHAR(46) NOT NULL,
   mint_out VARCHAR(46) NOT NULL,
   amount_in_num NUMERIC NOT NULL,
   amount_in_arr BYTEA NOT NULL,
   CHECK (octet_length(amount_in_arr) = 8),
   preswap_pool_token_ratio FLOAT8 NOT NULL,
   initially_below_max_pool_token_ratio BOOL NOT NULL,
   can_take_preferred_route BOOL NOT NULL,
   violates_pool_token_ratio_constraints BOOL NOT NULL,
   optimal_price FLOAT8 NOT NULL,
   panic_price FLOAT8 NOT NULL,
   amount_out_optimal_num NUMERIC NOT NULL,
   amount_out_optimal_arr BYTEA NOT NULL,
   CHECK (octet_length(amount_out_optimal_arr) = 8),
   fees_out_optimal_num NUMERIC NOT NULL,
   fees_out_optimal_arr BYTEA NOT NULL,
   CHECK (octet_length(fees_out_optimal_arr) = 8),
   amount_out_panic_num NUMERIC NOT NULL,
   amount_out_panic_arr BYTEA NOT NULL,
   CHECK (octet_length(amount_out_panic_arr) = 8),
   fees_out_panic_num NUMERIC NOT NULL,
   fees_out_panic_arr BYTEA NOT NULL,
   CHECK (octet_length(fees_out_panic_arr) = 8),
   total_accumulated_lp_rewards_num NUMERIC NOT NULL,
   total_accumulated_lp_rewards_arr BYTEA NOT NULL,
   CHECK (octet_length(total_accumulated_lp_rewards_arr) = 8)
);

CREATE TABLE IF NOT EXISTS pair_activity (
   event_id int8 PRIMARY KEY REFERENCES program_events(id) NOT NULL,
   pair VARCHAR(46) REFERENCES pair_info(address) NOT NULL,
   fees_generated_is_mint_a BOOL NOT NULL,
   total_fees_generated_num NUMERIC NOT NULL,
   total_fees_generated_arr BYTEA NOT NULL,
   CHECK (octet_length(total_fees_generated_arr) = 16),
   total_historical_volume_num NUMERIC NOT NULL,
   total_historical_volume_arr BYTEA NOT NULL,
   CHECK (octet_length(total_historical_volume_arr) = 16)
);

CREATE TABLE IF NOT EXISTS internal_swap_activity (
   event_id int8 PRIMARY KEY REFERENCES program_events(id) NOT NULL,
   pair VARCHAR(46) REFERENCES pair_info(address) NOT NULL,
   token_a_price FLOAT8 NOT NULL,
   token_b_price FLOAT8 NOT NULL,
   ssl_a_internally_swapped_volume_num NUMERIC NOT NULL,
   ssl_a_internally_swapped_volume_arr BYTEA NOT NULL,
   CHECK (octet_length(ssl_a_internally_swapped_volume_arr) = 16),
   ssl_b_internally_swapped_volume_num NUMERIC NOT NULL,
   ssl_b_internally_swapped_volume_arr BYTEA NOT NULL,
   CHECK (octet_length(ssl_b_internally_swapped_volume_arr) = 16)
);

CREATE TABLE IF NOT EXISTS oracle_prices (
   oracle_price_history VARCHAR(46) REFERENCES oracle_price_history_info(address) NOT NULL,
   price_num int8 NOT NULL,
   price_scale int8 NOT NULL,
   slot int8 NOT NULL,
   PRIMARY KEY (oracle_price_history, slot)
);
