use crate::adapters::OutputAdapter;
use crate::error::Result;
use similar::{ChangeTag, TextDiff};
use std::io::Write;

/// HTML output adapter that generates diff reports
pub struct HtmlOutputAdapter<W: Write> {
    writer: W,
}

impl<W: Write> HtmlOutputAdapter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Get modern CSS styles
    fn modern_css() -> &'static str {
        r#"
        * {
          margin: 0;
          padding: 0;
          box-sizing: border-box;
        }

        body {
          font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
          line-height: 1.6;
          color: #333;
          background-color: #f8f9fa;
        }

        .container {
          max-width: 1200px;
          margin: 0 auto;
          padding: 20px;
        }

        .header {
          background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
          color: white;
          padding: 2rem;
          border-radius: 12px;
          margin-bottom: 2rem;
          box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        }

        .header h1 {
          font-size: 2.5rem;
          font-weight: 700;
          margin-bottom: 0.5rem;
        }

        .subtitle {
          font-size: 1.1rem;
          opacity: 0.9;
        }

        .main-content {
          min-height: 400px;
        }

        .table-section {
          background: white;
          border-radius: 12px;
          padding: 2rem;
          margin-bottom: 2rem;
          box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
          border: 1px solid #e9ecef;
        }

        .table-title {
          font-size: 1.8rem;
          font-weight: 600;
          color: #2c3e50;
          margin-bottom: 1.5rem;
          padding-bottom: 0.5rem;
          border-bottom: 3px solid #3498db;
        }

        .diff-container {
          margin-top: 1rem;
        }

        .diff-part {
          display: flex;
          gap: 2rem;
          margin-bottom: 2rem;
          border: 1px solid #dee2e6;
          border-radius: 8px;
          overflow: hidden;
        }

        .diff-side {
          flex: 1;
          min-width: 0;
        }

        .diff-header {
          font-size: 1.2rem;
          font-weight: 600;
          padding: 1rem;
          margin: 0;
          text-align: center;
          color: white;
        }

        .diff-left .diff-header {
          background-color: #e74c3c;
        }

        .diff-right .diff-header {
          background-color: #27ae60;
        }

        .diff-content {
          padding: 1rem;
          background-color: #f8f9fa;
          min-height: 200px;
          overflow-x: auto;
        }

        .no-diff-message {
          text-align: center;
          padding: 4rem 2rem;
          background: white;
          border-radius: 12px;
          box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        }

        .no-diff-icon {
          font-size: 4rem;
          margin-bottom: 1rem;
        }

        .no-diff-message h2 {
          color: #27ae60;
          margin-bottom: 1rem;
          font-size: 2rem;
        }

        .no-diff-message p {
          font-size: 1.2rem;
          color: #6c757d;
        }

        .footer {
          text-align: center;
          padding: 2rem;
          color: #6c757d;
          margin-top: 2rem;
        }

        .footer a {
          color: #3498db;
          text-decoration: none;
          font-weight: 600;
        }

        .footer a:hover {
          text-decoration: underline;
        }

        @media (max-width: 768px) {
          .container {
            padding: 10px;
          }

          .header h1 {
            font-size: 2rem;
          }

          .diff-part {
            flex-direction: column;
            gap: 0;
          }

          .diff-side {
            border-bottom: 1px solid #dee2e6;
          }

          .diff-side:last-child {
            border-bottom: none;
          }
        }

        @media (prefers-color-scheme: dark) {
          body {
            background-color: #1a1a1a;
            color: #e0e0e0;
          }

          .table-section {
            background: #2d2d2d;
            border-color: #444;
          }

          .table-title {
            color: #f0f0f0;
          }

          .diff-content {
            background-color: #383838;
            color: #e0e0e0;
          }

          .no-diff-message {
            background: #2d2d2d;
          }

          .diff ul {
            background: #2d2d2d !important;
            color: #e0e0e0;
          }

          .diff li.ins {
            background: #2d5a2d !important;
            color: #90ee90 !important;
          }

          .diff li.del {
            background: #5a2d2d !important;
            color: #ffb3b3 !important;
          }

          .diff li:hover {
            background: #4a4a00 !important;
          }
        }
        "#
    }

    /// Get diff-specific CSS styles
    fn diffy_css() -> &'static str {
        r#"
        .diff {
          overflow: auto;
          border-radius: 6px;
        }

        .diff ul {
          background: #fff;
          overflow: auto;
          font-size: 13px;
          list-style: none;
          margin: 0;
          padding: 0;
          display: table;
          width: 100%;
          font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
        }

        .diff del, .diff ins {
          display: block;
          text-decoration: none;
        }

        .diff li {
          padding: 0;
          display: table-row;
          margin: 0;
          height: 1em;
        }

        .diff li.ins {
          background: #dfd;
          color: #080;
        }

        .diff li.del {
          background: #fee;
          color: #b00;
        }

        .diff li:hover {
          background: #ffc;
        }

        .diff del, .diff ins, .diff span {
          white-space: pre-wrap;
          font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
        }

        .diff del strong {
          font-weight: normal;
          background: #fcc;
        }

        .diff ins strong {
          font-weight: normal;
          background: #9f9;
        }

        .diff li.diff-comment {
          display: none;
        }

        .diff li.diff-block-info {
          background: none repeat scroll 0 0 gray;
        }
        "#
    }

    /// HTML escape a string
    fn html_escape(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }
}

impl<W: Write> OutputAdapter for HtmlOutputAdapter<W> {
    fn start_output(&mut self) -> Result<()> {
        let now = chrono::Local::now();
        write!(
            self.writer,
            r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Database Diff Report</title>
  <style>
{}
{}
  </style>
</head>
<body>
  <div class="container">
    <header class="header">
      <h1>Database Diff Report</h1>
      <p class="subtitle">Generated at {}</p>
    </header>
    <main class="main-content">
"#,
            Self::modern_css(),
            Self::diffy_css(),
            now.format("%Y-%m-%d %H:%M:%S")
        )?;
        Ok(())
    }

    fn end_output(&mut self) -> Result<()> {
        write!(
            self.writer,
            r#"    </main>
    <footer class="footer">
      <p>Generated by <a href="https://github.com/iwazzer/db_before_after" target="_blank">dbba (Rust version)</a></p>
    </footer>
  </div>
</body>
</html>
"#
        )?;
        Ok(())
    }

    fn write_title(&mut self, title: &str) -> Result<()> {
        write!(
            self.writer,
            r#"<section class="table-section">
  <h2 class="table-title">{}</h2>
  <div class="diff-container">
"#,
            Self::html_escape(title)
        )?;
        Ok(())
    }

    fn write_diff_section(&mut self, left: &str, right: &str) -> Result<()> {
        write!(
            self.writer,
            r#"<div class="diff-part">
  <div class="diff-side diff-left">
    <h3 class="diff-header">Before</h3>
    <div class="diff-content">{}</div>
  </div>
  <div class="diff-side diff-right">
    <h3 class="diff-header">After</h3>
    <div class="diff-content">{}</div>
  </div>
</div>
"#,
            left, right
        )?;
        Ok(())
    }

    fn write_no_diff_message(&mut self) -> Result<()> {
        write!(
            self.writer,
            r#"<div class="no-diff-message">
  <div class="no-diff-icon">✅</div>
  <h2>No Changes Detected</h2>
  <p>The database state remained unchanged during the operation.</p>
</div>
"#
        )?;
        Ok(())
    }

    fn close_section(&mut self) -> Result<()> {
        write!(
            self.writer,
            r#"  </div>
</section>
"#
        )?;
        Ok(())
    }

    fn generate_diff(&self, left: &str, right: &str) -> (String, String) {
        let diff = TextDiff::from_lines(left, right);

        let mut left_html = String::from("<div class=\"diff\"><ul>");
        let mut right_html = String::from("<div class=\"diff\"><ul>");

        for change in diff.iter_all_changes() {
            let line = Self::html_escape(change.value());
            match change.tag() {
                ChangeTag::Delete => {
                    left_html.push_str(&format!("<li class=\"del\"><del>{}</del></li>", line));
                    right_html.push_str(&format!("<li class=\"del\"><del></del></li>"));
                }
                ChangeTag::Insert => {
                    left_html.push_str(&format!("<li class=\"ins\"><ins></ins></li>"));
                    right_html.push_str(&format!("<li class=\"ins\"><ins>{}</ins></li>", line));
                }
                ChangeTag::Equal => {
                    left_html.push_str(&format!("<li><span>{}</span></li>", line));
                    right_html.push_str(&format!("<li><span>{}</span></li>", line));
                }
            }
        }

        left_html.push_str("</ul></div>");
        right_html.push_str("</ul></div>");

        (left_html, right_html)
    }
}
