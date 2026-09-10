import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/network/api_client.dart';
import '../../../core/theme/app_colors.dart';
import '../../../core/utils/formatters.dart';
import '../../../core/widgets/async_view.dart';
import '../../../features/auth/application/session_controller.dart';
import '../../../models/document.dart';
import '../application/document_opener.dart';
import '../application/documents_controller.dart';
import '../data/document_clients_repository.dart';

class DocumentsScreen extends ConsumerStatefulWidget {
  const DocumentsScreen({super.key});

  @override
  ConsumerState<DocumentsScreen> createState() => _DocumentsScreenState();
}

/// One entry in the navigation stack this screen keeps internally —
/// folders can nest arbitrarily deep, so rather than model every depth as
/// a route, the screen just pushes/pops locations onto its own stack and
/// rebuilds in place, à la a file manager.
class _Crumb {
  const _Crumb({required this.folderId, required this.name});
  final String? folderId;
  final String name;
}

class _DocumentsScreenState extends ConsumerState<DocumentsScreen> {
  String _category = 'company';
  String? _clientId;
  String? _clientName;
  final List<_Crumb> _stack = [const _Crumb(folderId: null, name: 'Home')];

  void _resetStack() {
    setState(() => _stack
      ..clear()
      ..add(const _Crumb(folderId: null, name: 'Home')));
  }

  void _openFolder(DocFolder folder) {
    setState(() => _stack.add(_Crumb(folderId: folder.id, name: folder.name)));
  }

  void _popTo(int index) {
    setState(() => _stack.removeRange(index + 1, _stack.length));
  }

  Future<void> _openDocument(DocumentMeta doc) async {
    final messenger = ScaffoldMessenger.of(context);
    messenger.showSnackBar(const SnackBar(content: Text('Opening…'), duration: Duration(seconds: 1)));
    try {
      await ref.read(documentOpenerProvider).open(
            documentId: doc.id,
            category: _category,
            ownerId: _clientId,
            fileName: doc.fileName,
          );
    } catch (e) {
      final message = e is ApiException ? e.message : e.toString();
      messenger.showSnackBar(SnackBar(content: Text(message)));
    }
  }

  @override
  Widget build(BuildContext context) {
    final session = ref.watch(sessionControllerProvider);
    final capabilities = session.value?.capabilities;

    return Scaffold(
      appBar: AppBar(title: const Text('Documents')),
      body: AsyncView(
        value: session,
        data: (_) {
          final canCompany = capabilities?.companyDocuments ?? false;
          final canClient = capabilities?.clientDocuments ?? false;

          if (!canCompany && !canClient) {
            return const EmptyState(
              icon: Icons.folder_off_outlined,
              title: 'No document access',
              subtitle: 'Ask an administrator to grant you access to company or client documents.',
            );
          }

          return Column(
            children: [
              if (canCompany && canClient)
                Padding(
                  padding: const EdgeInsets.fromLTRB(18, 12, 18, 0),
                  child: _CategoryTabs(
                    value: _category,
                    onChanged: (v) {
                      setState(() {
                        _category = v;
                        _clientId = null;
                        _clientName = null;
                      });
                      _resetStack();
                    },
                  ),
                ),
              if (_stack.length > 1 || _clientName != null)
                Padding(
                  padding: const EdgeInsets.fromLTRB(18, 10, 18, 0),
                  child: _Breadcrumb(
                    rootLabel: _clientName ?? (_category == 'company' ? 'Company documents' : 'Client documents'),
                    crumbs: _stack,
                    onTapRoot: () {
                      if (_clientName != null && _stack.length == 1) {
                        setState(() {
                          _clientId = null;
                          _clientName = null;
                        });
                      } else {
                        _popTo(0);
                      }
                    },
                    onTapCrumb: _popTo,
                  ),
                ),
              Expanded(
                child: _category == 'client' && _clientId == null
                    ? _ClientPicker(
                        onSelected: (id, name) => setState(() {
                          _clientId = id;
                          _clientName = name;
                          _stack
                            ..clear()
                            ..add(const _Crumb(folderId: null, name: 'Home'));
                        }),
                      )
                    : _FolderContents(
                        category: _category,
                        ownerId: _clientId,
                        folderId: _stack.last.folderId,
                        onOpenFolder: _openFolder,
                        onOpenDocument: _openDocument,
                      ),
              ),
            ],
          );
        },
      ),
    );
  }
}

class _ClientPicker extends ConsumerWidget {
  const _ClientPicker({required this.onSelected});

  final void Function(String id, String name) onSelected;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final asyncClients = ref.watch(documentClientsProvider);

    return AsyncView(
      value: asyncClients,
      onRetry: () => ref.invalidate(documentClientsProvider),
      data: (clients) {
        if (clients.isEmpty) {
          return const EmptyState(icon: Icons.business_outlined, title: 'No clients yet');
        }
        return ListView.builder(
          padding: const EdgeInsets.symmetric(horizontal: 18, vertical: 10),
          itemCount: clients.length,
          itemBuilder: (context, index) {
            final c = clients[index];
            return ListTile(
              contentPadding: EdgeInsets.zero,
              leading: const Icon(Icons.business_rounded),
              title: Text(c.name, style: const TextStyle(fontSize: 13.5, fontWeight: FontWeight.w600)),
              trailing: const Icon(Icons.chevron_right_rounded, size: 20),
              onTap: () => onSelected(c.id, c.name),
            );
          },
        );
      },
    );
  }
}

class _CategoryTabs extends StatelessWidget {
  const _CategoryTabs({required this.value, required this.onChanged});
  final String value;
  final ValueChanged<String> onChanged;

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    Widget seg(String key, String label) {
      final selected = value == key;
      return Expanded(
        child: InkWell(
          onTap: () => onChanged(key),
          borderRadius: BorderRadius.circular(9),
          child: Container(
            padding: const EdgeInsets.symmetric(vertical: 8),
            decoration: BoxDecoration(color: selected ? colors.surface : null, borderRadius: BorderRadius.circular(9)),
            alignment: Alignment.center,
            child: Text(label, style: TextStyle(fontSize: 12.5, fontWeight: FontWeight.w700, color: selected ? colors.accent : colors.textMuted)),
          ),
        ),
      );
    }

    return Container(
      padding: const EdgeInsets.all(3),
      decoration: BoxDecoration(color: colors.surfaceAlt, border: Border.all(color: colors.borderSoft), borderRadius: BorderRadius.circular(12)),
      child: Row(children: [seg('company', 'Company'), seg('client', 'Client')]),
    );
  }
}

class _Breadcrumb extends StatelessWidget {
  const _Breadcrumb({
    required this.rootLabel,
    required this.crumbs,
    required this.onTapRoot,
    required this.onTapCrumb,
  });

  final String rootLabel;
  final List<_Crumb> crumbs;
  final VoidCallback onTapRoot;
  final void Function(int index) onTapCrumb;

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    return SingleChildScrollView(
      scrollDirection: Axis.horizontal,
      child: Row(
        children: [
          _crumbButton(context, rootLabel, colors.accent, onTapRoot),
          for (int i = 1; i < crumbs.length; i++) ...[
            Icon(Icons.chevron_right_rounded, size: 15, color: colors.textFaint),
            _crumbButton(
              context,
              crumbs[i].name,
              i == crumbs.length - 1 ? colors.text : colors.accent,
              () => onTapCrumb(i),
            ),
          ],
        ],
      ),
    );
  }

  Widget _crumbButton(BuildContext context, String label, Color color, VoidCallback onTap) {
    return InkWell(
      onTap: onTap,
      child: Padding(
        padding: const EdgeInsets.symmetric(vertical: 6, horizontal: 4),
        child: Text(label, style: TextStyle(fontSize: 12.5, fontWeight: FontWeight.w700, color: color)),
      ),
    );
  }
}

class _FolderContents extends ConsumerWidget {
  const _FolderContents({
    required this.category,
    required this.ownerId,
    required this.folderId,
    required this.onOpenFolder,
    required this.onOpenDocument,
  });

  final String category;
  final String? ownerId;
  final String? folderId;
  final void Function(DocFolder folder) onOpenFolder;
  final void Function(DocumentMeta doc) onOpenDocument;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final location = DocumentLocation(category: category, ownerId: ownerId, folderId: folderId);
    final asyncListing = ref.watch(folderListingProvider(location));

    return AsyncView(
      value: asyncListing,
      onRetry: () => ref.invalidate(folderListingProvider(location)),
      data: (listing) {
        if (listing.folders.isEmpty && listing.documents.isEmpty) {
          return const EmptyState(icon: Icons.folder_open_outlined, title: 'This folder is empty');
        }
        return ListView(
          padding: const EdgeInsets.fromLTRB(18, 10, 18, 20),
          children: [
            if (listing.folders.isNotEmpty)
              GridView.count(
                shrinkWrap: true,
                physics: const NeverScrollableScrollPhysics(),
                crossAxisCount: 2,
                mainAxisSpacing: 10,
                crossAxisSpacing: 10,
                childAspectRatio: 2.4,
                children: listing.folders
                    .map((f) => _FolderCard(folder: f, onTap: () => onOpenFolder(f)))
                    .toList(),
              ),
            if (listing.documents.isNotEmpty) ...[
              const SizedBox(height: 14),
              ...listing.documents.map((doc) => _DocumentRow(doc: doc, onTap: () => onOpenDocument(doc))),
            ],
          ],
        );
      },
    );
  }
}

class _FolderCard extends StatelessWidget {
  const _FolderCard({required this.folder, required this.onTap});
  final DocFolder folder;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(16),
      child: Container(
        padding: const EdgeInsets.all(14),
        decoration: BoxDecoration(color: colors.surfaceAlt, border: Border.all(color: colors.borderSoft), borderRadius: BorderRadius.circular(16)),
        child: Row(
          children: [
            Container(
              width: 38,
              height: 38,
              decoration: BoxDecoration(color: colors.accentSoft, borderRadius: BorderRadius.circular(11)),
              alignment: Alignment.center,
              child: Icon(Icons.folder_rounded, color: colors.accent, size: 19),
            ),
            const SizedBox(width: 10),
            Expanded(
              child: Text(folder.name, maxLines: 2, overflow: TextOverflow.ellipsis, style: TextStyle(fontSize: 12.5, fontWeight: FontWeight.w700, color: colors.text)),
            ),
          ],
        ),
      ),
    );
  }
}

class _DocumentRow extends StatelessWidget {
  const _DocumentRow({required this.doc, required this.onTap});
  final DocumentMeta doc;
  final VoidCallback onTap;

  IconData get _icon {
    if (doc.isImage) return Icons.image_outlined;
    final ext = doc.fileName.split('.').last.toLowerCase();
    switch (ext) {
      case 'pdf':
        return Icons.picture_as_pdf_outlined;
      case 'xlsx':
      case 'xls':
      case 'csv':
        return Icons.table_chart_outlined;
      case 'doc':
      case 'docx':
        return Icons.description_outlined;
      case 'zip':
        return Icons.folder_zip_outlined;
      default:
        return Icons.insert_drive_file_outlined;
    }
  }

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    return InkWell(
      onTap: onTap,
      child: Padding(
        padding: const EdgeInsets.symmetric(vertical: 9),
        child: Row(
          children: [
            Container(
              width: 36,
              height: 36,
              decoration: BoxDecoration(color: colors.surfaceAlt, border: Border.all(color: colors.borderSoft), borderRadius: BorderRadius.circular(11)),
              alignment: Alignment.center,
              child: Icon(_icon, size: 18, color: colors.accent),
            ),
            const SizedBox(width: 11),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(doc.title, maxLines: 1, overflow: TextOverflow.ellipsis, style: TextStyle(fontSize: 13, fontWeight: FontWeight.w600, color: colors.text)),
                  Text(
                    '${Formatters.fileSize(doc.fileSize)} · ${Formatters.dayMonth(doc.createdAt)}',
                    style: TextStyle(fontSize: 11.5, color: colors.textMuted),
                  ),
                ],
              ),
            ),
            Icon(Icons.chevron_right_rounded, size: 20, color: colors.textFaint),
          ],
        ),
      ),
    );
  }
}
