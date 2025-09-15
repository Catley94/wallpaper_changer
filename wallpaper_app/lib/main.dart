// lib/main.dart
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'models/search_response.dart';
import 'models/collections_response.dart';
import 'package:path/path.dart' as path;
import 'models/collections_response.dart';


// Building app Linux (plus production build): https://docs.flutter.dev/platform-integration/linux/building

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
      home: const HomeTabs(),
    );
  }
}

class HomeTabs extends StatefulWidget {
  const HomeTabs({super.key});
  @override
  State<HomeTabs> createState() => _HomeTabsState();
}

class _HomeTabsState extends State<HomeTabs> {
  int _index = 0;
  final _pages = const [WallpaperPage(), CollectionsPage()];
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: _pages[_index],
      bottomNavigationBar: BottomNavigationBar(
        currentIndex: _index,
        onTap: (i) => setState(() => _index = i),
        items: const [
          BottomNavigationBarItem(icon: Icon(Icons.search), label: 'Search'),
          BottomNavigationBarItem(icon: Icon(Icons.collections), label: 'Collections'),
        ],
      ),
    );
  }
}

class CollectionsPage extends StatefulWidget {
  const CollectionsPage({super.key});
  @override
  State<CollectionsPage> createState() => _CollectionsPageState();
}

class _CollectionsPageState extends State<CollectionsPage> {
  bool _loading = false;
  List<CollectionTag> _tags = [];

  Future<void> _load() async {
    setState(() { _loading = true; });
    try {
      final resp = await http.get(Uri.parse('http://127.0.0.1:8080/collections'));
      if (resp.statusCode == 200) {
        final jsonMap = json.decode(resp.body) as Map<String, dynamic>;
        final data = CollectionsResponse.fromJson(jsonMap);
        setState(() { _tags = data.tags; });
      } else {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Error: ${resp.statusCode}')));
        }
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Error: $e')));
      }
    } finally {
      if (mounted) setState(() { _loading = false; });
    }
  }

  @override
  void initState() {
    super.initState();
    _load();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Collections')),
      body: _loading
          ? const Center(child: CircularProgressIndicator())
          : _tags.isEmpty
              ? const Center(child: Text('No tags yet'))
              : ListView.builder(
                  itemCount: _tags.length,
                  itemBuilder: (context, i) {
                    final tag = _tags[i];
                    return ExpansionTile(
                      title: Text(tag.name),
                      children: [
                        Padding(
                          padding: const EdgeInsets.all(12.0),
                          child: GridView.builder(
                            shrinkWrap: true,
                            physics: const NeverScrollableScrollPhysics(),
                            gridDelegate: const SliverGridDelegateWithMaxCrossAxisExtent(
                              maxCrossAxisExtent: 300,
                              crossAxisSpacing: 10,
                              mainAxisSpacing: 10,
                              childAspectRatio: 16/9,
                            ),
                            itemCount: tag.images.length,
                            itemBuilder: (context, j) {
                              final p = path.normalize(tag.images[j]).replaceAll('\\', '/');
                              return ClipRRect(
                                borderRadius: BorderRadius.circular(8),
                                child: Image.file(
                                  File(p),
                                  fit: BoxFit.cover,
                                  errorBuilder: (context, error, stackTrace) {
                                    return Container(
                                      color: Colors.grey[200],
                                      child: const Center(child: Icon(Icons.error)),
                                    );
                                  },
                                ),
                              );
                            },
                          ),
                        )
                      ],
                    );
                  },
                ),
      floatingActionButton: FloatingActionButton.extended(
        onPressed: () async {
          final name = await showDialog<String>(
            context: context,
            builder: (context) {
              final c = TextEditingController();
              return AlertDialog(
                title: const Text('Create Tag'),
                content: TextField(controller: c, decoration: const InputDecoration(labelText: 'Tag name')),
                actions: [
                  TextButton(onPressed: () => Navigator.pop(context), child: const Text('Cancel')),
                  ElevatedButton(onPressed: () => Navigator.pop(context, c.text), child: const Text('Create')),
                ],
              );
            },
          );
          if (name != null && name.trim().isNotEmpty) {
            try {
              final resp = await http.post(
                Uri.parse('http://127.0.0.1:8080/collections/tags'),
                headers: {'Content-Type': 'application/json'},
                body: json.encode({'name': name.trim()}),
              );
              if (resp.statusCode == 200) {
                _load();
              } else {
                if (mounted) ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Error: ${resp.statusCode}')));
              }
            } catch (e) {
              if (mounted) ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Error: $e')));
            }
          }
        },
        label: const Text('New Tag'), icon: const Icon(Icons.add),
      ),
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
  String topic = "";
  List<String> _thumbnailPaths = [];
  bool _isLoading = false;
  int page = 1;

  Future<void> _searchTheme() async {
    setState(() {
      _isLoading = true;
      _thumbnailPaths = [];
      if (_searchController.text != topic) {
        // When the searching topic changes, revert back to page 1
        topic = _searchController.text;
        page = 1;
        print("Resetting to page 1");
      }
    });
    try {
      final response = await http.get(
        Uri.parse('http://127.0.0.1:8080/search?topic=${_searchController.text}&page=${page}'),
      );
      
      if (response.statusCode == 200) {
        final searchResponse = SearchResponse.fromJson(json.decode(response.body));
        setState(() {
          _thumbnailPaths = searchResponse.thumbnailPaths.map((thumbnailPath) {
            // Normalize and replace backslashes
            return path.normalize(thumbnailPath).replaceAll(r'\', '/');
          }).toList();
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

  Future<void> _tagImage(String imageId) async {
    try {
      // Load tags
      final tagsResp = await http.get(Uri.parse('http://127.0.0.1:8080/collections'));
      if (tagsResp.statusCode != 200) {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Failed to load tags: ${tagsResp.statusCode}')));
        return;
      }
      final data = CollectionsResponse.fromJson(json.decode(tagsResp.body));
      if (data.tags.isEmpty) {
        ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('No tags. Create one in Collections tab.')));
        return;
      }
      final selected = await showDialog<String>(
        context: context,
        builder: (context) {
          return SimpleDialog(
            title: const Text('Tag image'),
            children: data.tags.map((t) => SimpleDialogOption(
              onPressed: () => Navigator.pop(context, t.name),
              child: Text(t.name),
            )).toList(),
          );
        }
      );
      if (selected == null) return;
      final resp = await http.post(
        Uri.parse('http://127.0.0.1:8080/collections/tag-image'),
        headers: {'Content-Type': 'application/json'},
        body: json.encode({'id': imageId, 'tag': selected}),
      );
      if (resp.statusCode == 200) {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Tagged to "$selected"')));
      } else {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Tagging failed: ${resp.statusCode}')));
      }
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Error: $e')));
    }
  }

  Future<void> _next() async {
    setState(() {
      // Increase the page number
      page += 1;
    });
    _searchTheme();
  }

  Future<void> _previous() async {
    setState(() {
      if (page > 1) {
        // Decrease the page number
        page -= 1;
      }
    });
    _searchTheme();
  }

  Future<void> _changeWallpaper(String imageId) async {
    try {
      final response = await http.get(
        Uri.parse('http://127.0.0.1:8080/change-wallpaper?id=${imageId}'),
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
            Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                ElevatedButton(
                  onPressed: _isLoading || page == 1 ? null : _previous,
                  child: const Text('Previous'),
                ),
                ElevatedButton(
                  onPressed: _isLoading ? null : _searchTheme,
                  child: Padding(
                    padding: const EdgeInsets.all(16.0),
                    child: const Text('Search')
                  ),
                ),
                ElevatedButton(
                  onPressed: _isLoading ? null : _next,
                  child: const Text('Next'),
                ),
              ],
            ),
            const SizedBox(height: 16),
            Expanded(
              child: _isLoading
                  ? const Center(child: CircularProgressIndicator())
                  : _thumbnailPaths.isEmpty
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
                                _changeWallpaper(imageId);
                              },
                              onLongPress: () {
                                _tagImage(imageId);
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