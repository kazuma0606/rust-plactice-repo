use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct TodoItem {
    id: u32,
    title: String,
    completed: bool,
}

fn main() {
    let todo = TodoItem {
        id: 1,
        title: String::from("Rustを勉強する"),
        completed: false,
    };
    
    // JSONへシリアライズ
    let json = serde_json::to_string_pretty(&todo).unwrap();
    println!("JSON形式:\n{}", json);
    
    // YAMLへシリアライズ (serde_yaml クレートが必要)
    let yaml = serde_yaml::to_string(&todo).unwrap();
    println!("\nYAML形式:\n{}", yaml);
    
    // RONへシリアライズ (ron クレートが必要)
    let ron = ron::to_string(&todo).unwrap();
    println!("\nRON形式:\n{}", ron);
    
    // TOMLへシリアライズ (toml クレートが必要)
    let toml = toml::to_string(&todo).unwrap();
    println!("\nTOML形式:\n{}", toml);
    
    // 異なる形式からデシリアライズする例
    println!("\n=== デシリアライズのテスト ===");
    
    // JSONからデシリアライズ
    let todo_from_json: TodoItem = serde_json::from_str(&json).unwrap();
    println!("JSONからデシリアライズ: {:?}", todo_from_json);
    
    // YAMLからデシリアライズ
    let todo_from_yaml: TodoItem = serde_yaml::from_str(&yaml).unwrap();
    println!("YAMLからデシリアライズ: {:?}", todo_from_yaml);
}