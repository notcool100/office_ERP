import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../models/document.dart';
import '../data/documents_repository.dart';

class DocumentLocation {
  const DocumentLocation({required this.category, this.ownerId, this.folderId});

  final String category;
  final String? ownerId;
  final String? folderId;

  @override
  bool operator ==(Object other) =>
      other is DocumentLocation &&
      other.category == category &&
      other.ownerId == ownerId &&
      other.folderId == folderId;

  @override
  int get hashCode => Object.hash(category, ownerId, folderId);
}

final folderListingProvider =
    FutureProvider.family.autoDispose<FolderListing, DocumentLocation>((ref, location) {
  return ref.watch(documentsRepositoryProvider).browse(
        category: location.category,
        ownerId: location.ownerId,
        folderId: location.folderId,
      );
});
