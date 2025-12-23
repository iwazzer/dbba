# DBBA - Database Before/After Diff Tool

A Rust implementation of [db_before_after](https://github.com/iwazzer/db_before_after). Visualizes database changes before and after executing a use case, generating HTML diff reports showing exactly what changed in your MySQL database.

English | [日本語](README_ja.md)

## Example Output

Here's what the generated HTML diff report looks like:

[![DB Before After Output](https://github.com/iwazzer/db_before_after/raw/main/docs/images/db_diff_output.gif)](https://github.com/iwazzer/db_before_after/blob/main/docs/images/db_diff_output.gif)

## Features

- 📊 **Visual Diff Reports**: Side-by-side HTML comparison of database changes
- 🎯 **Smart Detection**: Automatically detects added, deleted, and modified records
- 🔍 **All Tables**: Scans all tables in your database for changes
- 📋 **Clipboard Integration**: Automatically copies output file path to clipboard (macOS)
- 🏷️ **Unique Output**: Uses ULID for unique file naming
- 🌙 **Dark Mode**: Automatic dark mode support based on system preferences
- ⚡ **High Performance**: 10-100x faster than Ruby version
- 📦 **Single Binary**: No runtime dependencies, just 3.3MB executable

## Installation

### From Source

```bash
git clone https://github.com/yourusername/dbba
cd dbba
cargo build --release
cp target/release/dbba /usr/local/bin/
```

### Binary Distribution

For macOS users, you can distribute the compiled binary directly:

- **Apple Silicon**: The released binary works out of the box
- **Intel Mac**: Build separately with `--target x86_64-apple-darwin`
- **Universal Binary**: Use `lipo` to combine both architectures

All system dependencies are included in macOS by default.

## Usage

### Basic Usage

```bash
dbba -u <username> -p <password> -d <database>
```

### Full Options

```bash
dbba [OPTIONS] --username <USERNAME> --password <PASSWORD> --database <DATABASE>

Options:
      --help                 Show help information
  -h, --host <HOST>          Database host (default: 127.0.0.1)
  -P, --port <PORT>          Database port (default: 3306)
  -u, --username <USERNAME>  Database username (required)
  -p, --password <PASSWORD>  Database password (required)
  -d, --database <DATABASE>  Database name (required)
  -e, --encoding <ENCODING>  Database encoding (default: utf8)
  -s, --suffix <SUFFIX>      Output file suffix (default: db_diff.html)
  -V, --version              Print version
```

### Environment Variables

You can also set database connection parameters via environment variables:

```bash
export DB_HOST=localhost
export DB_PORT=3306
export DB_USERNAME=myuser
export DB_PASSWORD=mypassword
export DB_DATABASE=mydatabase
export DB_ENCODING=utf8
```

### Example Workflow

1. **Start the tool**:
   ```bash
   dbba -u myuser -p mypassword -d mydatabase
   ```

2. **Wait for first snapshot**:
   ```
   now reading db...
   run usecase now. then press any key when done.
   ```

3. **Execute your use case** (run your application, make API calls, etc.)

4. **Press Enter to continue**

5. **View the results**:
   ```
   now reading db...
   output: /tmp/01HG9TSFDH83E4YTZQX0PVJ5Q8_db_diff.html (Copied to clipboard)
   done.
   ```

6. **Open the HTML file** (path is already copied to clipboard):
   ```bash
   # Just paste the clipboard content
   open /tmp/01HG9TSFDH83E4YTZQX0PVJ5Q8_db_diff.html
   ```

## Output Format

The tool generates an HTML file with:

- **Side-by-side diff view** showing before and after states
- **Color-coded changes**: Green for additions, red for deletions
- **JSON formatted records** for easy reading
- **Sorted table display** in alphabetical order
- **Automatic handling** of different data types including binary data (MD5 hash)

## Performance

Compared to the Ruby version:

| Metric | Ruby | Rust |
|--------|------|------|
| Startup Time | ~500ms | ~10ms |
| Memory Usage | ~100MB | ~20MB |
| DB Read (10 tables) | ~2s | ~400ms |
| JSON Processing | ~1s | ~10ms |

## Requirements

- Rust 1.70 or later (for building)
- MySQL database
- macOS, Linux, or Windows

## Development

Build for development:

```bash
cargo build
```

Build for release:

```bash
cargo build --release
```

Run with cargo:

```bash
cargo run -- -u myuser -p mypass -d mydb
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture
```

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/yourusername/dbba.

## License

The tool is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).

## Links

- Original Ruby version: [db_before_after](https://github.com/iwazzer/db_before_after)
