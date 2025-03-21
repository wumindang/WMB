// 文件路径: WMB/app/nodes/nodes.go

package nodes

import (
	"encoding/json"
	"fmt"
	"sync"
)

// Node 表示一个区块链节点
type Node struct {
	PeerID  string `json:"peer_id"`
	Address string `json:"address"`
	Status  string `json:"status"`
}

// NodeManager 管理节点列表
type NodeManager struct {
	nodes map[string]Node // PeerID -> Node
	mu    sync.Mutex
}

// NewNodeManager 创建一个新的节点管理器
func NewNodeManager() *NodeManager {
	return &NodeManager{
		nodes: make(map[string]Node),
	}
}

// AddNode 添加一个节点
func (nm *NodeManager) AddNode(peerID, address string) error {
	nm.mu.Lock()
	defer nm.mu.Unlock()

	if _, exists := nm.nodes[peerID]; exists {
		return fmt.Errorf("节点 %s 已存在", peerID)
	}

	node := Node{
		PeerID:  peerID,
		Address: address,
		Status:  "active",
	}
	nm.nodes[peerID] = node
	return nil
}

// RemoveNode 移除一个节点
func (nm *NodeManager) RemoveNode(peerID string) error {
	nm.mu.Lock()
	defer nm.mu.Unlock()

	if _, exists := nm.nodes[peerID]; !exists {
		return fmt.Errorf("节点 %s 不存在", peerID)
	}

	delete(nm.nodes, peerID)
	return nil
}

// ListNodes 获取所有节点列表
func (nm *NodeManager) ListNodes() ([]Node, error) {
	nm.mu.Lock()
	defer nm.mu.Unlock()

	var nodeList []Node
	for _, node := range nm.nodes {
		nodeList = append(nodeList, node)
	}
	return nodeList, nil
}

// GetNodeStatus 获取节点状态
func (nm *NodeManager) GetNodeStatus(peerID string) (string, error) {
	nm.mu.Lock()
	defer nm.mu.Unlock()

	node, exists := nm.nodes[peerID]
	if !exists {
		return "", fmt.Errorf("节点 %s 不存在", peerID)
	}
	return node.Status, nil
}

// ToJSON 将节点列表转换为 JSON 字符串
func (nm *NodeManager) ToJSON() (string, error) {
	nodes, err := nm.ListNodes()
	if err != nil {
		return "", err
	}
	data, err := json.Marshal(nodes)
	if err != nil {
		return "", err
	}
	return string(data), nil
}
