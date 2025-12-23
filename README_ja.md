# DBBA - Database Before/After Diff Tool

[db_before_after](https://github.com/iwazzer/db_before_after)のRust実装です。ユースケース実行前後のデータベース変更を可視化し、MySQLデータベースで何が変更されたかを正確に示すHTML差分レポートを生成します。

[English](README.md) | 日本語

## 出力例

生成される HTML 差分レポートの表示例：

[![DB Before After Output](https://github.com/iwazzer/db_before_after/raw/main/docs/images/db_diff_output.gif)](https://github.com/iwazzer/db_before_after/blob/main/docs/images/db_diff_output.gif)

## 特徴

- 📊 **ビジュアル差分レポート**: データベース変更をサイドバイサイドでHTML比較表示
- 🎯 **スマート検出**: 追加、削除、変更されたレコードを自動検出
- 🔍 **全テーブルスキャン**: データベース内の全テーブルの変更を検出
- 📋 **クリップボード連携**: 出力ファイルパスを自動的にクリップボードにコピー (macOS)
- 🏷️ **ユニークな出力**: ULIDによるユニークなファイル名生成
- 🌙 **ダークモード**: システム設定に基づく自動ダークモード対応
- ⚡ **高性能**: Ruby版の10〜100倍高速
- 📦 **単一バイナリ**: ランタイム依存なし、わずか3.3MBの実行ファイル

## インストール

### ソースからビルド

```bash
git clone https://github.com/yourusername/dbba
cd dbba
cargo build --release
cp target/release/dbba /usr/local/bin/
```

### バイナリ配布

macOSユーザーの場合、コンパイル済みバイナリを直接配布できます：

- **Apple Silicon**: リリースされたバイナリがそのまま動作
- **Intel Mac**: `--target x86_64-apple-darwin` で別途ビルド
- **ユニバーサルバイナリ**: `lipo` で両アーキテクチャを結合

すべてのシステム依存関係はmacOSにデフォルトで含まれています。

## 使い方

### 基本的な使い方

```bash
dbba -u <username> -p <password> -d <database>
```

### 全オプション

```bash
dbba [OPTIONS] --username <USERNAME> --password <PASSWORD> --database <DATABASE>

オプション:
      --help                 ヘルプ情報を表示
  -h, --host <HOST>          データベースホスト (デフォルト: 127.0.0.1)
  -P, --port <PORT>          データベースポート (デフォルト: 3306)
  -u, --username <USERNAME>  データベースユーザー名 (必須)
  -p, --password <PASSWORD>  データベースパスワード (必須)
  -d, --database <DATABASE>  データベース名 (必須)
  -e, --encoding <ENCODING>  データベースエンコーディング (デフォルト: utf8)
  -s, --suffix <SUFFIX>      出力ファイルのサフィックス (デフォルト: db_diff.html)
  -V, --version              バージョン情報を表示
```

### 環境変数

環境変数でデータベース接続パラメータを設定することもできます：

```bash
export DB_HOST=localhost
export DB_PORT=3306
export DB_USERNAME=myuser
export DB_PASSWORD=mypassword
export DB_DATABASE=mydatabase
export DB_ENCODING=utf8
```

### 使用例

1. **ツールを起動**:
   ```bash
   dbba -u myuser -p mypassword -d mydatabase
   ```

2. **最初のスナップショットを待機**:
   ```
   now reading db...
   run usecase now. then press any key when done.
   ```

3. **ユースケースを実行** (アプリケーションの実行、API呼び出しなど)

4. **Enterキーを押して続行**

5. **結果を確認**:
   ```
   now reading db...
   output: /tmp/01HG9TSFDH83E4YTZQX0PVJ5Q8_db_diff.html (Copied to clipboard)
   done.
   ```

6. **HTMLファイルを開く** (パスは既にクリップボードにコピー済み):
   ```bash
   # クリップボードの内容を貼り付けるだけ
   open /tmp/01HG9TSFDH83E4YTZQX0PVJ5Q8_db_diff.html
   ```

## 出力形式

ツールは以下の内容を含むHTMLファイルを生成します：

- **サイドバイサイド差分表示** 変更前と変更後の状態を表示
- **色分けされた変更**: 追加は緑、削除は赤で表示
- **JSON形式のレコード** 読みやすい形式
- **ソート済みテーブル表示** アルファベット順に表示
- **自動処理** バイナリデータを含む様々なデータ型を自動処理（MD5ハッシュ）

## パフォーマンス

Ruby版との比較：

| 項目 | Ruby版 | Rust版 |
|------|--------|--------|
| 起動時間 | ~500ms | ~10ms |
| メモリ使用量 | ~100MB | ~20MB |
| DB読み取り (10テーブル) | ~2s | ~400ms |
| JSON処理 | ~1s | ~10ms |

## 必要要件

- Rust 1.70以降 (ビルド時)
- MySQLデータベース
- macOS、Linux、またはWindows

## 開発

開発用ビルド：

```bash
cargo build
```

リリース用ビルド：

```bash
cargo build --release
```

cargoで実行：

```bash
cargo run -- -u myuser -p mypass -d mydb
```

### テストの実行

```bash
# すべてのテストを実行
cargo test

# 詳細出力付きで実行
cargo test -- --nocapture
```

## コントリビューション

バグレポートやプルリクエストは GitHub https://github.com/yourusername/dbba で受け付けています。

## ライセンス

このツールは [MITライセンス](https://opensource.org/licenses/MIT) の下でオープンソースとして利用可能です。

## リンク

- オリジナルRuby版: [db_before_after](https://github.com/iwazzer/db_before_after)
