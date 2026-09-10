import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../../core/utils/formatters.dart';
import '../../../core/widgets/app_list_tile.dart';
import '../../../core/widgets/async_view.dart';
import '../../../core/widgets/section_label.dart';
import '../../../core/widgets/status_chip.dart';
import '../../../models/meeting.dart';
import '../application/meetings_controller.dart';

class MeetingsListScreen extends ConsumerWidget {
  const MeetingsListScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final meetingsAsync = ref.watch(myMeetingsProvider);

    return Scaffold(
      appBar: AppBar(title: const Text('Meetings')),
      body: RefreshIndicator(
        onRefresh: () async => ref.invalidate(myMeetingsProvider),
        child: AsyncView(
          value: meetingsAsync,
          onRetry: () => ref.invalidate(myMeetingsProvider),
          data: (meetings) {
            if (meetings.isEmpty) {
              return ListView(
                children: const [
                  EmptyState(
                    icon: Icons.groups_outlined,
                    title: 'No meetings yet',
                    subtitle: 'Meetings started from a chat channel will show up here.',
                  ),
                ],
              );
            }

            final active = meetings.where((m) => m.isActive).toList();
            final ended = meetings.where((m) => !m.isActive).toList()
              ..sort((a, b) => b.createdAt.compareTo(a.createdAt));

            return ListView(
              padding: const EdgeInsets.symmetric(horizontal: 18),
              children: [
                if (active.isNotEmpty) ...[
                  const SectionLabel('Active now'),
                  AppListCard(
                    children: active.map((m) => _MeetingRow(meeting: m)).toList(),
                  ),
                ],
                const SectionLabel('Past meetings'),
                AppListCard(
                  emptyLabel: 'No past meetings.',
                  children: ended.map((m) => _MeetingRow(meeting: m)).toList(),
                ),
                const SizedBox(height: 12),
              ],
            );
          },
        ),
      ),
    );
  }
}

class _MeetingRow extends StatelessWidget {
  const _MeetingRow({required this.meeting});

  final Meeting meeting;

  @override
  Widget build(BuildContext context) {
    return AppListTile(
      leading: const RowIcon(Icons.groups_rounded),
      title: meeting.title,
      subtitle: meeting.isActive
          ? 'Started ${Formatters.relativeTime(meeting.createdAt)}${meeting.hostName != null ? ' · ${meeting.hostName}' : ''}'
          : '${Formatters.weekdayDayMonth(meeting.createdAt)}${meeting.hostName != null ? ' · ${meeting.hostName}' : ''}',
      trailing: meeting.isActive
          ? const StatusChip('Active', tone: ChipTone.success)
          : const Icon(Icons.chevron_right_rounded, size: 20),
      onTap: () => context.push('/meetings/${meeting.id}'),
    );
  }
}
