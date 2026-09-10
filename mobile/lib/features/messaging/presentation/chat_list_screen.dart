import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../../core/theme/app_colors.dart';
import '../../../core/utils/formatters.dart';
import '../../../core/widgets/app_list_tile.dart';
import '../../../core/widgets/async_view.dart';
import '../../../core/widgets/initials_avatar.dart';
import '../../../core/widgets/section_label.dart';
import '../../../models/messaging.dart';
import '../application/channels_controller.dart';
import 'new_channel_sheet.dart';

class ChatListScreen extends ConsumerWidget {
  const ChatListScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final channelsAsync = ref.watch(channelsControllerProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Messages'),
        actions: [
          IconButton(
            icon: const Icon(Icons.edit_square, size: 20),
            onPressed: () => showNewChannelSheet(context, ref),
          ),
        ],
      ),
      body: RefreshIndicator(
        onRefresh: () => ref.read(channelsControllerProvider.notifier).refresh(),
        child: AsyncView(
          value: channelsAsync,
          onRetry: () => ref.invalidate(channelsControllerProvider),
          data: (channels) {
            if (channels.isEmpty) {
              return ListView(
                children: const [
                  EmptyState(
                    icon: Icons.forum_outlined,
                    title: 'No conversations yet',
                    subtitle: 'Start a channel or direct message with the compose button above.',
                  ),
                ],
              );
            }

            final groups = channels.where((c) => !c.isDirectMessage).toList();
            final directMessages = channels.where((c) => c.isDirectMessage).toList();

            return ListView(
              padding: const EdgeInsets.symmetric(horizontal: 18),
              children: [
                if (groups.isNotEmpty) ...[
                  const SectionLabel('Channels'),
                  AppListCard(children: groups.map((c) => _ChannelRow(channel: c)).toList()),
                ],
                if (directMessages.isNotEmpty) ...[
                  const SectionLabel('Direct messages'),
                  AppListCard(children: directMessages.map((c) => _ChannelRow(channel: c)).toList()),
                ],
                const SizedBox(height: 12),
              ],
            );
          },
        ),
      ),
    );
  }
}

class _ChannelRow extends StatelessWidget {
  const _ChannelRow({required this.channel});

  final Channel channel;

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    return AppListTile(
      leading: channel.isDirectMessage
          ? InitialsAvatar(channel.displayName(), seed: channel.id)
          : Container(
              width: 36,
              height: 36,
              decoration: BoxDecoration(
                color: colors.surface,
                borderRadius: BorderRadius.circular(11),
                border: Border.all(color: colors.borderSoft),
              ),
              alignment: Alignment.center,
              child: Text('#', style: TextStyle(color: colors.accent, fontWeight: FontWeight.w800)),
            ),
      title: channel.displayName(),
      subtitle: channel.description?.isNotEmpty == true ? channel.description : 'Tap to open',
      trailing: Text(
        Formatters.relativeTime(channel.createdAt),
        style: TextStyle(fontSize: 11, color: colors.textFaint, fontFamily: 'monospace'),
      ),
      onTap: () => context.push('/chat/${channel.id}', extra: channel),
    );
  }
}
