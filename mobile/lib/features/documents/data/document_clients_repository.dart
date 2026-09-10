import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/network/api_client.dart';
import '../../../core/network/providers.dart';

class DocumentClient {
  DocumentClient({required this.id, required this.name});
  final String id;
  final String name;

  factory DocumentClient.fromJson(Map<String, dynamic> json) =>
      DocumentClient(id: json['id'] as String, name: json['name'] as String);
}

class DocumentClientsRepository {
  DocumentClientsRepository(this._dio);
  final Dio _dio;

  Future<List<DocumentClient>> list() async {
    try {
      final response = await _dio.get<List<dynamic>>('/mobile/documents/clients');
      return response.data!.map((e) => DocumentClient.fromJson(e as Map<String, dynamic>)).toList();
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }
}

final documentClientsRepositoryProvider = Provider<DocumentClientsRepository>((ref) {
  return DocumentClientsRepository(ref.watch(dioProvider));
});

final documentClientsProvider = FutureProvider.autoDispose<List<DocumentClient>>((ref) {
  return ref.watch(documentClientsRepositoryProvider).list();
});
