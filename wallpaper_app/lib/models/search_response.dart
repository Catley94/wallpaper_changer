class SearchResponse {
  final List<String> thumbnailPaths;
  // You can add more fields from the WHSearchResponse if needed

  SearchResponse({
    required this.thumbnailPaths,
  });

  factory SearchResponse.fromJson(Map<String, dynamic> json) {
    return SearchResponse(
      thumbnailPaths: List<String>.from(json['thumbnail_paths'] as List),
    );
  }
}
