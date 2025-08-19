use anyhow::Result;

use super::types::DefaultTaskData;
use crate::creator::core::TaskRequest;

/// Default task data factory for counter operations
pub struct DefaultTaskDataFactory;

impl DefaultTaskDataFactory {
    pub async fn create_task_data(&self) -> Result<DefaultTaskData> {
        Ok(DefaultTaskData::default())
    }

    pub async fn create_task_data_from_request(
        &self,
        request: &TaskRequest,
    ) -> Result<DefaultTaskData> {
        Ok(DefaultTaskData {
            var1: request.body.var1.clone(),
            var2: request.body.var2.clone(),
            var3: request.body.var3.clone(),
        })
    }
}
