# lastlogin-chain-signatures

This is a fork of [google-chain-signatures](https://github.com/esaminu/google-chain-signatures), adapted to verify sessions using [LastLogin](https://lastlogin.io/).

```rust
  #[payable]
  pub fn sign_with_lastlogin_session(
      &mut self,
      proof: Proof,             // generated on your device
      public_inputs: Vec<U256>, // accompanies the proof for verification
      challenge: String,        // challenge to ensure freshness
      chain: u64,               // target chain for signature
  ) -> Promise {
    // verify proof and sign
  }
```

## Proof & Verifier

This contract needs to be accompanied by a client creating a proof, and a verifier deployed with the contract.

TBD

## How to Build Locally?

Install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
cargo near build
```

## How to Test Locally?

```bash
cargo test
```

## How to Deploy?

Deployment is automated with GitHub Actions CI/CD pipeline.
To deploy manually, install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
cargo near deploy <account-id>
```

## Useful Links

- [cargo-near](https://github.com/near/cargo-near) - NEAR smart contract development toolkit for Rust
- [near CLI](https://near.cli.rs) - Interact with NEAR blockchain from command line
- [NEAR Rust SDK Documentation](https://docs.near.org/sdk/rust/introduction)
- [NEAR Documentation](https://docs.near.org)
- [NEAR StackOverflow](https://stackoverflow.com/questions/tagged/nearprotocol)
- [NEAR Discord](https://near.chat)
- [NEAR Telegram Developers Community Group](https://t.me/neardev)
- NEAR DevHub: [Telegram](https://t.me/neardevhub), [Twitter](https://twitter.com/neardevhub)
