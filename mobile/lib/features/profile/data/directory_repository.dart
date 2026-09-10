import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/network/api_client.dart';
import '../../../core/network/providers.dart';
import '../../../models/directory_entry.dart';

class DirectoryRepository {
  DirectoryRepository(this._dio);
  final Dio _dio;

  Future<List<DirectoryEntry>> search(String query) async {
    try {
      final response = await _dio.get<List<dynamic>>(
        '/mobile/directory',
        queryParameters: {if (query.trim().isNotEmpty) 'search': query.trim()},
      );
      return response.data!.map((e) => DirectoryEntry.fromJson(e as Map<String, dynamic>)).toList();
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }
}

final directoryRepositoryProvider = Provider<DirectoryRepository>((ref) {
  return DirectoryRepository(ref.watch(dioProvider));
});

/// Debounced-by-nature via `.family`: Riverpod dedupes identical arguments,
/// and each keystroke's provider is auto-disposed once nothing watches it
/// — good enough for a directory search box without a manual debouncer.
final directoryProvider = FutureProvider.family.autoDispose<List<DirectoryEntry>, String>((ref, query) {
  return ref.watch(directoryRepositoryProvider).search(query);
});
