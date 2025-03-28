use std::str::FromStr;

use clap::{Args, Parser, Subcommand};

use crate::config::{DatabaseConnection, DatabaseType};

/// Docker データベースコンテナに簡単に接続できる CLI ツール
#[derive(Debug, Parser)]
#[command(
    name = "dbcli",
    about = "Docker データベースコンテナに簡単に接続するツール",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// サブコマンド
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// データベースコンテナに接続
    #[command(name = "connect", about = "データベースコンテナに接続")]
    Connect(ConnectArgs),

    /// 接続設定を追加
    #[command(name = "add", about = "接続設定を追加")]
    Add(AddArgs),

    /// 接続設定を削除
    #[command(name = "remove", about = "接続設定を削除")]
    Remove(RemoveArgs),

    /// 接続設定の一覧を表示
    #[command(name = "list", about = "接続設定の一覧を表示")]
    List,
}

/// 接続コマンドの引数
#[derive(Debug, Args)]
pub struct ConnectArgs {
    /// エイリアス名（未指定の場合はコンテナ名などの引数が必要）
    pub alias: Option<String>,

    /// コンテナ名（エイリアスを使用しない場合）
    #[arg(short, long)]
    pub container: Option<String>,

    /// データベースの種類（postgres, mysql, mongodbのいずれか）
    #[arg(short, long)]
    pub db_type: Option<String>,

    /// ユーザー名
    #[arg(short, long)]
    pub user: Option<String>,

    /// パスワード
    #[arg(short, long)]
    pub password: Option<String>,

    /// データベース名
    #[arg(short, long)]
    pub database: Option<String>,

    /// ポート番号
    #[arg(short, long)]
    pub port: Option<u16>,
}

impl ConnectArgs {
    /// 接続情報をDatabaseConnectionに変換
    pub fn to_connection(&self) -> Option<DatabaseConnection> {
        if let (Some(container), Some(db_type_str), Some(user)) =
            (&self.container, &self.db_type, &self.user)
        {
            if let Ok(db_type) = DatabaseType::from_str(db_type_str) {
                return Some(DatabaseConnection {
                    db_type,
                    container: container.clone(),
                    user: user.clone(),
                    password: self.password.clone(),
                    database: self.database.clone(),
                    port: self.port,
                    options: None,
                });
            }
        }
        None
    }
}

/// 設定追加コマンドの引数
#[derive(Debug, Args)]
pub struct AddArgs {
    /// エイリアス名
    pub alias: String,

    /// コンテナ名
    #[arg(short, long)]
    pub container: String,

    /// データベースの種類（postgres, mysql, mongodbのいずれか）
    #[arg(short, long)]
    pub db_type: String,

    /// ユーザー名
    #[arg(short, long)]
    pub user: String,

    /// パスワード
    #[arg(short, long)]
    pub password: Option<String>,

    /// データベース名
    #[arg(short, long)]
    pub database: Option<String>,

    /// ポート番号
    #[arg(short, long)]
    pub port: Option<u16>,
}

impl AddArgs {
    /// 接続情報をDatabaseConnectionに変換
    pub fn to_connection(&self) -> Result<DatabaseConnection, String> {
        let db_type = DatabaseType::from_str(&self.db_type)
            .map_err(|e| format!("データベースタイプの解析エラー: {}", e))?;

        Ok(DatabaseConnection {
            db_type,
            container: self.container.clone(),
            user: self.user.clone(),
            password: self.password.clone(),
            database: self.database.clone(),
            port: self.port,
            options: None,
        })
    }
}

/// 設定削除コマンドの引数
#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// 削除するエイリアス名
    pub alias: String,
}
