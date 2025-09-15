class CollectionsResponse {
  final List<CollectionTag> tags;
  CollectionsResponse({required this.tags});
  factory CollectionsResponse.fromJson(Map<String, dynamic> json) {
    final tagsJson = json['tags'] as List<dynamic>? ?? [];
    return CollectionsResponse(
      tags: tagsJson.map((e) => CollectionTag.fromJson(e as Map<String, dynamic>)).toList(),
    );
  }
}

class CollectionTag {
  final String name;
  final List<String> images;
  CollectionTag({required this.name, required this.images});
  factory CollectionTag.fromJson(Map<String, dynamic> json) {
    final images = (json['images'] as List<dynamic>? ?? []).map((e) => e.toString()).toList();
    return CollectionTag(name: json['name']?.toString() ?? '', images: images);
  }
}
