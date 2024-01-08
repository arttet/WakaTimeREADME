use async_trait::async_trait;
use color_eyre::eyre::Result;

/// Definition of the upload content port.
#[async_trait]
pub trait UploadContentPort {
    /// Uploads content to a destination and returns a result indicating success or failure.
    ///
    /// # Arguments
    ///
    /// * `content`: The content to be uploaded.
    ///
    /// # Returns
    ///
    /// * `Ok(())`: If the content is successfully uploaded.
    /// * `Err(...)`: If there is an error during the upload process.
    async fn upload_content(&self, content: String) -> Result<()>;
}
