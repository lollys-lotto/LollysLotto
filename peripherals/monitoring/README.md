## Program Event Monitor
This folder hosts the crates responsible for monitoring the LollysLotto program.
- `program_monitor_db` -- A `lib` crate that defines a Postgres database. This database schema is also maintained as pure SQL in the `migrations` folder.
- `program_event_monitor` -- A `bin` crate with several entrypoints, each of which perform some aspect of the monitoring:
  - `event_monitor` -- Watches on a Solana RPC Websocket `logs_subscribe` for Anchor `Event` data on the LollysLotto program, and inserts the data into a Postgres instance.
  - `backfiller` -- Backfills the data in a Postgres instance by combing for transactions on the LollysLotto program, and using recorded `event_id` values to spot any gaps in the data.

## Setup and Testing
### Setup
- Rust, Cargo -- These crates have the usual requirements: `rust`, `cargo`
- `cargo-make` Task Runner -- `cargo install cargo-make`.
- Docker, Docker Compose -- We use a dockerized Postgres instance for local development, so you'll need `docker` and `docker-compose`.
- SQLx CLI -- We use this CLI to perform local DB migrations. `cargo install sqlx-cli`.

### The `env.local` File
There is an `env.local` file in the same directory as this README.
The values inside it are preconfigured to "just work" for local development.
They're pulled into the `cargo make` commands automatically (the default profile in `cargo-make` is "development".

### Dockerized Postgres
Start up a Dockerized local Postgres instance like so:
```
docker-compose up
```

After spinning up the `docker-compose`, you'll need to do a one-time setup by executing the migrations.
This is accomplished using a `sqlx-cli` command, which has a `cargo-make` command:
```
cargo make setup_db
```

### Testing with `cargo test`
After you've performed the setup, you should be able to run the tests:
```
cargo make test_db
```

Note: this `cargo-make` task will drop the current local database and rebuild it!

### Localnet Testing
The localnet test suites produce transactions on localnet which generate Anchor events.
This means you can locally test the program monitor binary, and index those Anchor events to the local DB.

- In one terminal run your desired localnet test suite.
For example, you might run the swap tests (from the project root, up two levels from this README):
```commandline
cargo make test_swaps
```

- In another terminal, run the program monitor binary. 
From this current directory, that would look like:
```commandline
../../target/debug/lollys_lotto_event_monitor
```
Notice that we are using the workspace's already-compiled debug binary, rather than re-running
a "build and run" command, because Cargo will be locked up and busy running the localnet tests in the first terminal.

Note also that the command will probably fail to connect to localnet until the localnet has finished starting up.
This may take a few seconds, you can keep up-arrowing until it works.
