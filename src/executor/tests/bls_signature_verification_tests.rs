use crate::bindings::blssigcheckoperatorstateretriever::BLSSigCheckOperatorStateRetriever::getNonSignerStakesAndSignatureReturn;
use crate::executor::interface::{BlsSignatureVerificationHandler, ExecutionResult};
use crate::executor::tests::mocks::MockVerificationHandler;
use alloy_primitives::{Bytes, FixedBytes};
use anyhow::Result;
use async_trait::async_trait;

// Test BLS signature verification handler that can be configured for different test scenarios
struct TestBlsSignatureVerificationHandler {
    should_succeed: bool,
    expected_result: ExecutionResult,
    call_count: u32,
}

impl TestBlsSignatureVerificationHandler {
    fn new(should_succeed: bool, expected_result: ExecutionResult) -> Self {
        Self {
            should_succeed,
            expected_result,
            call_count: 0,
        }
    }
}

#[async_trait]
impl BlsSignatureVerificationHandler for TestBlsSignatureVerificationHandler {
    async fn handle_verification(
        &mut self,
        _msg_hash: FixedBytes<32>,
        _quorum_numbers: Bytes,
        _current_block_number: u32,
        _non_signer_data: getNonSignerStakesAndSignatureReturn,
    ) -> Result<ExecutionResult> {
        self.call_count += 1;

        if self.should_succeed {
            Ok(self.expected_result.clone())
        } else {
            Err(anyhow::anyhow!("Contract handler test failure"))
        }
    }
}

#[test]
fn test_mock_contract_handler_creation() {
    let handler = MockVerificationHandler::new();
    assert_eq!(format!("{:?}", handler), "MockVerificationHandler");
}

#[test]
fn test_mock_contract_handler_default() {
    let handler = MockVerificationHandler;
    assert_eq!(format!("{:?}", handler), "MockVerificationHandler");
}

#[tokio::test]
async fn test_mock_contract_handler_success() {
    let mut handler = MockVerificationHandler::new();

    let msg_hash = FixedBytes::<32>::ZERO;
    let quorum_numbers = Bytes::from_static(b"test");
    let current_block_number = 12345;

    // Create a mock non-signer data
    let mock_data = getNonSignerStakesAndSignatureReturn {
        _0: crate::bindings::blssigcheckoperatorstateretriever::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature {
            nonSignerQuorumBitmapIndices: vec![],
            nonSignerPubkeys: vec![],
            quorumApks: vec![],
            apkG2: crate::bindings::blssigcheckoperatorstateretriever::BN254::G2Point {
                X: [Default::default(), Default::default()],
                Y: [Default::default(), Default::default()],
            },
            sigma: crate::bindings::blssigcheckoperatorstateretriever::BN254::G1Point {
                X: Default::default(),
                Y: Default::default(),
            },
            quorumApkIndices: vec![],
            totalStakeIndices: vec![],
            nonSignerStakeIndices: vec![],
        },
    };

    let result = handler
        .handle_verification(msg_hash, quorum_numbers, current_block_number, mock_data)
        .await;

    assert!(result.is_ok());
    let execution_result = result.unwrap();
    assert_eq!(execution_result.transaction_hash, "0x1234567890abcdef");
    assert_eq!(execution_result.block_number, Some(12345));
    assert_eq!(execution_result.gas_used, Some(21000));
    assert_eq!(execution_result.status, Some(true));
    assert_eq!(execution_result.contract_address, None);
}

#[tokio::test]
async fn test_contract_handler_trait_success() {
    let expected_result = ExecutionResult {
        transaction_hash: "0xtest123".to_string(),
        block_number: Some(99999),
        gas_used: Some(75000),
        status: Some(true),
        contract_address: Some("0xcontract456".to_string()),
    };

    let mut handler = TestBlsSignatureVerificationHandler::new(true, expected_result.clone());

    let msg_hash = FixedBytes::<32>::ZERO;
    let quorum_numbers = Bytes::from_static(b"test");
    let current_block_number = 54321;

    let mock_data = getNonSignerStakesAndSignatureReturn {
        _0: crate::bindings::blssigcheckoperatorstateretriever::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature {
            nonSignerQuorumBitmapIndices: vec![],
            nonSignerPubkeys: vec![],
            quorumApks: vec![],
            apkG2: crate::bindings::blssigcheckoperatorstateretriever::BN254::G2Point {
                X: [Default::default(), Default::default()],
                Y: [Default::default(), Default::default()],
            },
            sigma: crate::bindings::blssigcheckoperatorstateretriever::BN254::G1Point {
                X: Default::default(),
                Y: Default::default(),
            },
            quorumApkIndices: vec![],
            totalStakeIndices: vec![],
            nonSignerStakeIndices: vec![],
        },
    };

    let result = handler
        .handle_verification(msg_hash, quorum_numbers, current_block_number, mock_data)
        .await;

    assert!(result.is_ok());
    let execution_result = result.unwrap();
    assert_eq!(
        execution_result.transaction_hash,
        expected_result.transaction_hash
    );
    assert_eq!(execution_result.block_number, expected_result.block_number);
    assert_eq!(execution_result.gas_used, expected_result.gas_used);
    assert_eq!(handler.call_count, 1);
}

#[tokio::test]
async fn test_contract_handler_trait_failure() {
    let expected_result = ExecutionResult {
        transaction_hash: "".to_string(),
        block_number: None,
        gas_used: None,
        status: None,
        contract_address: None,
    };

    let mut handler = TestBlsSignatureVerificationHandler::new(false, expected_result);

    let msg_hash = FixedBytes::<32>::ZERO;
    let quorum_numbers = Bytes::from_static(b"test");
    let current_block_number = 54321;

    let mock_data = getNonSignerStakesAndSignatureReturn {
        _0: crate::bindings::blssigcheckoperatorstateretriever::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature {
            nonSignerQuorumBitmapIndices: vec![],
            nonSignerPubkeys: vec![],
            quorumApks: vec![],
            apkG2: crate::bindings::blssigcheckoperatorstateretriever::BN254::G2Point {
                X: [Default::default(), Default::default()],
                Y: [Default::default(), Default::default()],
            },
            sigma: crate::bindings::blssigcheckoperatorstateretriever::BN254::G1Point {
                X: Default::default(),
                Y: Default::default(),
            },
            quorumApkIndices: vec![],
            totalStakeIndices: vec![],
            nonSignerStakeIndices: vec![],
        },
    };

    let result = handler
        .handle_verification(msg_hash, quorum_numbers, current_block_number, mock_data)
        .await;

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Contract handler test failure")
    );
    assert_eq!(handler.call_count, 1);
}
