use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum CounterInstruction {
    //
    // Creates a new counter
    //
    // Accounts:
    // 0: User (signer)
    // 1: Counter PDA account (writable)
    //    Seeds: ['counter', user]
    // 2: Payer (signer, writable)
    // 3: System program
    //
    CreateCounter,

    //
    // Increases counter by a set amount
    //
    // Accounts:
    // 0: User (signer)
    // 1: Counter PDA account (writable)
    //    Seeds: ['counter', user]
    // 2: Payer (signer, writable)
    //
    IncreaseCounter {
        delta: u8
    }
}
