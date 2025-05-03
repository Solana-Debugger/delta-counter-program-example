use std::str::FromStr;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::system_program;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use solana_program_test::*;
use solana_sdk::account::Account;
use solana_sdk::hash::Hash;
use solana_sdk::rent::Rent;
use solana_sdk::transaction::Transaction;
use counter_program::instruction::CounterInstruction;
use counter_program::processor::process_instruction;
use counter_program::state::Counter;

mod save_input;
use save_input::save_input;

pub struct CounterTestContext {
    pub banks_client: BanksClient,
    pub payer: Keypair,
    pub recent_blockhash: Hash,
    pub user: Keypair,
    pub counter_pubkey: Pubkey,
    pub program_id: Pubkey,
}

impl CounterTestContext {
    pub async fn setup() -> CounterTestContext {
        let program_id = Pubkey::from_str("CounterProgram11111111111111111111111111111").unwrap();

        let program_test = ProgramTest::new(
            "counter_program",
            program_id,
            processor!(process_instruction)
        );

        let user = Keypair::new();

        let (counter_pubkey, bump_seed) = Pubkey::find_program_address(
            &[b"counter", user.pubkey().as_ref()],
            &program_id,
        );

        let (banks_client, payer, recent_blockhash) = program_test.start().await;

        Self {
            banks_client,
            payer,
            recent_blockhash,
            user,
            counter_pubkey,
            program_id
        }
    }

    pub async fn get_counter_account(&self) -> Option<Account> {
        self.banks_client.get_account(self.counter_pubkey).await.unwrap()
    }

    pub async fn get_counter_count(&self) -> u8 {
        let counter_account = self.get_counter_account().await.unwrap();

        let counter = Counter::try_from_slice(&counter_account.data).unwrap();

        counter.count
    }

    pub async fn run_create_counter(&self) -> Result<(), BanksClientError> {

        let ix_data = CounterInstruction::CreateCounter.try_to_vec().unwrap();

        let transaction = Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: self.program_id,
                accounts: vec![
                    AccountMeta::new_readonly(self.user.pubkey(), true),
                    AccountMeta::new(self.counter_pubkey, false),
                    AccountMeta::new(self.payer.pubkey(), true),
                    AccountMeta::new_readonly(system_program::id(), false)
                ],
                data: ix_data,
            }],
            Some(&self.payer.pubkey()),
            &[&self.payer, &self.user],
            self.recent_blockhash,
        );

        save_input(&self.banks_client, &transaction, &[&self.payer, &self.user]).await.unwrap();

        self.banks_client.process_transaction(transaction).await
    }

    pub async fn run_increase_counter(&self, delta: u8) -> Result<(), BanksClientError> {

        let ix_data = CounterInstruction::IncreaseCounter { delta }.try_to_vec().unwrap();

        let transaction = Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: self.program_id,
                accounts: vec![
                    AccountMeta::new_readonly(self.user.pubkey(), true),
                    AccountMeta::new(self.counter_pubkey, false),
                    AccountMeta::new(self.payer.pubkey(), true),
                ],
                data: ix_data,
            }],
            Some(&self.payer.pubkey()),
            &[&self.payer, &self.user],
            self.recent_blockhash,
        );

        save_input(&self.banks_client, &transaction, &[&self.payer, &self.user]).await.unwrap();

        self.banks_client.process_transaction(transaction).await
    }

    pub async fn run_increase_counter_double(&self, delta: u8, delta_2: u8) -> Result<(), BanksClientError> {

        let ix_data = CounterInstruction::IncreaseCounter { delta }.try_to_vec().unwrap();
        let ix_data_2 = CounterInstruction::IncreaseCounter { delta: delta_2 }.try_to_vec().unwrap();

        let transaction = Transaction::new_signed_with_payer(
            &[
                Instruction {
                    program_id: self.program_id,
                    accounts: vec![
                        AccountMeta::new_readonly(self.user.pubkey(), true),
                        AccountMeta::new(self.counter_pubkey, false),
                        AccountMeta::new(self.payer.pubkey(), true),
                    ],
                    data: ix_data,
                },
                Instruction {
                    program_id: self.program_id,
                    accounts: vec![
                        AccountMeta::new_readonly(self.user.pubkey(), true),
                        AccountMeta::new(self.counter_pubkey, false),
                        AccountMeta::new(self.payer.pubkey(), true),
                    ],
                    data: ix_data_2,
                }
            ],
            Some(&self.payer.pubkey()),
            &[&self.payer, &self.user],
            self.recent_blockhash,
        );

        save_input(&self.banks_client, &transaction, &[&self.payer, &self.user]).await.unwrap();

        self.banks_client.process_transaction(transaction).await
    }

    pub async fn get_rent(&self) -> Rent {
        self.banks_client.get_rent().await.unwrap()
    }
}
