import 'package:flutter/material.dart';

void main() {
  runApp(const WuminApp());
}

class WuminApp extends StatelessWidget {
  const WuminApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Wumin App',
      home: Scaffold(
        appBar: AppBar(title: const Text('Wumin App')),
        body: const Center(child: Text('Hello, Wumin!')),
      ),
    );
  }
}