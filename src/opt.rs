use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Metaboss", about = "Metaplex NFT 'Swiss Army Knife' tool.")]
pub struct Opt {
    /// RPC endpoint url to override using the Solana config or the hard-coded default
    #[structopt(short, long, global = true)]
    pub rpc: Option<String>,

    /// Timeout to override default value of 90 seconds
    #[structopt(short = "T", long, global = true, default_value = "90")]
    pub timeout: u64,

    /// Log level
    #[structopt(short, long, global = true, default_value = "warn")]
    pub log_level: String,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// NFT collections commands
    #[structopt(name = "collections")]
    Collections {
        #[structopt(subcommand)]
        collections_subcommands: CollectionsSubcommands,
    },
    /// NFT uses commands
    #[structopt(name = "uses")]
    Uses {
        #[structopt(subcommand)]
        uses_subcommands: UsesSubcommands,
    },
    /// Burn token
    #[structopt(name = "burn")]
    Burn {
        #[structopt(subcommand)]
        burn_subcommands: BurnSubcommands,
    },
    /// Decode on-chain data into JSON format
    #[structopt(name = "decode")]
    Decode {
        #[structopt(subcommand)]
        decode_subcommands: DecodeSubcommands,
    },
    /// Derive PDAs for various account types
    Derive {
        #[structopt(subcommand)]
        derive_subcommands: DeriveSubcommands,
    },
    /// Find things.
    #[structopt(name = "find")]
    Find {
        #[structopt(subcommand)]
        find_subcommands: FindSubcommands,
    },
    /// Mint new NFTs from JSON files
    #[structopt(name = "mint")]
    Mint {
        #[structopt(subcommand)]
        mint_subcommands: MintSubcommands,
    },
    /// Update various aspects of NFTs
    #[structopt(name = "update")]
    Update {
        #[structopt(subcommand)]
        update_subcommands: UpdateSubcommands,
    },
    /// Set non-Data struct values for a NFT
    #[structopt(name = "set")]
    Set {
        #[structopt(subcommand)]
        set_subcommands: SetSubcommands,
    },
    /// Sign metadata for an unverified creator
    #[structopt(name = "sign")]
    Sign {
        #[structopt(subcommand)]
        sign_subcommands: SignSubcommands,
    },
    /// Get snapshots of various blockchain states
    #[structopt(name = "snapshot")]
    Snapshot {
        #[structopt(subcommand)]
        snapshot_subcommands: SnapshotSubcommands,
    },
    /// Withdraw
    Withdraw {
        #[structopt(subcommand)]
        withdraw_subcommands: WithdrawSubcommands,
    },
}

#[derive(Debug, StructOpt)]
pub enum BurnSubcommands {
    /// Burn one NFT.
    #[structopt(name = "one")]
    One {
        /// Path to the authority & funder keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// Token mint account
        #[structopt(short, long)]
        account: String,
    },
}

#[derive(Debug, StructOpt)]
pub enum UsesSubcommands {
    /// Approve a delegate authority that is allowed to make changes to the NFT's Use data.
    #[structopt(name = "approve-authority")]
    ApproveAuthority {
        /// Path to the update authority keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// NFT mint address
        #[structopt(short, long)]
        mint_nft: String,

        /// Delegate use authority address
        #[structopt(short, long)]
        delegate_use_authority: String,

        /// NFT Owner Token Account
        #[structopt(short, long)]
        token_account_nft: String,

        /// Burner Program ID
        #[structopt(short, long)]
        burner_program_id: String,

        /// Number of uses
        #[structopt(short, long)]
        number_of_uses: u64,
    },
    /// Revoke a delegate authority from being allowed to make changes to the NFT's Use data.
    #[structopt(name = "revoke-authority")]
    RevokeAuthority {
        /// Path to the update authority keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// NFT mint address
        #[structopt(short, long)]
        mint_nft: String,

        /// Delegate use authority address
        #[structopt(short, long)]
        delegate_use_authority: String,

        /// NFT Owner Token Account
        #[structopt(short, long)]
        token_account_nft: String,
    },
    /// Use a NFT, following the on-chain logic for burning it if set.
    #[structopt(name = "utilize")]
    Utilize {
        /// Path to the use authority keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// NFT mint address
        #[structopt(short, long)]
        mint_nft: String,

        /// NFT Holder/Owner address
        #[structopt(short, long)]
        holder_nft: String,

        /// NFT Owner Token Account
        #[structopt(short, long)]
        token_account_nft: String,

        /// Optional Burner Program ID (if token use is set to Burn)
        #[structopt(short, long)]
        burner_program_id: Option<String>,

        /// Option if the signing keypair is a use authority delegate.
        #[structopt(short = "d", long)]
        is_delegate: bool,
    },
}

#[derive(Debug, StructOpt)]
pub enum CollectionsSubcommands {
    /// Verify collection on an NFT.
    #[structopt(name = "verify")]
    VerifyCollection {
        /// Path to the update authority keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// Collection mint address
        #[structopt(short, long)]
        collection_mint: String,

        /// NFT mint address
        #[structopt(short = "n", long)]
        nft_mint: String,

        /// Option if the signing keypair is a collection authority delegate.
        #[structopt(short = "d", long)]
        is_delegate: bool,
    },
    /// Set an NFT's collection as unverified.
    #[structopt(name = "unverify")]
    UnverifyCollection {
        /// Path to the update authority keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// Collection mint address
        #[structopt(short, long)]
        collection_mint: String,

        /// NFT mint address
        #[structopt(short = "n", long)]
        nft_mint: String,

        /// Option if the signing keypair is a collection authority delegate.
        #[structopt(short = "d", long)]
        is_delegate: bool,
    },
    /// Set collection value on NFT and verify in the same step.
    #[structopt(name = "set-and-verify")]
    SetAndVerifyCollection {
        /// Path to the collection update authority keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// Collection mint address
        #[structopt(short, long)]
        collection_mint: String,

        /// NFT mint address
        #[structopt(short = "n", long)]
        nft_mint: String,

        /// NFT update authority
        #[structopt(short, long)]
        update_authority_nft: String,

        /// Option if the signing keypair is a collection authority delegate.
        #[structopt(short = "d", long)]
        is_delegate: bool,
    },
    /// Approve a delegate authority that is allowed to change collection data on the NFT.
    #[structopt(name = "approve-authority")]
    ApproveAuthority {
        /// Path to the update authority keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// Collection mint address
        #[structopt(short, long)]
        collection_mint: String,

        /// Delegate authority address
        #[structopt(short, long)]
        delegate_authority: String,
    },
    /// Revoke the delegate authority from being allowed to change collection data on the NFT.
    #[structopt(name = "revoke-authority")]
    RevokeAuthority {
        /// Path to the update authority keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// Collection mint address
        #[structopt(short, long)]
        collection_mint: String,

        /// Delegate authority address
        #[structopt(short, long)]
        delegate_authority: String,
    },
}

#[derive(Debug, StructOpt)]
pub enum DecodeSubcommands {
    /// Decode a mint account's metadata
    #[structopt(name = "mint")]
    Mint {
        /// Single mint account to decode
        #[structopt(short, long)]
        account: Option<String>,

        /// Use this to write the full metadata struct to the output
        #[structopt(long)]
        full: bool,

        /// Path to JSON file containing a list of mint accounts to decode
        #[structopt(short = "L", long)]
        list_file: Option<String>,

        /// Decode into raw bytes
        #[structopt(long)]
        raw: bool,

        /// Path to directory to save output files.
        #[structopt(short, long, default_value = ".")]
        output: String,
    },
    /// Decode a mint account's master edition
    Master {
        #[structopt(short, long)]
        account: String,
    },
}

#[derive(Debug, StructOpt)]
pub enum DeriveSubcommands {
    /// Derive generic PDA from seeds and program id
    #[structopt(name = "pda")]
    Pda {
        /// Seeds to derive PDA from
        seeds: String,
        /// Program id to derive PDA from
        program_id: String,
    },
    /// Derive Metadata PDA
    #[structopt(name = "metadata")]
    Metadata { mint_account: String },
    /// Derive Edition PDA
    #[structopt(name = "edition")]
    Edition { mint_account: String },
    /// Derive CMV2 PDA
    #[structopt(name = "cmv2-creator")]
    CMV2Creator { candy_machine_id: String },
}

#[derive(Debug, StructOpt)]
pub enum FindSubcommands {
    /// Find any missing editions for a Master NFT mint account.
    #[structopt(name = "missing-editions")]
    MissingEditions {
        #[structopt(short, long)]
        account: String,
    },
}

#[derive(Debug, StructOpt)]
pub enum MintSubcommands {
    /// Mint a single NFT from a JSON file
    #[structopt(name = "one")]
    One {
        /// Path to the update_authority keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// Receiving address, if different from update authority.
        #[structopt(short = "R", long)]
        receiver: Option<String>,

        /// On-chain formatted metadata for the new NFT
        #[structopt(short = "d", long)]
        nft_data_file: Option<String>,

        /// Link to external metadata to use to create the NFT
        #[structopt(short = "u", long)]
        external_metadata_uri: Option<String>,

        /// Mint the NFT with immutable data fields
        #[structopt(short, long)]
        immutable: bool,

        /// Mint the NFT with primary_sale_happened set to true
        #[structopt(short, long)]
        primary_sale_happened: bool,

        /// Maximum number of editions. Defaults to zero, meaning no editions allowed.
        #[structopt(short = "e", long, default_value = "0")]
        max_editions: u64,

        /// Sign NFT after minting it
        #[structopt(long)]
        sign: bool,
    },
    /// Mint one or more editions from a Master NFT.
    #[structopt(name = "editions")]
    Editions {
        /// Path to the update_authority keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// Master Edition NFT mint account/token address.
        #[structopt(short, long)]
        account: String,

        /// Receiving address, if different from update authority.
        #[structopt(short = "R", long)]
        receiver: Option<String>,

        /// Mint the next n editions in order.
        #[structopt(short, long)]
        next_editions: Option<u64>,

        /// Mint the provided specific editions e.g.: --specific-editions 1,7,10
        #[structopt(short = "s", long)]
        specific_editions: Option<Vec<u64>>,
    },
    /// Find any missing editions for a Master NFT.
    #[structopt(name = "missing-editions")]
    MissingEditions {
        /// Path to the update_authority keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        #[structopt(short, long)]
        account: String,
    },
    #[structopt(name = "list")]
    /// Mint a list of NFTs from a directory of JSON files
    List {
        /// Path to the update_authority keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// Receiving address, if different from update authority
        #[structopt(short = "R", long)]
        receiver: Option<String>,

        /// Directory of on-chain formatted metadata files for the new NFTs
        #[structopt(short = "d", long)]
        nft_data_dir: Option<String>,

        /// List of external metadata links to use to create the NFTs
        #[structopt(short = "u", long)]
        external_metadata_uris: Option<String>,

        /// Mint the NFTs with immutable data fields
        #[structopt(short, long)]
        immutable: bool,

        /// Mint the NFTs with primary_sale_happened set to true
        #[structopt(short, long)]
        primary_sale_happened: bool,

        /// Sign NFTs after minting them
        #[structopt(long)]
        sign: bool,

        /// Track whether URIs are succesfully minted or not, and output to
        /// `minted` file with URIs -> Mint Accounts or to `unminted` file
        /// with list of unminted URIs for easy continuation of command
        #[structopt(long)]
        track: bool,
    },
}

#[derive(Debug, StructOpt)]
pub enum SetSubcommands {
    /// Set primary sale happened to true
    #[structopt(name = "primary-sale-happened")]
    PrimarySaleHappened {
        /// Path to the creator's keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// Mint account of corresponding metadata to update
        #[structopt(short, long)]
        account: String,
    },
    /// Set update authority to a new account
    #[structopt(name = "update-authority")]
    UpdateAuthority {
        /// Path to the creator's keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// Mint account of corresponding metadata to update
        #[structopt(short, long)]
        account: String,

        /// New update authority address
        #[structopt(short = "u", long)]
        new_update_authority: String,

        //Path to the payers's keypair file
        #[structopt(short = "p", long)]
        keypair_payer: Option<String>,
    },
    /// Set update authority on multiple accounts to a new account
    #[structopt(name = "update-authority-all")]
    UpdateAuthorityAll {
        /// Path to the creator's keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// Path to JSON mint accounts file
        #[structopt(short = "a", long)]
        mint_accounts_file: String,

        /// New update authority address
        #[structopt(short = "u", long)]
        new_update_authority: String,

        //Path to the payers's keypair file
        #[structopt(short = "p", long)]
        keypair_payer: Option<String>,
    },
    /// Set is-mutable to false, preventing any future updates to the NFT
    #[structopt(name = "immutable")]
    Immutable {
        /// Path to the creator's keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// Mint account of corresponding metadata to update
        #[structopt(short, long)]
        account: String,
    },
    ImmutableAll {
        /// Path to the creator's keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// Path to JSON mint accounts file
        #[structopt(short, long)]
        mint_accounts_file: String,
    },
}

#[derive(Debug, StructOpt)]
pub enum SignSubcommands {
    /// Sign the metadata for a single mint account
    #[structopt(name = "one")]
    One {
        /// Path to the creator's keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// Mint account to sign
        #[structopt(short, long)]
        account: String,
    },
    /// Sign all metadata from a JSON list or for a given candy machine id / creator
    #[structopt(name = "all")]
    All {
        /// Path to the creator's keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// Creator to filter accounts by (for CM v2 use --v2 if candy_machine account is passed)
        #[structopt(short, long)]
        creator: Option<String>,

        /// CM creator index to filter by
        #[structopt(short, long, default_value = "0")]
        position: usize,

        /// Candy machine v2 id
        #[structopt(long = "v2")]
        v2: bool,

        /// Path to JSON file with list of mint accounts to sign
        #[structopt(short, long)]
        mint_accounts_file: Option<String>,
    },
}

#[derive(Debug, StructOpt)]
pub enum SnapshotSubcommands {
    /// Snapshot all current holders of NFTs by candy_machine_id / creator or update_authority
    #[structopt(name = "holders")]
    Holders {
        /// Update authority to filter accounts by.
        #[structopt(short, long)]
        update_authority: Option<String>,

        /// Creator to filter accounts by (for CM v2 use --v2 if candy_machine account is passed)
        #[structopt(short, long)]
        creator: Option<String>,

        /// CM creator index to filter by
        #[structopt(short, long, default_value = "0")]
        position: usize,

        /// Candy machine v2 id
        #[structopt(long = "v2")]
        v2: bool,

        /// Path to JSON file with list of mint accounts to sign
        #[structopt(short, long)]
        mint_accounts_file: Option<String>,

        /// Path to directory to save output files.
        #[structopt(short, long, default_value = ".")]
        output: String,
    },
    ///Snapshot all candy machine config and state accounts for a given update_authority
    #[structopt(name = "cm-accounts")]
    CMAccounts {
        /// Update authority to filter accounts by.
        #[structopt(short, long)]
        update_authority: String,

        /// Path to directory to save output files.
        #[structopt(short, long, default_value = ".")]
        output: String,
    },
    /// Snapshot all mint accounts for a given candy_machine_id / creatoro or update authority
    #[structopt(name = "mints")]
    Mints {
        /// Creator to filter accounts by (for CM v2 use --v2 if candy_machine account is passed)
        #[structopt(short, long)]
        creator: Option<String>,

        /// CM creator index to filter by
        #[structopt(short, long, default_value = "0")]
        position: usize,

        /// Update authority to filter accounts by.
        #[structopt(short, long)]
        update_authority: Option<String>,

        /// Candy machine v2 id
        #[structopt(long = "v2")]
        v2: bool,

        /// Path to directory to save output file
        #[structopt(short, long, default_value = ".")]
        output: String,
    },
}

#[derive(Debug, StructOpt)]
pub enum UpdateSubcommands {
    /// Update the name field inside the data struct on an NFT
    #[structopt(name = "name")]
    Name {
        /// Path to the creator's keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// Mint account of corresponding metadata to update
        #[structopt(short, long)]
        account: String,

        /// New name for the metadata
        #[structopt(short, long)]
        new_name: String,
    },
    /// Update the symbol field inside the data struct on an NFT
    #[structopt(name = "symbol")]
    Symbol {
        /// Path to the creator's keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// Mint account of corresponding metadata to update
        #[structopt(short, long)]
        account: String,

        /// New name for the metadata
        #[structopt(short = "s", long)]
        new_symbol: String,
    },
    /// Update the creators field by position inside the data struct on an NFT
    #[structopt(name = "creators")]
    Creators {
        /// Path to the creator's keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// Mint account of corresponding metadata to update
        #[structopt(short, long)]
        account: String,

        /// New creators in the format: address1:share:verified,address2:share:verified,...
        #[structopt(short = "c", long)]
        new_creators: String,

        /// Should be appended instead of overwriting
        #[structopt(short = "A", long = "append")]
        append: bool,
    },
    /// Update the data struct on a NFT
    #[structopt(name = "data")]
    Data {
        /// Path to the creator's keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// Mint account of corresponding metadata to update
        #[structopt(short, long)]
        account: String,

        /// Path to JSON file containing new data
        #[structopt(short, long)]
        new_data_file: String,
    },
    /// Update the data struct on a list of NFTs
    #[structopt(name = "data-all")]
    DataAll {
        /// Path to the creator's keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// Path to directory containing JSON files with new data
        #[structopt(short, long)]
        data_dir: String,
    },
    /// Update the metadata URI, keeping the rest of the data the same
    #[structopt(name = "uri")]
    Uri {
        /// Path to the creator's keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// Mint account of corresponding metadata to update
        #[structopt(short, long)]
        account: String,

        /// New uri
        #[structopt(short = "u", long)]
        new_uri: String,
    },
    /// Update the metadata URI on a list of mint accounts
    #[structopt(name = "uri-all")]
    UriAll {
        /// Path to the creator's keypair file
        #[structopt(short, long)]
        keypair: Option<String>,

        /// JSON file with list of mint accounts and new URIs
        #[structopt(short = "u", long)]
        json_file: String,
    },
}

#[derive(Debug, StructOpt)]
pub enum WithdrawSubcommands {
    /// Withdraw funds from a candy machine v2
    #[structopt(name = "cm-v2")]
    CMV2 {
        /// Candy Machine V2 ID
        candy_machine_id: String,

        /// Path to the creator's keypair file
        #[structopt(short, long)]
        keypair: Option<String>,
    },
}
