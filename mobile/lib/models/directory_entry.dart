class DirectoryEntry {
  DirectoryEntry({
    required this.userId,
    required this.displayName,
    required this.email,
    this.department,
    this.position,
  });

  final String userId;
  final String displayName;
  final String email;
  final String? department;
  final String? position;

  factory DirectoryEntry.fromJson(Map<String, dynamic> json) => DirectoryEntry(
        userId: json['userId'] as String,
        displayName: json['displayName'] as String,
        email: json['email'] as String,
        department: json['department'] as String?,
        position: json['position'] as String?,
      );
}
