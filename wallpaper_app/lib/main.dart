// lib/main.dart
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'models/search_response.dart';


void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Wallpaper Changer',
      theme: ThemeData(
        primarySwatch: Colors.blue,
        useMaterial3: true,
      ),
      home: const WallpaperPage(),
    );
  }
}

class WallpaperPage extends StatefulWidget {
  const WallpaperPage({super.key});

  @override
  State<WallpaperPage> createState() => _WallpaperPageState();
}

class _WallpaperPageState extends State<WallpaperPage> {
  final TextEditingController _searchController = TextEditingController();
  List<String> _thumbnailPaths = [];
  bool _isLoading = false;

  Future<void> _searchTheme() async {
    setState(() {
      isLoading: true;
      _thumbnailPaths = [];
    });
    try {
      final response = await http.get(
        Uri.parse('http://127.0.0.1:8080/search?topic=${_searchController.text}&page=1'),
      );
      
      if (response.statusCode == 200) {
        final searchResponse = SearchResponse.fromJson(json.decode(response.body));
        setState(() {
          _thumbnailPaths = searchResponse.thumbnailPaths;

        });
      } else {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text("Error: ${response.statusCode}")),
        );
      }
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Error: $e')),
      );
    } finally {
      setState(() {
        _isLoading = false;
      });
    }
  }

  Future<void> _changeWallpaper(String themeId) async {
    try {
      final response = await http.post(
        Uri.parse('http://127.0.0.1:8080/change'),
        body: json.encode({'theme_id': themeId}),
        headers: {'Content-Type': 'application/json'},
      );
      
      if (response.statusCode == 200) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Wallpaper changed successfully!')),
        );
      } else {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Error: ${response.statusCode}')),
        );
      }
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Error: $e')),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Wallpaper Changer'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            TextField(
              controller: _searchController,
              decoration: const InputDecoration(
                labelText: 'Search Themes',
                border: OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: _searchTheme,
              child: const Text('Search'),
            ),
            const SizedBox(height: 16),
            Expanded(
              child: _thumbnailPaths.isEmpty
                  ? const Center(
                      child: Text('No images found'),
                    )
                  : GridView.builder(
                      gridDelegate: const SliverGridDelegateWithMaxCrossAxisExtent(
                        maxCrossAxisExtent: 300, // Maximum width for each item
                        crossAxisSpacing: 10,
                        mainAxisSpacing: 10,
                        childAspectRatio: 16/9, // Maintain your aspect ratio
                      ),
                      itemCount: _thumbnailPaths.length,
                      itemBuilder: (context, index) {
  // Extract just the ID part (assuming format wallhaven-XXXXXX.jpg)
  String imageId = _thumbnailPaths[index]
      .split('/')
      .last                    // Get filename from path
      .replaceAll('wallhaven-', '') // Remove 'wallhaven-' prefix
      .split('.')
      .first;                  // Remove file extension

  return InkWell(
    onTap: () async {
      print('Image Clicked: $imageId');
      final response = await http.get(
        Uri.parse('http://127.0.0.1:8080/change-wallpaper?id=${imageId}'),
      );

      if (response.statusCode == 200) {
        print('Wallpaper Changed');
      }

    },
    child: Container(
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(8),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withOpacity(0.2),
            blurRadius: 5,
            offset: const Offset(0, 3),
          ),
        ],
      ),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(8),
        child: Image.file(
          File(_thumbnailPaths[index]),
          fit: BoxFit.cover,
          errorBuilder: (context, error, stackTrace) {
            return Container(
              color: Colors.grey[200],
              child: const Center(
                child: Icon(Icons.error),
              ),
            );
          },
        ),
      ),
    ),
  );
},
                    ),
            ),

          ],
        ),
      ),
    );
  }

  @override
  void dispose() {
    _searchController.dispose();
    super.dispose();
  }
}