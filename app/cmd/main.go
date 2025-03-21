// 文件路径: WMB/app/cmd/main.go

package main

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "wmb",
	Short: "五民币区块链命令行工具",
	Long:  `WMB 是五民币金融区块链系统的命令行工具，用于管理密钥、交易和网络操作。`,
}

func main() {
	rootCmd.AddCommand(keygenCmd)
	rootCmd.AddCommand(signCmd)
	rootCmd.AddCommand(broadcastCmd)
	rootCmd.AddCommand(nodesCmd) // 新增

	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
