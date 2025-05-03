use borsh::BorshDeserialize;
use solana_program_test::*;
use counter_program::state::Counter;

mod counter_test;
use counter_test::CounterTestContext;


#[tokio::test]
async fn test_create_counter() {

    let test_context = CounterTestContext::setup().await;

    test_context.run_create_counter().await.unwrap();

    // Verify that it was created
    let counter_account = test_context.get_counter_account().await.unwrap();

    let rent = test_context.get_rent().await;
    let space = std::mem::size_of::<Counter>();
    let rent_lamports = rent.minimum_balance(space);

    // Check lamports
    assert_eq!(rent_lamports, counter_account.lamports);

    // Check data
    assert_eq!(vec![0; space], counter_account.data);

    let counter = Counter::try_from_slice(&counter_account.data).unwrap();
    assert_eq!(0, counter.count);

    // Check owner
    assert_eq!(test_context.program_id, counter_account.owner);

    // Check executable
    assert!(!counter_account.executable);
}

#[tokio::test]
async fn test_increase_counter() {
    let test_context = CounterTestContext::setup().await;

    test_context.run_create_counter().await.unwrap();

    assert_eq!(0, test_context.get_counter_count().await);

    test_context.run_increase_counter(1).await.unwrap();

    assert_eq!(1, test_context.get_counter_count().await);

    test_context.run_increase_counter(2).await.unwrap();

    assert_eq!(3, test_context.get_counter_count().await);
}

#[tokio::test]
async fn test_arithmetic_overflow() {
    let test_context = CounterTestContext::setup().await;

    test_context.run_create_counter().await.unwrap();

    test_context.run_increase_counter(100).await.unwrap();

    assert_eq!(100, test_context.get_counter_count().await);

    test_context.run_increase_counter(155).await.unwrap();

    assert_eq!(255, test_context.get_counter_count().await);

    let result = test_context.run_increase_counter(1).await;

    let error = result.expect_err("arithmetic overflow");

    assert_eq!("transport transaction error: Error processing Instruction 0: Program arithmetic overflowed", error.to_string());
}

#[tokio::test]
async fn test_increase_counter_double() {
    let test_context = CounterTestContext::setup().await;

    test_context.run_create_counter().await.unwrap();

    test_context.run_increase_counter_double(2, 5).await.unwrap();

    assert_eq!(7, test_context.get_counter_count().await);
}