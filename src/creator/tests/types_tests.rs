use crate::creator::types::{TaskData, TaskQueue};
use crate::ingress::TaskRequest;

/// Mock task queue for testing TaskData
#[derive(Debug)]
struct MockQueue;

impl TaskQueue for MockQueue {
    fn push(&self, _task: TaskRequest) {
        // Mock implementation
    }

    fn pop(&self) -> Option<TaskRequest> {
        None
    }
}

#[test]
fn test_task_data_new() {
    let task_data = TaskData::new(
        "var1_value".to_string(),
        "var2_value".to_string(),
        "var3_value".to_string(),
    );

    assert_eq!(task_data.var1, "var1_value");
    assert_eq!(task_data.var2, "var2_value");
    assert_eq!(task_data.var3, "var3_value");
}

#[test]
fn test_task_data_default() {
    let task_data = TaskData::default();

    assert_eq!(task_data.var1, "default_var1");
    assert_eq!(task_data.var2, "default_var2");
    assert_eq!(task_data.var3, "default_var3");
}

#[test]
fn test_task_data_encode_into_payload() {
    let task_data = TaskData::new(
        "test1".to_string(),
        "test2".to_string(),
        "test3".to_string(),
    );

    let initial_payload = vec![1, 2, 3, 4];
    let result = task_data.encode_into_payload(initial_payload.clone());

    // Should contain the initial payload plus the encoded task data
    assert!(result.len() > initial_payload.len());

    // Should contain the task data strings with null terminators
    let result_str = String::from_utf8_lossy(&result);
    assert!(result_str.contains("test1"));
    assert!(result_str.contains("test2"));
    assert!(result_str.contains("test3"));
}

#[test]
fn test_task_data_encode_into_payload_empty_initial() {
    let task_data = TaskData::new("var1".to_string(), "var2".to_string(), "var3".to_string());

    let result = task_data.encode_into_payload(vec![]);

    // Should contain the task data even with empty initial payload
    assert!(!result.is_empty());

    let result_str = String::from_utf8_lossy(&result);
    assert!(result_str.contains("var1"));
    assert!(result_str.contains("var2"));
    assert!(result_str.contains("var3"));
}

#[test]
fn test_task_data_encode_into_payload_with_null_terminators() {
    let task_data = TaskData::new("hello".to_string(), "world".to_string(), "test".to_string());

    let result = task_data.encode_into_payload(vec![]);

    // Check that null terminators are present
    let mut null_count = 0;
    for &byte in &result {
        if byte == 0 {
            null_count += 1;
        }
    }

    // Should have 3 null terminators (one for each string)
    assert_eq!(null_count, 3);
}

#[test]
fn test_task_data_encode_into_payload_preserves_initial_data() {
    let task_data = TaskData::new("var1".to_string(), "var2".to_string(), "var3".to_string());

    let initial_payload = vec![1, 2, 3, 4, 5];
    let result = task_data.encode_into_payload(initial_payload.clone());

    // The initial payload should be preserved at the beginning
    assert_eq!(
        &result[0..initial_payload.len()],
        initial_payload.as_slice()
    );
}

#[test]
fn test_task_data_with_empty_strings() {
    let task_data = TaskData::new("".to_string(), "".to_string(), "".to_string());

    let result = task_data.encode_into_payload(vec![]);

    // Should still contain null terminators even for empty strings
    let mut null_count = 0;
    for &byte in &result {
        if byte == 0 {
            null_count += 1;
        }
    }

    assert_eq!(null_count, 3);
}

#[test]
fn test_task_data_with_unicode_strings() {
    let task_data = TaskData::new(
        "hello 世界".to_string(),
        "world 🌍".to_string(),
        "test 🧪".to_string(),
    );

    let result = task_data.encode_into_payload(vec![]);

    // Should handle unicode correctly
    let result_str = String::from_utf8_lossy(&result);
    assert!(result_str.contains("hello 世界"));
    assert!(result_str.contains("world 🌍"));
    assert!(result_str.contains("test 🧪"));
}

#[test]
fn test_task_queue_trait_bounds() {
    // Test that TaskQueue trait bounds are satisfied
    let queue = MockQueue;

    // Test Send + Sync bounds
    fn assert_send_sync<T: Send + Sync>(_t: T) {}
    assert_send_sync(queue);

    // Additional verification that the mock queue works as expected
    let queue2 = MockQueue;
    assert!(queue2.pop().is_none());
}

#[test]
fn test_task_data_clone() {
    let task_data = TaskData::new(
        "original1".to_string(),
        "original2".to_string(),
        "original3".to_string(),
    );

    let cloned = task_data.clone();

    assert_eq!(task_data.var1, cloned.var1);
    assert_eq!(task_data.var2, cloned.var2);
    assert_eq!(task_data.var3, cloned.var3);
}

#[test]
fn test_task_data_debug() {
    let task_data = TaskData::new(
        "debug1".to_string(),
        "debug2".to_string(),
        "debug3".to_string(),
    );

    let debug_str = format!("{:?}", task_data);

    // Should contain the field names and values
    assert!(debug_str.contains("debug1"));
    assert!(debug_str.contains("debug2"));
    assert!(debug_str.contains("debug3"));
}

#[test]
fn test_task_data_equality() {
    let task_data1 = TaskData::new(
        "same1".to_string(),
        "same2".to_string(),
        "same3".to_string(),
    );

    let task_data2 = TaskData::new(
        "same1".to_string(),
        "same2".to_string(),
        "same3".to_string(),
    );

    let task_data3 = TaskData::new(
        "different1".to_string(),
        "same2".to_string(),
        "same3".to_string(),
    );

    // Should be equal when all fields are the same
    assert_eq!(task_data1.var1, task_data2.var1);
    assert_eq!(task_data1.var2, task_data2.var2);
    assert_eq!(task_data1.var3, task_data2.var3);

    // Should be different when fields differ
    assert_ne!(task_data1.var1, task_data3.var1);
}
