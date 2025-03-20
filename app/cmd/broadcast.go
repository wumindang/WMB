package cmd

/*
#include <stdlib.h>

void free_string(char* str) {
    free(str);
}
*/
import "C"

import (
	"context"
	"fmt"
	"log"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/libp2p/go-libp2p"
	"github.com/libp2p/go-libp2p/core/host"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/libp2p/go-libp2p/p2p/discovery/mdns"
	"github.com/multiformats/go-multiaddr"
	"github.com/spf13/cobra"
	"github.com/wumindang/WMB/core/network"
)

// broadcastCmd represents the broadcast command
var broadcastCmd = &cobra.Command{
	Use:   "broadcast",
	Short: "Broadcast a message to the network",
	Long:  `Broadcast a message to the network using libp2p.`,
	Run:   broadcast,
}

func init() {
	rootCmd.AddCommand(broadcastCmd)
}

func broadcast(cmd *cobra.Command, args []string) {
	// 创建上下文
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// 创建 libp2p 主机
	host, err := libp2p.New(libp2p.ListenAddrStrings("/ip4/0.0.0.0/tcp/0"))
	if err != nil {
		log.Fatalf("Failed to create libp2p host: %v", err)
	}
	log.Printf("Host ID: %s", host.ID().Pretty())

	// 初始化 mDNS 服务发现
	mdnsService := mdns.NewMdnsService(host, network.DiscoveryTag)
	mdnsService.RegisterNotifee(&discoveryNotifee{Host: host})
	log.Println("mDNS discovery started")

	// 处理信号
	sigCh := make(chan os.Signal, 1)
	signal.Notify(sigCh, syscall.SIGINT, syscall.SIGTERM)

	// 启动消息广播
	go startBroadcasting(ctx, host)

	// 等待信号
	<-sigCh
	log.Println("Shutting down...")
}

type discoveryNotifee struct {
	Host host.Host
}

func (n *discoveryNotifee) HandlePeerFound(pi peer.AddrInfo) {
	log.Printf("Discovered peer: %s", pi.ID.Pretty())
	if err := n.Host.Connect(context.Background(), pi); err != nil {
		log.Printf("Failed to connect to peer %s: %v", pi.ID.Pretty(), err)
	}
}

func startBroadcasting(ctx context.Context, host host.Host) {
	ticker := time.NewTicker(5 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-ticker.C:
			msg := fmt.Sprintf("Hello from %s at %s", host.ID().Pretty(), time.Now().Format(time.RFC3339))
			for _, p := range host.Peerstore().Peers() {
				if p == host.ID() {
					continue
				}
				stream, err := host.NewStream(ctx, p, network.ProtocolID)
				if err != nil {
					log.Printf("Failed to create stream to peer %s: %v", p.Pretty(), err)
					continue
				}
				_, err = stream.Write([]byte(msg))
				if err != nil {
					log.Printf("Failed to send message to peer %s: %v", p.Pretty(), err)
					stream.Reset()
					continue
				}
				stream.Close()
				log.Printf("Sent message to peer %s: %s", p.Pretty(), msg)
			}
		case <-ctx.Done():
			return
		}
	}
}

func parseMultiaddr(addrStr string) (multiaddr.Multiaddr, error) {
	addr, err := multiaddr.NewMultiaddr(addrStr)
	if err != nil {
		return nil, err
	}
	return addr, nil
}

func parsePeerID(idStr string) (peer.ID, error) {
	id, err := peer.Decode(idStr)
	if err != nil {
		return "", err
	}
	return id, nil
}
