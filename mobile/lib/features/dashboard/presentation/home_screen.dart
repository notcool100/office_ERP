import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../../core/theme/app_colors.dart';
import '../../../core/utils/formatters.dart';
import '../../../core/widgets/app_list_tile.dart';
import '../../../core/widgets/async_view.dart';
import '../../../core/widgets/initials_avatar.dart';
import '../../../core/widgets/section_label.dart';
import '../../../features/notifications/application/notifications_provider.dart';
import '../../../models/dashboard.dart';
import '../data/dashboard_repository.dart';

class HomeScreen extends ConsumerWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final colors = AppColors.of(context);
    final dashboardAsync = ref.watch(dashboardProvider);
    final unread = ref.watch(unreadNotificationCountProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Adya'),
        actions: [
          Stack(
            children: [
              IconButton(
                icon: const Icon(Icons.notifications_none_rounded),
                onPressed: () => context.push('/notifications'),
              ),
              if (unread > 0)
                Positioned(
                  right: 8,
                  top: 8,
                  child: Container(
                    padding: const EdgeInsets.all(3),
                    decoration: BoxDecoration(color: colors.danger, shape: BoxShape.circle),
                    constraints: const BoxConstraints(minWidth: 14, minHeight: 14),
                  ),
                ),
            ],
          ),
        ],
      ),
      body: RefreshIndicator(
        onRefresh: () async => ref.invalidate(dashboardProvider),
        child: AsyncView(
          value: dashboardAsync,
          onRetry: () => ref.invalidate(dashboardProvider),
          data: (dashboard) => ListView(
            padding: const EdgeInsets.symmetric(horizontal: 18),
            children: [
              const SizedBox(height: 6),
              Text('Good ${_greetingWord()}, ${dashboard.greetingName}', style: Theme.of(context).textTheme.headlineSmall),
              const SizedBox(height: 2),
              Text(Formatters.fullDate(DateTime.now()), style: TextStyle(fontSize: 12.5, color: colors.textMuted)),
              const SizedBox(height: 16),
              _AttendanceCard(dashboard: dashboard),
              const SectionLabel("Today's schedule"),
              AppListCard(
                emptyLabel: 'Nothing scheduled today.',
                children: [
                  for (final m in dashboard.meetings.take(3))
                    AppListTile(
                      leading: const RowIcon(Icons.groups_rounded),
                      title: m.title,
                      subtitle: m.isActive ? 'Active now' : Formatters.time(m.createdAt),
                      onTap: () => context.push('/meetings/${m.id}'),
                    ),
                  for (final e in dashboard.events)
                    AppListTile(
                      leading: RowIcon(e.allDay ? Icons.event_rounded : Icons.calendar_month_rounded),
                      title: e.title,
                      subtitle: e.allDay
                          ? 'All day · ${_scopeLabel(e.scope)}'
                          : '${Formatters.time(e.startAt)} – ${Formatters.time(e.endAt)} · ${_scopeLabel(e.scope)}',
                    ),
                ],
              ),
              if (dashboard.pendingApprovals.isNotEmpty) ...[
                const SectionLabel('Awaiting your approval'),
                AppListCard(
                  children: dashboard.pendingApprovals
                      .map(
                        (r) => AppListTile(
                          leading: InitialsAvatar(r.employeeName, seed: r.employeeId),
                          title: '${r.employeeName} · ${r.leaveTypeName}',
                          subtitle: Formatters.dateRange(r.startDate, r.endDate),
                          onTap: () => context.push('/leave'),
                        ),
                      )
                      .toList(),
                ),
              ],
              const SectionLabel('Quick actions'),
              _QuickActions(),
              const SizedBox(height: 12),
            ],
          ),
        ),
      ),
    );
  }

  String _greetingWord() {
    final hour = DateTime.now().hour;
    if (hour < 12) return 'morning';
    if (hour < 17) return 'afternoon';
    return 'evening';
  }

  String _scopeLabel(String scope) {
    switch (scope) {
      case 'company':
        return 'Company';
      case 'department':
        return 'Department';
      default:
        return 'Personal';
    }
  }
}

class _AttendanceCard extends StatelessWidget {
  const _AttendanceCard({required this.dashboard});
  final DashboardResponse dashboard;

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    final attendance = dashboard.attendance;
    final checkedIn = attendance?.isCheckedIn ?? false;
    final complete = attendance?.isComplete ?? false;

    String label;
    String time;
    String sub;
    if (complete) {
      label = 'Completed';
      time = attendance!.totalHours != null ? Formatters.durationHm(attendance.totalHours!) : '—';
      sub = 'Checked out at ${Formatters.time(attendance.checkOut!)}';
    } else if (checkedIn) {
      label = 'Checked in';
      time = Formatters.time(attendance!.checkIn!);
      sub = 'Tap Attendance to check out';
    } else {
      label = 'Not checked in';
      time = '—';
      sub = 'Tap Attendance to check in';
    }

    return Container(
      padding: const EdgeInsets.all(18),
      decoration: BoxDecoration(
        gradient: LinearGradient(begin: Alignment.topLeft, end: Alignment.bottomRight, colors: [colors.accentHover, colors.accent]),
        borderRadius: BorderRadius.circular(20),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(label.toUpperCase(), style: const TextStyle(color: Colors.white70, fontSize: 11, fontWeight: FontWeight.w700, letterSpacing: 0.6)),
                    const SizedBox(height: 5),
                    Text(time, style: const TextStyle(color: Colors.white, fontSize: 26, fontWeight: FontWeight.w600, fontFamily: 'monospace')),
                    const SizedBox(height: 3),
                    Row(
                      children: [
                        const Icon(Icons.location_on_rounded, size: 12, color: Colors.white70),
                        const SizedBox(width: 4),
                        Flexible(child: Text(sub, style: const TextStyle(color: Colors.white70, fontSize: 11.5), overflow: TextOverflow.ellipsis)),
                      ],
                    ),
                  ],
                ),
              ),
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 9, vertical: 4),
                decoration: BoxDecoration(color: Colors.white.withValues(alpha: 0.2), borderRadius: BorderRadius.circular(99)),
                child: Text(
                  complete ? 'Done' : (checkedIn ? 'In progress' : 'Pending'),
                  style: const TextStyle(color: Colors.white, fontSize: 10.5, fontWeight: FontWeight.w700),
                ),
              ),
            ],
          ),
          const SizedBox(height: 14),
          SizedBox(
            width: double.infinity,
            child: OutlinedButton(
              style: OutlinedButton.styleFrom(
                backgroundColor: Colors.white.withValues(alpha: 0.16),
                foregroundColor: Colors.white,
                side: BorderSide(color: Colors.white.withValues(alpha: 0.3)),
              ),
              onPressed: () => GoRouter.of(context).go('/attendance'),
              child: const Text('Go to Attendance →'),
            ),
          ),
        ],
      ),
    );
  }
}

class _QuickActions extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final items = [
      (Icons.work_history_rounded, 'Apply Leave', '/leave'),
      (Icons.chat_bubble_rounded, 'Messages', '/chat'),
      (Icons.folder_rounded, 'Documents', '/documents'),
      (Icons.calendar_month_rounded, 'Calendar', '/calendar'),
    ];

    return GridView.count(
      shrinkWrap: true,
      physics: const NeverScrollableScrollPhysics(),
      crossAxisCount: 4,
      mainAxisSpacing: 8,
      crossAxisSpacing: 8,
      childAspectRatio: 0.85,
      children: items
          .map(
            (item) => _QuickAction(icon: item.$1, label: item.$2, onTap: () => GoRouter.of(context).go(item.$3)),
          )
          .toList(),
    );
  }
}

class _QuickAction extends StatelessWidget {
  const _QuickAction({required this.icon, required this.label, required this.onTap});
  final IconData icon;
  final String label;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(15),
      child: Container(
        decoration: BoxDecoration(color: colors.surfaceAlt, border: Border.all(color: colors.borderSoft), borderRadius: BorderRadius.circular(15)),
        padding: const EdgeInsets.symmetric(vertical: 10),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Container(
              width: 34,
              height: 34,
              decoration: BoxDecoration(color: colors.surface, borderRadius: BorderRadius.circular(10)),
              alignment: Alignment.center,
              child: Icon(icon, size: 17, color: colors.accent),
            ),
            const SizedBox(height: 6),
            Text(label, textAlign: TextAlign.center, style: TextStyle(fontSize: 10, fontWeight: FontWeight.w700, color: colors.text)),
          ],
        ),
      ),
    );
  }
}
