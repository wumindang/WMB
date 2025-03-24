import 'package:flutter/material.dart';

void main() {
  runApp(const WuminbiApp());
}

class WuminbiApp extends StatelessWidget {
  const WuminbiApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Wuminbi',
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Wuminbi 手机端'),
        ),
        body: Center(
          child: ElevatedButton(
            onPressed: () {
              print('中文注释：Wuminbi 启动');
              // 中文注释：待实现与 Rust 网络交互
            },
            child: const Text('启动'),
          ),
        ),
      ),
    );
  }
}