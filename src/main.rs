use clap::Parser;
use dbba::adapters::{DatabaseAdapter, HtmlOutputAdapter, MySqlAdapter};
use dbba::{DbDiff, DbInfo};
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "dbba")]
#[command(about = "Database Before/After Diff Tool", long_about = None)]
#[command(version)]
#[command(disable_help_flag = true)]
struct Cli {
    /// Show help information
    #[arg(long, action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// Database host
    #[arg(short = 'h', long, env = "DB_HOST", default_value = "127.0.0.1")]
    host: String,

    /// Database port
    #[arg(short = 'P', long, env = "DB_PORT", default_value_t = 3306)]
    port: u16,

    /// Database username
    #[arg(short = 'u', long, env = "DB_USERNAME")]
    username: String,

    /// Database password
    #[arg(short = 'p', long, env = "DB_PASSWORD")]
    password: String,

    /// Database name
    #[arg(short = 'd', long, env = "DB_DATABASE")]
    database: String,

    /// Database encoding
    #[arg(short = 'e', long, env = "DB_ENCODING", default_value = "utf8")]
    encoding: String,

    /// Output file suffix
    #[arg(short = 's', long, default_value = "db_diff.html")]
    suffix: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Create database info
    let db_info = DbInfo::new(
        cli.host,
        cli.port,
        cli.username,
        cli.password,
        cli.database,
        cli.encoding,
    );

    // Generate output file path with ULID
    let ulid = ulid::Ulid::new();
    let output_path: PathBuf = if let Ok(rails_root) = std::env::var("RAILS_ROOT") {
        PathBuf::from(rails_root)
    } else {
        PathBuf::from("/tmp")
    }
    .join(format!("{}_{}", ulid, cli.suffix));

    // Create output file with buffered writer
    let file = File::create(&output_path)?;
    let writer = BufWriter::new(file);

    // Create adapters
    let mut db_adapter = MySqlAdapter::new(db_info);
    db_adapter.connect().await?;

    let output_adapter = HtmlOutputAdapter::new(writer);

    // Create and execute diff
    let mut db_diff = DbDiff::new(db_adapter, output_adapter);
    db_diff.execute().await?;

    // Copy to clipboard
    let clipboard_content = format!("open {}", output_path.display());
    match arboard::Clipboard::new() {
        Ok(mut clipboard) => {
            if let Err(e) = clipboard.set_text(&clipboard_content) {
                eprintln!("Warning: Failed to copy to clipboard: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Warning: Failed to initialize clipboard: {}", e);
        }
    }

    println!(
        "output: {} (Copied to clipboard)",
        output_path.display()
    );

    Ok(())
}
