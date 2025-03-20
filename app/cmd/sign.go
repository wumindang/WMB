// 文件路径: WMB/app/cmd/sign.go

package main

import (
	"fmt"

	"github.com/spf13/cobra"
)

// #cgo LDFLAGS: -L../../core/target/release -lwmb_core
// #include <stdlib.h>
// extern void sign_transaction(const char* sender, const char* receiver, double amount, double fee, const char* private_key, char** signature);
// extern void free_string(char* ptr);
import "C"

var signCmd = &cobra.Command{
	Use:   "sign [sender] [receiver] [amount] [fee] [privateKey]",
	Short: "对交易进行签名",
	Long:  `使用私钥对指定的交易进行签名，生成签名数据。金额和费用单位为元，支持两位小数（分）。`,
	Args:  cobra.ExactArgs(5),
	Run: func(cmd *cobra.Command, args []string) {
		sender := C.CString(args[0])
		receiver := C.CString(args[1])
		amount := C.double(StringToFloat(args[2]))
		fee := C.double(StringToFloat(args[3]))
		privateKey := C.CString(args[4])

		var signature *C.char
		C.sign_transaction(sender, receiver, amount, fee, privateKey, &signature)

		signatureStr := C.GoString(signature)

		fmt.Println("交易签名 (HEX):", signatureStr)

		// 释放 C 内存
		C.free_string(sender)
		C.free_string(receiver)
		C.free_string(privateKey)
		C.free_string(signature)
	},
}

// StringToFloat 简单转换字符串到浮点数
func StringToFloat(s string) float64 {
	var f float64
	fmt.Sscanf(s, "%f", &f)
	return f
}

func init() {}
