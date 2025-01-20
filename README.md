# Solana Token Transfer Program

A secure and efficient Solana program for transferring SPL Token-2022 tokens between accounts.

## Overview

This program implements a token transfer mechanism using the Token-2022 program on Solana blockchain. It enables secure transfers of SPL tokens between token accounts while maintaining all necessary checks and balances.

## Features

- Supports Token-2022 standard
- Implements checked transfer operations
- Handles decimal precision automatically
- Built with Anchor framework
- Complete account validation

## Technical Details

### Program ID
```6GmLK8Qz9hBnCa89jcaVoX4b8nKCYc62p4hUbZ5XrsBL```

### Account Structure
The program requires the following accounts for execution:
- `from`: Signer account initiating the transfer
- `from_ata`: Source token account
- `to_ata`: Destination token account
- `mint`: Token mint account
- `token_program`: Token-2022 program

### Instructions
- `transfer_token2022`: Transfers specified amount of tokens between accounts

## Development

### Prerequisites
- Solana Tool Suite
- Anchor Framework
- Rust

### Building
```bash
anchor build
```

### Testing
```
anchor test
```

### Security
The program implements several security measures:

- Checked transfers to prevent decimal precision errors
- Signer verification
- Mutable account constraints
- Token account ownership validation

### License
MIT