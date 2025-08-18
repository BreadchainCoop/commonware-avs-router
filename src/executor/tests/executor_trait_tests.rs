use crate::executor::interface::{ExecutionResult, ExecutorTrait};
use anyhow::Result;
use async_trait::async_trait;
use bn254::{G1PublicKey, PublicKey, Signature};

// Test mock implementation of ExecutorTrait for testing
struct TestExecutor {
    should_succeed: bool,
    expected_result: ExecutionResult,
}

#[async_trait]
impl ExecutorTrait for TestExecutor {
    async fn execute_verification(
        &mut self,
        _payload_hash: &[u8],
        _participating_g1: &[G1PublicKey],
        _participating: &[PublicKey],
        _signatures: &[Signature],
    ) -> Result<ExecutionResult> {
        if self.should_succeed {
            Ok(self.expected_result.clone())
        } else {
            Err(anyhow::anyhow!("Test execution failed"))
        }
    }
}

#[tokio::test]
async fn test_executor_trait_success() {
    let expected = ExecutionResult {
        transaction_hash: "0xsuccess".to_string(),
        block_number: Some(100),
        gas_used: Some(50000),
        status: Some(true),
        contract_address: None,
    };

    let mut executor = TestExecutor {
        should_succeed: true,
        expected_result: expected.clone(),
    };

    let payload_hash = b"test_payload";
    let participating_g1 = vec![];
    let participating = vec![];
    let signatures = vec![];

    let result = executor
        .execute_verification(
            &payload_hash[..],
            &participating_g1,
            &participating,
            &signatures,
        )
        .await;

    assert!(result.is_ok());
    let execution_result = result.unwrap();
    assert_eq!(execution_result.transaction_hash, expected.transaction_hash);
    assert_eq!(execution_result.block_number, expected.block_number);
    assert_eq!(execution_result.gas_used, expected.gas_used);
}

#[tokio::test]
async fn test_executor_trait_failure() {
    let mut executor = TestExecutor {
        should_succeed: false,
        expected_result: ExecutionResult {
            transaction_hash: "".to_string(),
            block_number: None,
            gas_used: None,
            status: None,
            contract_address: None,
        },
    };

    let payload_hash = b"test_payload";
    let participating_g1 = vec![];
    let participating = vec![];
    let signatures = vec![];

    let result = executor
        .execute_verification(
            &payload_hash[..],
            &participating_g1,
            &participating,
            &signatures,
        )
        .await;

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Test execution failed")
    );
}

#[tokio::test]
async fn test_executor_trait_empty_inputs() {
    let expected = ExecutionResult {
        transaction_hash: "0xempty".to_string(),
        block_number: Some(0),
        gas_used: Some(0),
        status: Some(true),
        contract_address: None,
    };

    let mut executor = TestExecutor {
        should_succeed: true,
        expected_result: expected.clone(),
    };

    let result = executor.execute_verification(&[], &[], &[], &[]).await;

    assert!(result.is_ok());
    let execution_result = result.unwrap();
    assert_eq!(execution_result.transaction_hash, "0xempty");
}
