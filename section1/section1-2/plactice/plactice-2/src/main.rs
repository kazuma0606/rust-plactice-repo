use clap::Parser;
use jsonschema::{Draft, JSONSchema};
use std::fs;
use std::path::PathBuf;

/// コマンドライン引数の構造体
#[derive(Parser)]
struct Args {
    #[arg(short, long, value_parser = validate_json_file)]
    file: PathBuf,
    
    #[arg(short, long)]
    schema: Option<PathBuf>,
}

/// `.json` 拡張子のバリデーション
fn validate_json_file(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("json") => Ok(path),
        _ => Err(String::from("拡張子 .json のファイルを指定してください")),
    }
}

fn main() {
    let args = Args::parse();

    // JSONファイルの読み込み
    let data = fs::read_to_string(&args.file)
        .expect("ファイルの読み込みに失敗しました");

    // JSONのパース
    let json: serde_json::Value = serde_json::from_str(&data)
        .expect("不正なJSONです");
    
    // スキーマの検証
    if let Some(schema_path) = args.schema {
        let schema_str = fs::read_to_string(&schema_path)
            .expect("スキーマファイルの読み込みに失敗しました");
            
        let schema: serde_json::Value = serde_json::from_str(&schema_str)
            .expect("不正なスキーマJSONです");
            
        // JSONSchemaコンパイラの作成
        let compiled_schema = JSONSchema::options()
            .with_draft(Draft::Draft7)  // Draft7を使用
            .compile(&schema)
            .expect("スキーマのコンパイルに失敗しました");
            
        // バリデーション実行
        let validation_result = compiled_schema.validate(&json);
        
        // 検証結果の処理
        if let Err(errors) = validation_result {
            println!("JSONスキーマ検証エラー:");
            for error in errors {
                println!("- {}", error);
            }
            std::process::exit(1);
        } else {
            println!("スキーマ検証: 成功");
        }
    } else {
        println!("スキーマファイルが指定されていないため、検証はスキップされました");
    }

    // 整形して出力
    println!("JSON内容:");
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}