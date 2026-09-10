import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/network/api_client.dart';
import '../../../core/network/providers.dart';
import '../../../models/document.dart';

class DocumentsRepository {
  DocumentsRepository(this._dio);
  final Dio _dio;

  Future<FolderListing> browse({
    required String category,
    String? ownerId,
    String? folderId,
  }) async {
    try {
      final response = await _dio.get<Map<String, dynamic>>(
        '/mobile/documents',
        queryParameters: {
          'category': category,
          if (ownerId != null) 'ownerId': ownerId,
          if (folderId != null) 'folderId': folderId,
        },
      );
      return FolderListing.fromJson(response.data!);
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  /// Downloads a document's bytes to hand to `open_filex` — the endpoint
  /// requires the app's own bearer token, so it can't just be opened as a
  /// plain URL.
  Future<Response<List<int>>> downloadBytes({
    required String documentId,
    required String category,
    String? ownerId,
  }) async {
    try {
      return await _dio.get<List<int>>(
        '/mobile/documents/$documentId/file',
        queryParameters: {
          'category': category,
          if (ownerId != null) 'ownerId': ownerId,
        },
        options: Options(responseType: ResponseType.bytes),
      );
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }
}

final documentsRepositoryProvider = Provider<DocumentsRepository>((ref) {
  return DocumentsRepository(ref.watch(dioProvider));
});
