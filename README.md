# Solana Transaction Parser

A Rust-based transaction parser for Solana that specializes in decoding and analyzing transactions from various DeFi protocols, including Pump AMM and Pump Fun.

## Project Structure

The project is organized as a workspace with multiple crates:

```
.
├── crates/
│   ├── parser-core/        # Core parsing functionality
│   ├── parser-pump-amm/    # Pump AMM protocol parser
│   ├── parser-pumpfun/     # Pump Fun protocol parser
│   ├── parser-raydium/     # Raydium protocol parser
│   ├── types/             # Shared type definitions
│   ├── utils/             # Common utility functions
│   ├── instruction-parser/ # Base instruction parsing
│   ├── proto-gen/         # Generated protobuf code
│   └── grpc-server/       # gRPC server implementation
├── src/                   # Main application code
└── proto/                 # Protocol buffer definitions
```

## Features

- Real-time transaction monitoring via gRPC
- Support for multiple DeFi protocols:
  - Pump AMM
  - Pump Fun
  - Raydium (planned)
- Transaction decoding and event extraction
- Balance change tracking
- Automatic reconnection with exponential backoff

## Prerequisites

- Rust (latest stable version)
- Protocol Buffers compiler
- Solana CLI tools (optional)

## Setup

1. Clone the repository:

   ```bash
   git clone <repository-url>
   cd actual-rust-parser
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. Run the parser:
   ```bash
   cargo run -- --endpoint <gRPC-endpoint> --x-token <your-token>
   ```

## Configuration

The parser can be configured through command-line arguments:

- `--endpoint`: gRPC endpoint URL
- `--x-token`: Authentication token for the gRPC service

## Protocol Support

### Pump AMM

- Program ID: `pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA`
- Supported instructions:
  - Buy
  - Sell
  - Create Pool

### Pump Fun

- Program ID: `6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P`
- Supported instructions:
  - Buy
  - Sell
  - Create Pool

## Development

### Adding a New Protocol Parser

1. Create a new crate in the `crates` directory
2. Implement the `InstructionParser` trait
3. Add the parser to `TransactionParserNew` in `parser-core`
4. Update the program ID constants in `main.rs`

### Running Tests

```bash
cargo test
```
