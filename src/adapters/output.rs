use crate::error::Result;

/// Trait for output adapters that generate diff reports
pub trait OutputAdapter {
    /// Start the output (write header)
    fn start_output(&mut self) -> Result<()>;

    /// End the output (write footer)
    fn end_output(&mut self) -> Result<()>;

    /// Write a table title/section header
    fn write_title(&mut self, title: &str) -> Result<()>;

    /// Write a diff section with left (before) and right (after) content
    fn write_diff_section(&mut self, left: &str, right: &str) -> Result<()>;

    /// Write a message when no differences are detected
    fn write_no_diff_message(&mut self) -> Result<()>;

    /// Close the current section
    fn close_section(&mut self) -> Result<()>;

    /// Generate diff HTML from left and right content
    fn generate_diff(&self, left: &str, right: &str) -> (String, String);
}
