import 'dart:io';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:open_filex/open_filex.dart';
import 'package:path_provider/path_provider.dart';

import '../data/documents_repository.dart';

/// Downloads a document through the authenticated API and hands it to the
/// OS's own viewer via `open_filex` — there is no in-app PDF/DOCX/XLSX
/// renderer, and re-implementing one is out of scope for a view-only
/// documents tab.
class DocumentOpener {
  DocumentOpener(this._repository);
  final DocumentsRepository _repository;

  Future<void> open({
    required String documentId,
    required String category,
    String? ownerId,
    required String fileName,
  }) async {
    final response = await _repository.downloadBytes(
      documentId: documentId,
      category: category,
      ownerId: ownerId,
    );

    final dir = await getTemporaryDirectory();
    final safeName = fileName.replaceAll(RegExp(r'[\\/]'), '_');
    final file = File('${dir.path}/$documentId-$safeName');
    await file.writeAsBytes(response.data!, flush: true);

    final result = await OpenFilex.open(file.path);
    if (result.type != ResultType.done) {
      throw Exception(result.message.isNotEmpty ? result.message : 'Could not open this file.');
    }
  }
}

final documentOpenerProvider = Provider<DocumentOpener>((ref) {
  return DocumentOpener(ref.watch(documentsRepositoryProvider));
});
