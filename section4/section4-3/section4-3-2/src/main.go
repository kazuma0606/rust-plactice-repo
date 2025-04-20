package main

import (
	"fmt"
	"syscall"
	"unsafe"
)

func main() {
	// DLLをロード
	dll, err := syscall.LoadDLL("rust_lib.dll")
	if err != nil {
		fmt.Printf("DLLロードエラー: %v\n", err)
		return
	}
	defer dll.Release()

	// 関数を取得
	doubleFunc, err := dll.FindProc("double")
	if err != nil {
		fmt.Printf("関数取得エラー: %v\n", err)
		return
	}

	// 関数を呼び出し
	result, _, _ := doubleFunc.Call(uintptr(21))
	fmt.Printf("The double of 21 is: %d\n", result)

	// 文字列処理関数
	processStringFunc, _ := dll.FindProc("process_string")
	freeStringFunc, _ := dll.FindProc("free_string")

	// Goの文字列をCの文字列に変換
	input := "Hello from Go!"
	inputPtr, _ := syscall.BytePtrFromString(input)

	// 関数呼び出し
	processed, _, _ := processStringFunc.Call(uintptr(unsafe.Pointer(inputPtr)))
	if processed != 0 {
		// C文字列をGo文字列に変換
		goString := cStringToGoString(processed)
		fmt.Println(goString)

		// Rustのメモリを解放
		freeStringFunc.Call(processed)
	}
}

// C文字列をGo文字列に変換するヘルパー関数
func cStringToGoString(cString uintptr) string {
	// C文字列のNULL終端を探してバイト配列を作成
	ptr := unsafe.Pointer(cString)
	length := 0
	for {
		if *(*byte)(unsafe.Pointer(uintptr(ptr) + uintptr(length))) == 0 {
			break
		}
		length++
	}

	// バイトスライスを作成
	bytes := make([]byte, length)
	for i := 0; i < length; i++ {
		bytes[i] = *(*byte)(unsafe.Pointer(uintptr(ptr) + uintptr(i)))
	}

	// バイトスライスから文字列を作成
	return string(bytes)
}
