use bs58;
use clap::{App, Arg};
use regex::Regex;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::{collections::HashMap, time::Duration};

const MAX_PROGRAM_ID_LEN: usize = 44;

fn main() {
    let matches = App::new("SolTrack")
        .version("1.0")
        .author("Wasin Sae-ngow <https://github.com/chrsow>")
        .about("Track the developer of a Solana program")
        .arg(
            Arg::with_name("program_id")
                .value_name("PROGRAM_ID")
                .required(true)
                .index(1)
                .help("Program ID to find."),
        )
        .arg(
            Arg::with_name("network")
                .short("n")
                .long("network")
                .value_name("NETWORK")
                .takes_value(true)
                .required(false)
                .help(
                    "Network of the programID. \
                    [default: mainnet]",
                ),
        )
        .get_matches();

    // 0. Parse inputs
    let program_id = matches.value_of("program_id").unwrap();
    let network = matches.value_of("network").unwrap_or("mainnet");

    if program_id.len() > MAX_PROGRAM_ID_LEN {
        eprintln!("[-] Invalid ProgramID length");
        std::process::exit(1);
    }

    let pubkey_vec = bs58::decode(program_id)
        .into_vec()
        .map_err(|_| panic!("Invalid Base58 string"))
        .unwrap();
    let account_pubkey = Pubkey::new(&pubkey_vec);

    let url: &str;
    match network {
        "mainnet" => url = "https://api.mainnet-beta.solana.com",
        "devnet" => url = "https://api.devnet.solana.com",
        "testnet" => url = "https://api.testnet.solana.com",
        "localhost" => url = "http://localhost:8899",
        _ => {
            eprintln!("[-] Invalid network (only supported 'mainnet', 'devnet', 'testnet' or 'localhost')");
            std::process::exit(1);
        }
    }

    // 1. Get a program executable content from the Solana chain
    let rpc_timeout = Duration::from_secs(10);
    let rpc_client = RpcClient::new_with_timeout(url.to_string(), rpc_timeout);
    let account = rpc_client.get_account(&account_pubkey);
    let account = match account {
        Ok(_account) => _account,
        Err(error) => {
            eprintln!("[-] Couldn't get the account data: {}", error);
            std::process::exit(1);
        }
    };
    let is_executable = account.executable;

    if !is_executable {
        eprintln!("[-] The account does not contain an executable program");
        std::process::exit(1);
    }
    let data = account.data.clone();
    let binary_content = String::from_utf8_lossy(&data);

    // 2. Find a username from the binary
    let re = Regex::new(r"/(?P<username>[a-zA-Z0-9_-]*)/.cargo").unwrap();
    let caps = re.captures(&binary_content).unwrap();
    let owner: HashMap<&str, &str> = re
        .capture_names()
        .map(|o| o.and_then(|n| Some((n, caps.name(n)?.as_str()))))
        .flatten()
        .collect();

    let username = &owner["username"];
    let username_len = username.chars().count();
    if username_len > 0 {
        println!(
            "[+] The program {} on {} is deployed by: {}",
            program_id, network, username
        );
    } else {
        eprintln!("[-] Username not found");
        std::process::exit(1);
    }
}
