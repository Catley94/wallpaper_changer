// lib/main.dart
import 'dart:io';
import 'dart:async';

import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'models/search_response.dart';
import 'models/collections_response.dart';
import 'package:path/path.dart' as path;
import 'package:file_selector/file_selector.dart';
import 'package:flutter_svg/flutter_svg.dart';


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

  // Slideshow state
  Timer? _slideshowTimer;
  List<String> _slideshowImages = [];
  int _slideshowIndex = 0;
  Duration? _slideshowInterval;

  bool get _isSlideshowRunning => _slideshowTimer != null;

  Future<void> _setWallpaperFromPath(String p) async {
    try {
      final resp = await http.post(
        Uri.parse('http://127.0.0.1:8080/change-wallpaper-from-path'),
        headers: {'Content-Type': 'application/json'},
        body: json.encode({'path': path.normalize(p).replaceAll('\\', '/') }),
      );
      if (resp.statusCode != 200) {
        if (mounted) {
          final body = resp.body.isNotEmpty ? '\n${resp.body}' : '';
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(content: Text('Failed to set wallpaper (${resp.statusCode})$body')),
          );
        }
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Error: $e')));
      }
    }
  }

  Future<Duration?> _askInterval() async {
    final c = TextEditingController(text: '5');
    final res = await showDialog<String>(
      context: context,
      builder: (context) {
        return AlertDialog(
          title: const Text('Change interval (minutes)') ,
          content: TextField(
            controller: c,
            keyboardType: TextInputType.number,
            decoration: const InputDecoration(hintText: 'e.g. 5 (0 for no loop)'),
          ),
          actions: [
            TextButton(onPressed: () => Navigator.pop(context), child: const Text('Cancel')),
            ElevatedButton(onPressed: () => Navigator.pop(context, c.text.trim()), child: const Text('OK')),
          ],
        );
      }
    );
    if (res == null) return null;
    final mins = int.tryParse(res) ?? 0;
    if (mins <= 0) return Duration.zero; // means set once, no loop
    return Duration(minutes: mins);
  }

  void _stopSlideshow() {
    _slideshowTimer?.cancel();
    _slideshowTimer = null;
    _slideshowImages = [];
    _slideshowIndex = 0;
    _slideshowInterval = null;
    setState(() {});
  }

  Future<void> _startSlideshow(List<String> images) async {
    if (images.isEmpty) return;
    final interval = await _askInterval();
    if (interval == null) return; // cancelled
    _slideshowImages = images.map((e) => path.normalize(e).replaceAll('\\', '/')).toList();
    _slideshowIndex = 0;
    _slideshowInterval = interval;

    // Always set first image immediately
    await _setWallpaperFromPath(_slideshowImages[_slideshowIndex % _slideshowImages.length]);
    _slideshowIndex++;

    // If zero interval => no loop
    if (interval == Duration.zero) {
      _stopSlideshow();
      return;
    }

    _slideshowTimer?.cancel();
    _slideshowTimer = Timer.periodic(interval, (_) async {
      if (_slideshowImages.isEmpty) return;
      final img = _slideshowImages[_slideshowIndex % _slideshowImages.length];
      _slideshowIndex++;
      await _setWallpaperFromPath(img);
    });
    setState(() {});
  }

  Future<void> _pickSingleImage() async {
    final typeGroup = XTypeGroup(label: 'media', extensions: ['jpg','jpeg','png','bmp','gif','webp','svg','mp4','mkv','webm']);
    final file = await openFile(acceptedTypeGroups: [typeGroup]);
    if (file == null) return;
    await _setWallpaperFromPath(file.path);
  }

  Future<void> _pickFolderAndStart() async {
    final dirPath = await getDirectoryPath();
    if (dirPath == null) return;
    final dir = Directory(dirPath);
    if (!await dir.exists()) return;
    final images = await dir
        .list()
        .where((e) => e is File)
        .map((e) => (e as File).path)
        .where((p) {
          final ext = path.extension(p).toLowerCase();
          return ['.jpg','.jpeg','.png','.bmp','.gif','.webp','.svg'].contains(ext);
        })
        .toList();
    await _startSlideshow(images);
  }

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
      appBar: AppBar(
        title: const Text('Collections'),
        actions: [
          if (_isSlideshowRunning)
            TextButton.icon(
              onPressed: _stopSlideshow,
              style: TextButton.styleFrom(foregroundColor: Colors.red),
              icon: const Icon(Icons.stop_circle),
              label: const Text('Stop'),
            ),
          IconButton(
            tooltip: 'Choose Image',
            icon: const Icon(Icons.image),
            onPressed: _pickSingleImage,
          ),
          IconButton(
            tooltip: 'Choose Folder',
            icon: const Icon(Icons.folder_open),
            onPressed: _pickFolderAndStart,
          ),
        ],
      ),
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
                              final ext = path.extension(p).toLowerCase();
                              final isSvg = ext == '.svg';
                              return ClipRRect(
                                borderRadius: BorderRadius.circular(8),
                                child: isSvg
                                    ? SvgPicture.file(
                                        File(p),
                                        fit: BoxFit.cover,
                                        placeholderBuilder: (context) => Container(
                                          color: Colors.grey[200],
                                          child: const Center(child: CircularProgressIndicator(strokeWidth: 2)),
                                        ),
                                      )
                                    : Image.file(
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
                        ),
                        Padding(
                          padding: const EdgeInsets.symmetric(horizontal: 12.0),
                          child: Align(
                            alignment: Alignment.centerLeft,
                            child: TextButton.icon(
                              onPressed: () => _startSlideshow(tag.images),
                              icon: const Icon(Icons.play_circle_fill),
                              label: const Text('Slideshow'),
                            ),
                          ),
                        ),
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

  @override
  void dispose() {
    _slideshowTimer?.cancel();
    _slideshowTimer = null;
    super.dispose();
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
                                  child: (() {
                                    final p = _thumbnailPaths[index];
                                    final ext = path.extension(p).toLowerCase();
                                    if (ext == '.svg') {
                                      return SvgPicture.file(
                                        File(p),
                                        fit: BoxFit.cover,
                                        placeholderBuilder: (context) => Container(
                                          color: Colors.grey[200],
                                          child: const Center(child: CircularProgressIndicator(strokeWidth: 2)),
                                        ),
                                      );
                                    }
                                    return Image.file(
                                      File(p),
                                      fit: BoxFit.cover,
                                      errorBuilder: (context, error, stackTrace) {
                                        return Container(
                                          color: Colors.grey[200],
                                          child: const Center(
                                            child: Icon(Icons.error),
                                          ),
                                        );
                                      },
                                    );
                                  })(),
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