class DocFolder {
  DocFolder({
    required this.id,
    required this.name,
    this.parentId,
    this.createdByName,
    required this.createdAt,
  });

  final String id;
  final String name;
  final String? parentId;
  final String? createdByName;
  final DateTime createdAt;

  factory DocFolder.fromJson(Map<String, dynamic> json) => DocFolder(
        id: json['id'] as String,
        name: json['name'] as String,
        parentId: json['parentId'] as String?,
        createdByName: json['createdByName'] as String?,
        createdAt: DateTime.parse(json['createdAt'] as String).toLocal(),
      );
}

class DocumentMeta {
  DocumentMeta({
    required this.id,
    this.folderId,
    this.docType,
    required this.title,
    this.description,
    required this.fileName,
    required this.contentType,
    required this.fileSize,
    required this.isImage,
    this.expiryDate,
    this.uploadedByName,
    required this.createdAt,
  });

  final String id;
  final String? folderId;
  final String? docType;
  final String title;
  final String? description;
  final String fileName;
  final String contentType;
  final int fileSize;
  final bool isImage;
  final DateTime? expiryDate;
  final String? uploadedByName;
  final DateTime createdAt;

  factory DocumentMeta.fromJson(Map<String, dynamic> json) => DocumentMeta(
        id: json['id'] as String,
        folderId: json['folderId'] as String?,
        docType: json['docType'] as String?,
        title: json['title'] as String,
        description: json['description'] as String?,
        fileName: json['fileName'] as String,
        contentType: json['contentType'] as String,
        fileSize: json['fileSize'] as int,
        isImage: json['isImage'] as bool? ?? false,
        expiryDate: json['expiryDate'] != null ? DateTime.parse(json['expiryDate'] as String) : null,
        uploadedByName: json['uploadedByName'] as String?,
        createdAt: DateTime.parse(json['createdAt'] as String).toLocal(),
      );
}

class FolderListing {
  FolderListing({required this.folders, required this.documents, required this.breadcrumb});

  final List<DocFolder> folders;
  final List<DocumentMeta> documents;
  final List<DocFolder> breadcrumb;

  factory FolderListing.fromJson(Map<String, dynamic> json) => FolderListing(
        folders: (json['folders'] as List<dynamic>)
            .map((e) => DocFolder.fromJson(e as Map<String, dynamic>))
            .toList(),
        documents: (json['documents'] as List<dynamic>)
            .map((e) => DocumentMeta.fromJson(e as Map<String, dynamic>))
            .toList(),
        breadcrumb: (json['breadcrumb'] as List<dynamic>)
            .map((e) => DocFolder.fromJson(e as Map<String, dynamic>))
            .toList(),
      );
}
