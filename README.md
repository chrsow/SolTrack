# SolTrack

A tool to find the developer of a Solana program.

## Install
```
cargo install soltrack
```

## Build from source
```
cargo build
```
The compiled binary will be at `./target/debug/soltrack`.

```bash
$ ./target/debug/soltrack -h

# SolTrack 1.0
# Wasin Sae-ngow <https://github.com/chrsow>
# Track potential developer of a Solana program

# USAGE:
#     soltrack [OPTIONS] <PROGRAM_ID>
# ...
```

## Usage
```
soltrack <PROGRAM_ID> --network <NETWORK>
```

For example, if you want to find who is the developer of the native [Memo program](https://spl.solana.com/memo) in the Solana devnet. You can run the following command.

```bash
$ soltrack MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr --network devnet

# [+] The program MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr on mainnet is deployed by: tyeraeulberg
```
As you can see, we have obtained the developer of the Memo program is "tyeraeulberg" who is one of the engineers of Solana.

## Background

At the fundamental level, Solana programs are just variantion of eEBPF programs which are compiled from the Rust compiler. By default, the programs contain paths which also contains username of the machine you used to compile it. There were efforts like [this pre-RFC](https://github.com/rust-lang/rfcs/pull/3127) to remove the username by default, but it is not implemented at the moment. The following is an example that the home directory and username are leaked in a compiled program.

```bash
$ strings your-compiled-program.so

# ...
# /<your-home-directory>/<your-username>/.cargo/registry/src/github.com-1ecc6299db9ec823/solana-program-1.7.11/src/account_info.rs
# ...
```

If you follow official documentations on Solana or a framework like [Anchor](https://github.com/project-serum/anchor) for building programs, chances are your identity is leaked in the programs.

This is a [well known issue](https://www.bleepingcomputer.com/news/security/most-loved-programming-language-rust-sparks-privacy-concerns/) for several years and it is actually fine as long as you have no reason to hide your identity. However, in case you want to stay anonymous, with the fact that your programs contain your username and Solana programs are publicly accessible by anyone on the internet, your programs might leak your identity if your username can be used to identify you (which in most cases it could).

In the meantime, to strip a username in programs, we can use `--remap-path-prefix` flag to map the default path prefix to something else.

For example, we can add the flag like the following.
```
RUSTFLAGS=--remap-path-prefix=/<your-home-directory>/<my-username>=src cargo build-bpf
```

Or if you use Anchor framework.
```
RUSTFLAGS=--remap-path-prefix=/<your-home-directory>/<my-username>=src anchor build
```

Alternatively, you can consider to build your program with a separated CI/CD machine (i.e., Github Actions which also supports private repo, etc.).