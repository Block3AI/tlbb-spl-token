use anchor_lang::prelude::*;
use anchor_lang::InstructionData;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use solana_program_test::*;
use solana_sdk::{
    account::Account as SolanaAccount,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

#[tokio::test]
async fn test_initialize() {
    let mut program_test = ProgramTest::new(
        "tlbb_spl_token",                         // Program name
        tlbb_spl_token::id(),                     // Program ID
        processor!(tlbb_spl_token::entry),        // Program entry point
    );

    // Add accounts for testing
    let mint_keypair = Keypair::new();
    let owner_keypair = Keypair::new();
    let liquidity_pool_keypair = Keypair::new();
    let presale_wallet_keypair = Keypair::new();
    let marketing_wallet_keypair = Keypair::new();
    let team_wallet_keypair = Keypair::new();
    let community_rewards_keypair = Keypair::new();
    let charity_wallet_keypair = Keypair::new();

    // Add accounts to the test environment
    program_test.add_account(
        mint_keypair.pubkey(),
        SolanaAccount {
            lamports: 1_000_000_000,
            data: vec![0; Mint::LEN],
            owner: spl_token::id(),
            executable: false,
            rent_epoch: 0,
        },
    );

    program_test.add_account(
        owner_keypair.pubkey(),
        SolanaAccount {
            lamports: 1_000_000_000,
            data: vec![0; TokenAccount::LEN],
            owner: spl_token::id(),
            executable: false,
            rent_epoch: 0,
        },
    );

    // Start the test environment
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Build the instruction for initialization
    let total_supply = 1_000_000_000;
    let initialize_ix = anchor_lang::InstructionData::data(&tlbb_spl_token::instruction::Initialize {
        total_supply,
    });

    let accounts = vec![
        AccountMeta::new(mint_keypair.pubkey(), false),
        AccountMeta::new(owner_keypair.pubkey(), true),
        AccountMeta::new(liquidity_pool_keypair.pubkey(), false),
        AccountMeta::new(presale_wallet_keypair.pubkey(), false),
        AccountMeta::new(marketing_wallet_keypair.pubkey(), false),
        AccountMeta::new(team_wallet_keypair.pubkey(), false),
        AccountMeta::new(community_rewards_keypair.pubkey(), false),
        AccountMeta::new(charity_wallet_keypair.pubkey(), false),
    ];

    let tx = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id: tlbb_spl_token::id(),
            accounts,
            data: initialize_ix,
        }],
        Some(&payer.pubkey()),
        &[&payer, &owner_keypair],
        recent_blockhash,
    );

    // Send the transaction and assert success
    banks_client.process_transaction(tx).await.unwrap();

    // Check token supply allocation
    let liquidity_pool_account: TokenAccount =
        banks_client.get_account(liquidity_pool_keypair.pubkey()).await.unwrap();
    assert_eq!(liquidity_pool_account.amount, total_supply * 30 / 100);

    let presale_wallet_account: TokenAccount =
        banks_client.get_account(presale_wallet_keypair.pubkey()).await.unwrap();
    assert_eq!(presale_wallet_account.amount, total_supply * 25 / 100);

    let charity_wallet_account: TokenAccount =
        banks_client.get_account(charity_wallet_keypair.pubkey()).await.unwrap();
    assert_eq!(charity_wallet_account.amount, total_supply * 5 / 100);
}

#[tokio::test]
async fn test_transfer_with_fee() {
    let mut program_test = ProgramTest::new(
        "tlbb_spl_token",
        tlbb_spl_token::id(),
        processor!(tlbb_spl_token::entry),
    );

    // Add sender and recipient accounts
    let sender_keypair = Keypair::new();
    let recipient_keypair = Keypair::new();
    let charity_wallet_keypair = Keypair::new();
    let development_wallet_keypair = Keypair::new();

    // Add accounts to the test environment
    program_test.add_account(
        sender_keypair.pubkey(),
        SolanaAccount {
            lamports: 1_000_000_000,
            data: vec![0; TokenAccount::LEN],
            owner: spl_token::id(),
            executable: false,
            rent_epoch: 0,
        },
    );

    program_test.add_account(
        recipient_keypair.pubkey(),
        SolanaAccount {
            lamports: 1_000_000_000,
            data: vec![0; TokenAccount::LEN],
            owner: spl_token::id(),
            executable: false,
            rent_epoch: 0,
        },
    );

    // Start the test environment
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Build the instruction for transferring with fees
    let transfer_amount = 100_000;
    let transfer_with_fee_ix = anchor_lang::InstructionData::data(&tlbb_spl_token::instruction::TransferWithFee {
        amount: transfer_amount,
    });

    let accounts = vec![
        AccountMeta::new(sender_keypair.pubkey(), true),
        AccountMeta::new(recipient_keypair.pubkey(), false),
        AccountMeta::new(charity_wallet_keypair.pubkey(), false),
        AccountMeta::new(development_wallet_keypair.pubkey(), false),
    ];

    let tx = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id: tlbb_spl_token::id(),
            accounts,
            data: transfer_with_fee_ix,
        }],
        Some(&payer.pubkey()),
        &[&payer, &sender_keypair],
        recent_blockhash,
    );

    // Send the transaction and assert success
    banks_client.process_transaction(tx).await.unwrap();

    // Validate transaction fee distribution
    let charity_wallet_account: TokenAccount =
        banks_client.get_account(charity_wallet_keypair.pubkey()).await.unwrap();
    assert_eq!(charity_wallet_account.amount, transfer_amount * 1 / 100);

    let recipient_account: TokenAccount =
        banks_client.get_account(recipient_keypair.pubkey()).await.unwrap();
    assert_eq!(recipient_account.amount, transfer_amount - transfer_amount * 2 / 100);
}