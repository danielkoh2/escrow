# escrow

## Introduction

This Solana program is called an **_escrow_** - it allows a user to swap a specific amount of one token for a desired amount of another token.

For example, Alice is offering 10 USDC, and wants 100 WIF in return.

Without our program, users would have to engage in manual token swapping. Imagine the potential problems if Bob promised to send Alice 100 WIF, but instead took the 10 USDC and ran? Or what if Alice was dishonest, received the 10 USDC from Bob, and decided not to send the 100 WIF? Our Escrow program handles these complexities by acting a trusted entity that will only release tokens to both parties at the right time.

Our Escrow program is designed to provide a secure environment for users to swap a specific amount of one token with a specific amount of another token without having to trust each other.

Better yet, since our program allows Alice and Bob to transact directly with each other, they both get a hundred percent of the token they desire!

## Usage

### localnet
`anchor build`

`surfpool start`

`solana airdrop 100 -k ./key/daniel_local.json`

`anchor test --skip-local-validator`
