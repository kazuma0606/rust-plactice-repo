fn call_once<F>(f: F)
where
    F: FnOnce(),
{
    // FnOnceは一度だけ呼び出せる
    f();
    // f(); // これはコンパイルエラー - 既に消費されている
}

fn call_mut<F: FnMut()>(mut f: F) {
    // FnMutは状態を変更できる
    f();
    f(); // 複数回呼び出し可能
}

fn call<F: Fn()>(f: F) {
    // Fnは何度でも呼び出せる（状態を変更しない）
    f();
    f();
    f();
}

fn main() {
    // 1. FnOnceの例 - 値を消費するクロージャ
    let text = String::from("消費される");
    call_once(|| {
        // 値の所有権を奪う操作
        let _consumed = text;
        println!("FnOnce: 値を消費: {}", _consumed);
    });
    // println!("{}", text); // エラー: textは既に消費されている

    // 2. FnMutの例 - 状態を変更するクロージャ
    let mut counter = 0;
    call_mut(|| {
        // 変数を変更する
        counter += 1;
        println!("FnMut: カウンター: {}", counter);
    });
    println!("最終カウンター: {}", counter); // 2になっている

    // 3. Fnの例 - 状態を変更しないクロージャ
    let name = String::from("Rust");
    call(|| {
        // 値を参照するだけ
        println!("Fn: Hello, {}!", name);
    });
    println!("nameはまだ使える: {}", name);

    // 4. より明確な違いを示す例
    let mut values = vec![1, 2, 3];
    
    // Fn - 参照のみ
    call(|| {
        println!("Fn: 値を読み取るだけ: {:?}", values);
    });

    // FnMut - 可変参照
    call_mut(|| {
        values.push(4); // 変更する
        println!("FnMut: 値を変更: {:?}", values);
    });

    // FnOnce - 所有権を奪う
    call_once(|| {
        let sum: i32 = values.iter().sum(); // values自体は消費しない
        println!("FnOnce: 合計を計算: {}", sum);
    });
    
    // valuesはまだ使える
    println!("最終的な値: {:?}", values);
    
    // 本当のFnOnceの例 - 所有権を奪う
    call_once(|| {
        let taken = std::mem::take(&mut values); // valuesの内容を奪う
        println!("FnOnce: 値を奪った: {:?}", taken);
    });
    
    println!("valuesは空になった: {:?}", values);
}