import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../../core/network/api_client.dart';
import '../../../core/theme/app_colors.dart';
import '../../../core/widgets/initials_avatar.dart';
import '../../profile/data/directory_repository.dart';
import '../application/channels_controller.dart';

Future<void> showNewChannelSheet(BuildContext context, WidgetRef ref) {
  return showModalBottomSheet(
    context: context,
    isScrollControlled: true,
    builder: (_) => const _NewChannelSheet(),
  );
}

class _NewChannelSheet extends ConsumerStatefulWidget {
  const _NewChannelSheet();

  @override
  ConsumerState<_NewChannelSheet> createState() => _NewChannelSheetState();
}

class _NewChannelSheetState extends ConsumerState<_NewChannelSheet> {
  final _nameController = TextEditingController();
  final _searchController = TextEditingController();
  String _mode = 'dm'; // 'dm' or 'channel'
  bool _busy = false;

  @override
  void dispose() {
    _nameController.dispose();
    _searchController.dispose();
    super.dispose();
  }

  Future<void> _startDirectMessage(String userId, String displayName) async {
    setState(() => _busy = true);
    try {
      final channel = await ref.read(channelsControllerProvider.notifier).createChannel(
            name: displayName,
            isPrivate: true,
            members: [userId],
          );
      if (mounted) {
        Navigator.of(context).pop();
        context.push('/chat/${channel.id}', extra: channel);
      }
    } catch (e) {
      final message = e is ApiException ? e.message : e.toString();
      if (mounted) ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
    } finally {
      if (mounted) setState(() => _busy = false);
    }
  }

  Future<void> _createChannel() async {
    final name = _nameController.text.trim();
    if (name.isEmpty) return;
    setState(() => _busy = true);
    try {
      final channel = await ref.read(channelsControllerProvider.notifier).createChannel(
            name: name,
            isPrivate: false,
          );
      if (mounted) {
        Navigator.of(context).pop();
        context.push('/chat/${channel.id}', extra: channel);
      }
    } catch (e) {
      final message = e is ApiException ? e.message : e.toString();
      if (mounted) ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
    } finally {
      if (mounted) setState(() => _busy = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);

    return Padding(
      padding: EdgeInsets.only(bottom: MediaQuery.of(context).viewInsets.bottom),
      child: DraggableScrollableSheet(
        initialChildSize: 0.7,
        minChildSize: 0.4,
        maxChildSize: 0.92,
        expand: false,
        builder: (context, scrollController) {
          return Padding(
            padding: const EdgeInsets.symmetric(horizontal: 20),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                const SizedBox(height: 16),
                Text('New conversation', style: Theme.of(context).textTheme.titleLarge),
                const SizedBox(height: 14),
                Row(
                  children: [
                    Expanded(
                      child: _ModeButton(
                        label: 'Direct message',
                        selected: _mode == 'dm',
                        onTap: () => setState(() => _mode = 'dm'),
                      ),
                    ),
                    const SizedBox(width: 8),
                    Expanded(
                      child: _ModeButton(
                        label: 'New channel',
                        selected: _mode == 'channel',
                        onTap: () => setState(() => _mode = 'channel'),
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 16),
                if (_mode == 'channel') ...[
                  TextField(
                    controller: _nameController,
                    decoration: const InputDecoration(labelText: 'Channel name'),
                  ),
                  const SizedBox(height: 14),
                  ElevatedButton(
                    onPressed: _busy ? null : _createChannel,
                    child: Text(_busy ? 'Creating…' : 'Create channel'),
                  ),
                ] else ...[
                  TextField(
                    controller: _searchController,
                    onChanged: (_) => setState(() {}),
                    decoration: const InputDecoration(
                      labelText: 'Search people',
                      prefixIcon: Icon(Icons.search_rounded, size: 20),
                    ),
                  ),
                  const SizedBox(height: 10),
                  Expanded(
                    child: Consumer(
                      builder: (context, ref, _) {
                        final asyncDirectory = ref.watch(directoryProvider(_searchController.text));
                        return asyncDirectory.when(
                          data: (entries) => ListView.builder(
                            controller: scrollController,
                            itemCount: entries.length,
                            itemBuilder: (context, index) {
                              final e = entries[index];
                              return ListTile(
                                contentPadding: EdgeInsets.zero,
                                leading: InitialsAvatar(e.displayName, seed: e.userId),
                                title: Text(e.displayName, style: const TextStyle(fontSize: 13.5, fontWeight: FontWeight.w600)),
                                subtitle: Text(
                                  [e.position, e.department].where((s) => s != null && s.isNotEmpty).join(' · '),
                                  style: TextStyle(fontSize: 11.5, color: colors.textMuted),
                                ),
                                onTap: _busy ? null : () => _startDirectMessage(e.userId, e.displayName),
                              );
                            },
                          ),
                          loading: () => const Center(child: CircularProgressIndicator(strokeWidth: 2)),
                          error: (e, _) => Center(child: Text(e.toString())),
                        );
                      },
                    ),
                  ),
                ],
              ],
            ),
          );
        },
      ),
    );
  }
}

class _ModeButton extends StatelessWidget {
  const _ModeButton({required this.label, required this.selected, required this.onTap});

  final String label;
  final bool selected;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(12),
      child: Container(
        padding: const EdgeInsets.symmetric(vertical: 11),
        decoration: BoxDecoration(
          color: selected ? colors.accentSoft : colors.surfaceAlt,
          border: Border.all(color: selected ? colors.accent : colors.borderSoft),
          borderRadius: BorderRadius.circular(12),
        ),
        alignment: Alignment.center,
        child: Text(
          label,
          style: TextStyle(
            fontSize: 12.5,
            fontWeight: FontWeight.w700,
            color: selected ? colors.accent : colors.textMuted,
          ),
        ),
      ),
    );
  }
}
