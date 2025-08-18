use crate::executor::interface::ExecutionResult;

#[test]
fn test_execution_result_creation() {
    let result = ExecutionResult {
        transaction_hash: "0x1234567890abcdef".to_string(),
        block_number: Some(12345),
        gas_used: Some(21000),
        status: Some(true),
        contract_address: None,
    };

    assert_eq!(result.transaction_hash, "0x1234567890abcdef");
    assert_eq!(result.block_number, Some(12345));
    assert_eq!(result.gas_used, Some(21000));
    assert_eq!(result.status, Some(true));
    assert_eq!(result.contract_address, None);
}

#[test]
fn test_execution_result_clone() {
    let result = ExecutionResult {
        transaction_hash: "0xabcdef1234567890".to_string(),
        block_number: Some(67890),
        gas_used: Some(42000),
        status: Some(false),
        contract_address: Some("0xcontract123".to_string()),
    };

    let cloned = result.clone();

    assert_eq!(result.transaction_hash, cloned.transaction_hash);
    assert_eq!(result.block_number, cloned.block_number);
    assert_eq!(result.gas_used, cloned.gas_used);
    assert_eq!(result.status, cloned.status);
    assert_eq!(result.contract_address, cloned.contract_address);
}

#[test]
fn test_execution_result_debug() {
    let result = ExecutionResult {
        transaction_hash: "0xdebug".to_string(),
        block_number: None,
        gas_used: None,
        status: None,
        contract_address: None,
    };

    let debug_string = format!("{:?}", result);
    assert!(debug_string.contains("0xdebug"));
    assert!(debug_string.contains("ExecutionResult"));
}
