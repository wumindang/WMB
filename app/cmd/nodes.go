// 文件路径: WMB/app/cmd/nodes.go

package main

import (
	"fmt"

	"github.com/spf13/cobra"
	"github.com/wumindang/WMB/app/nodes" // 替换为实际模块路径
)

var nodeManager = nodes.NewNodeManager()

var nodesCmd = &cobra.Command{
	Use:   "nodes",
	Short: "管理区块链节点",
	Long:  `添加、移除或列出区块链网络中的节点。`,
}

var addNodeCmd = &cobra.Command{
	Use:   "add [peer_id] [address]",
	Short: "添加一个节点",
	Long:  `向节点管理器添加一个新的节点，指定 PeerID 和 Multiaddr。`,
	Args:  cobra.ExactArgs(2),
	Run: func(cmd *cobra.Command, args []string) {
		peerID := args[0]
		address := args[1]

		err := nodeManager.AddNode(peerID, address)
		if err != nil {
			fmt.Println("错误:", err)
			return
		}
		fmt.Println("节点添加成功:", peerID)
	},
}

var removeNodeCmd = &cobra.Command{
	Use:   "remove [peer_id]",
	Short: "移除一个节点",
	Long:  `从节点管理器中移除指定 PeerID 的节点。`,
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		peerID := args[0]

		err := nodeManager.RemoveNode(peerID)
		if err != nil {
			fmt.Println("错误:", err)
			return
		}
		fmt.Println("节点移除成功:", peerID)
	},
}

var listNodesCmd = &cobra.Command{
	Use:   "list",
	Short: "列出所有节点",
	Long:  `获取当前节点管理器中的所有节点信息。`,
	Run: func(cmd *cobra.Command, args []string) {
		jsonStr, err := nodeManager.ToJSON()
		if err != nil {
			fmt.Println("错误:", err)
			return
		}
		fmt.Println("节点列表:", jsonStr)
	},
}

func init() {
	nodesCmd.AddCommand(addNodeCmd)
	nodesCmd.AddCommand(removeNodeCmd)
	nodesCmd.AddCommand(listNodesCmd)
	rootCmd.AddCommand(nodesCmd)
}
