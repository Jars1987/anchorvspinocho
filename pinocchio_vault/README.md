# Solana Pinocchio Starter

## Steps to start (Please use wsl or linux for smooth devex)

### Clone the repo

```bash
git clone https://github.com/Jars1987/pinocchio_vault
```
### Build program

```bash
cargo build-sbf
```

- After build is successful get the program pubkey and replace with the pinocchio_pubkey::declare_id!(...)

```bash
solana address -k target/deploy/solana_pinocchio_starter-keypair.json
```

### Running Tests

```bash
cargo test --features test-default
```

### Running Benchmarks

```bash
cargo bench --features bench-default
```
