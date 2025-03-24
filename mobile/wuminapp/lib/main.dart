import 'package:flutter/material.dart';

void main() {
  runApp(const WuminappApp());
}

class WuminappApp extends StatelessWidget {
  const WuminappApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Wuminapp',
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Wuminapp 手机端'),
        ),
        body: Center(
          child: ElevatedButton(
            onPressed: () {
              print('中文注释：Wuminapp 启动');
              // 中文注释：待实现与 Rust 网络交互
            },
            child: const Text('启动'),
          ),
        ),
      ),
    );
  }
}