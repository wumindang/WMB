// 文件路径: WMB/app/cmd/keygen.go

package main

import (
	"fmt"

	"github.com/spf13/cobra"
)

// #cgo LDFLAGS: -L../../core/target/release -lwmb_core
// #include <stdlib.h>
// extern void generate_keypair(char** private_key, char** public_key);
// extern void free_string(char* ptr);
import "C"

var keygenCmd = &cobra.Command{
	Use:   "keygen",
	Short: "生成 ECDSA 密钥对",
	Long:  `生成一个新的 ECDSA 密钥对（私钥和公钥），使用 secp256k1 曲线。`,
	Run: func(cmd *cobra.Command, args []string) {
		var privateKey, publicKey *C.char
		C.generate_keypair(&privateKey, &publicKey)

		privateKeyStr := C.GoString(privateKey)
		publicKeyStr := C.GoString(publicKey)

		fmt.Println("私钥 (HEX):", privateKeyStr)
		fmt.Println("公钥 (HEX):", publicKeyStr)

		// 释放 C 内存
		C.free_string(privateKey)
		C.free_string(publicKey)
	},
}

func init() {}
