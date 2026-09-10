import 'package:flutter/material.dart';
import 'package:flutter_html/flutter_html.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/network/api_client.dart';
import '../../../core/theme/app_colors.dart';
import '../../../core/utils/formatters.dart';
import '../../../core/widgets/app_list_tile.dart';
import '../../../core/widgets/async_view.dart';
import '../../../core/widgets/initials_avatar.dart';
import '../../../core/widgets/status_chip.dart';
import '../../../features/auth/application/session_controller.dart';
import '../application/meetings_controller.dart';
import '../data/meetings_repository.dart';

class MeetingDetailScreen extends ConsumerStatefulWidget {
  const MeetingDetailScreen({super.key, required this.meetingId});

  final String meetingId;

  @override
  ConsumerState<MeetingDetailScreen> createState() => _MeetingDetailScreenState();
}

class _MeetingDetailScreenState extends ConsumerState<MeetingDetailScreen>
    with SingleTickerProviderStateMixin {
  late TabController _tabController;
  bool _busy = false;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 3, vsync: this);
  }

  @override
  void dispose() {
    _tabController.dispose();
    super.dispose();
  }

  void _invalidateAll() {
    ref.invalidate(meetingDetailProvider(widget.meetingId));
    ref.invalidate(meetingParticipantsProvider(widget.meetingId));
  }

  Future<void> _act(Future<void> Function() action, {required String successMessage}) async {
    setState(() => _busy = true);
    try {
      await action();
      _invalidateAll();
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(successMessage)));
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
    final meetingAsync = ref.watch(meetingDetailProvider(widget.meetingId));
    final myUserId = ref.watch(sessionControllerProvider).value?.user.id;

    return Scaffold(
      appBar: AppBar(
        title: meetingAsync.value != null ? Text(meetingAsync.value!.title) : const Text('Meeting'),
      ),
      body: AsyncView(
        value: meetingAsync,
        onRetry: () => ref.invalidate(meetingDetailProvider(widget.meetingId)),
        data: (meeting) {
          final isHost = meeting.hostId == myUserId;

          return Column(
            children: [
              Padding(
                padding: const EdgeInsets.fromLTRB(18, 12, 18, 4),
                child: Container(
                  padding: const EdgeInsets.all(14),
                  decoration: BoxDecoration(
                    color: colors.surfaceAlt,
                    border: Border.all(color: colors.borderSoft),
                    borderRadius: BorderRadius.circular(16),
                  ),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Row(
                        children: [
                          Icon(Icons.schedule_rounded, size: 14, color: colors.textMuted),
                          const SizedBox(width: 6),
                          Text(
                            meeting.isActive
                                ? 'Started ${Formatters.relativeTime(meeting.createdAt)}'
                                : Formatters.weekdayDayMonth(meeting.createdAt),
                            style: TextStyle(fontSize: 12, color: colors.textMuted),
                          ),
                          const Spacer(),
                          StatusChip(
                            meeting.isActive ? 'Active' : 'Ended',
                            tone: meeting.isActive ? ChipTone.success : ChipTone.neutral,
                          ),
                        ],
                      ),
                      const SizedBox(height: 12),
                      if (meeting.isActive)
                        Row(
                          children: [
                            Expanded(
                              child: ElevatedButton.icon(
                                onPressed: _busy
                                    ? null
                                    : () => _act(
                                          () => ref.read(meetingsRepositoryProvider).join(meeting.id),
                                          successMessage: 'Joined meeting',
                                        ),
                                icon: const Icon(Icons.login_rounded, size: 17),
                                label: const Text('Join'),
                              ),
                            ),
                            const SizedBox(width: 8),
                            if (isHost)
                              Expanded(
                                child: OutlinedButton(
                                  style: OutlinedButton.styleFrom(
                                      backgroundColor: colors.dangerSoft, foregroundColor: colors.danger),
                                  onPressed: _busy
                                      ? null
                                      : () => _act(
                                            () => ref.read(meetingsRepositoryProvider).end(meeting.id),
                                            successMessage: 'Meeting ended',
                                          ),
                                  child: const Text('End meeting'),
                                ),
                              )
                            else
                              Expanded(
                                child: OutlinedButton(
                                  onPressed: _busy
                                      ? null
                                      : () => _act(
                                            () => ref.read(meetingsRepositoryProvider).leave(meeting.id),
                                            successMessage: 'Left meeting',
                                          ),
                                  child: const Text('Leave'),
                                ),
                              ),
                          ],
                        ),
                    ],
                  ),
                ),
              ),
              TabBar(
                controller: _tabController,
                labelColor: colors.accent,
                unselectedLabelColor: colors.textFaint,
                indicatorColor: colors.accent,
                labelStyle: const TextStyle(fontWeight: FontWeight.w700, fontSize: 13),
                tabs: const [
                  Tab(text: 'Participants'),
                  Tab(text: 'Minutes'),
                  Tab(text: 'Files'),
                ],
              ),
              Expanded(
                child: TabBarView(
                  controller: _tabController,
                  children: [
                    _ParticipantsTab(meetingId: widget.meetingId),
                    _MinutesTab(meetingId: widget.meetingId),
                    _AttachmentsTab(meetingId: widget.meetingId),
                  ],
                ),
              ),
            ],
          );
        },
      ),
    );
  }
}

class _ParticipantsTab extends ConsumerWidget {
  const _ParticipantsTab({required this.meetingId});
  final String meetingId;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final asyncParticipants = ref.watch(meetingParticipantsProvider(meetingId));
    return AsyncView(
      value: asyncParticipants,
      onRetry: () => ref.invalidate(meetingParticipantsProvider(meetingId)),
      data: (participants) {
        if (participants.isEmpty) {
          return const EmptyState(icon: Icons.people_outline, title: 'No participants yet');
        }
        return ListView.builder(
          padding: const EdgeInsets.symmetric(horizontal: 18, vertical: 10),
          itemCount: participants.length,
          itemBuilder: (context, index) {
            final p = participants[index];
            return AppListTile(
              leading: InitialsAvatar(p.displayName, seed: p.userId),
              title: p.displayName,
              subtitle: p.email,
              trailing: p.role == 'host' ? const StatusChip('Host', tone: ChipTone.accent) : null,
            );
          },
        );
      },
    );
  }
}

class _MinutesTab extends ConsumerWidget {
  const _MinutesTab({required this.meetingId});
  final String meetingId;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final colors = AppColors.of(context);
    final asyncMinutes = ref.watch(meetingMinutesProvider(meetingId));

    return AsyncView(
      value: asyncMinutes,
      onRetry: () => ref.invalidate(meetingMinutesProvider(meetingId)),
      data: (minutes) {
        if (minutes.content.trim().isEmpty || minutes.content.trim() == '<p></p>') {
          return const EmptyState(
            icon: Icons.description_outlined,
            title: 'No minutes yet',
            subtitle: 'Minutes taken on the web app will appear here.',
          );
        }
        return SingleChildScrollView(
          padding: const EdgeInsets.all(18),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              if (minutes.updatedByName != null)
                Padding(
                  padding: const EdgeInsets.only(bottom: 10),
                  child: Text(
                    'Last updated by ${minutes.updatedByName} · ${Formatters.relativeTime(minutes.updatedAt)}',
                    style: TextStyle(fontSize: 11.5, color: colors.textFaint),
                  ),
                ),
              Html(
                data: minutes.content,
                style: {
                  'body': Style(
                    margin: Margins.zero,
                    fontSize: FontSize(13.5),
                    color: colors.text,
                  ),
                },
              ),
            ],
          ),
        );
      },
    );
  }
}

class _AttachmentsTab extends ConsumerWidget {
  const _AttachmentsTab({required this.meetingId});
  final String meetingId;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final asyncAttachments = ref.watch(meetingAttachmentsProvider(meetingId));
    return AsyncView(
      value: asyncAttachments,
      onRetry: () => ref.invalidate(meetingAttachmentsProvider(meetingId)),
      data: (attachments) {
        if (attachments.isEmpty) {
          return const EmptyState(icon: Icons.attach_file_rounded, title: 'No files attached');
        }
        return ListView.builder(
          padding: const EdgeInsets.symmetric(horizontal: 18, vertical: 10),
          itemCount: attachments.length,
          itemBuilder: (context, index) {
            final a = attachments[index];
            return AppListTile(
              leading: RowIcon(
                a.category == 'recording' ? Icons.videocam_outlined : Icons.insert_drive_file_outlined,
              ),
              title: a.fileName,
              subtitle: '${Formatters.fileSize(a.fileSize)} · ${a.uploadedByName ?? 'Unknown'}',
              trailing: const Icon(Icons.download_rounded, size: 18),
            );
          },
        );
      },
    );
  }
}
