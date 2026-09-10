import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/network/api_client.dart';
import '../../../core/theme/app_colors.dart';
import '../../../core/utils/formatters.dart';
import '../../../core/widgets/app_list_tile.dart';
import '../../../core/widgets/async_view.dart';
import '../../../core/widgets/initials_avatar.dart';
import '../../../core/widgets/section_label.dart';
import '../../../core/widgets/status_chip.dart';
import '../../../features/auth/application/session_controller.dart';
import '../../../models/leave.dart';
import '../application/leave_controller.dart';

class LeaveScreen extends ConsumerStatefulWidget {
  const LeaveScreen({super.key});

  @override
  ConsumerState<LeaveScreen> createState() => _LeaveScreenState();
}

class _LeaveScreenState extends ConsumerState<LeaveScreen> {
  final _reasonController = TextEditingController();
  String? _leaveTypeId;
  DateTime? _startDate;
  DateTime? _endDate;
  bool _submitting = false;

  @override
  void dispose() {
    _reasonController.dispose();
    super.dispose();
  }

  Future<void> _pickDateRange() async {
    final now = DateTime.now();
    final picked = await showDateRangePicker(
      context: context,
      firstDate: DateTime(now.year - 1),
      lastDate: DateTime(now.year + 2),
      initialDateRange: _startDate != null && _endDate != null
          ? DateTimeRange(start: _startDate!, end: _endDate!)
          : null,
    );
    if (picked != null) {
      setState(() {
        _startDate = picked.start;
        _endDate = picked.end;
      });
    }
  }

  Future<void> _submit() async {
    if (_leaveTypeId == null || _startDate == null || _endDate == null) {
      ScaffoldMessenger.of(context)
          .showSnackBar(const SnackBar(content: Text('Pick a leave type and date range first')));
      return;
    }
    setState(() => _submitting = true);
    try {
      await ref.read(myLeaveRequestsProvider.notifier).submit(
            leaveTypeId: _leaveTypeId!,
            startDate: _startDate!,
            endDate: _endDate!,
            reason: _reasonController.text.trim(),
          );
      _reasonController.clear();
      setState(() {
        _startDate = null;
        _endDate = null;
      });
      if (mounted) {
        ScaffoldMessenger.of(context)
            .showSnackBar(const SnackBar(content: Text('Leave request submitted')));
      }
    } catch (e) {
      final message = e is ApiException ? e.message : e.toString();
      if (mounted) ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
    } finally {
      if (mounted) setState(() => _submitting = false);
    }
  }

  Future<void> _decide(LeaveRequest request, {required bool approve}) async {
    try {
      if (approve) {
        await ref.read(leaveApprovalsProvider.notifier).approve(request.id);
      } else {
        await ref.read(leaveApprovalsProvider.notifier).reject(request.id);
      }
      if (mounted) {
        ScaffoldMessenger.of(context)
            .showSnackBar(SnackBar(content: Text(approve ? 'Leave approved' : 'Leave declined')));
      }
    } catch (e) {
      final message = e is ApiException ? e.message : e.toString();
      if (mounted) ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
    }
  }

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    final canApprove = ref.watch(sessionControllerProvider).value?.capabilities.leaveApprovals ?? false;
    final balanceAsync = ref.watch(leaveBalanceProvider);
    final typesAsync = ref.watch(leaveTypesProvider);
    final historyAsync = ref.watch(myLeaveRequestsProvider);

    return Scaffold(
      appBar: AppBar(title: const Text('Leave')),
      body: RefreshIndicator(
        onRefresh: () async {
          ref.invalidate(leaveBalanceProvider);
          await ref.read(myLeaveRequestsProvider.notifier).refresh();
          if (canApprove) await ref.read(leaveApprovalsProvider.notifier).refresh();
        },
        child: ListView(
          padding: const EdgeInsets.symmetric(horizontal: 18),
          children: [
            const SizedBox(height: 8),
            AsyncView(
              value: balanceAsync,
              onRetry: () => ref.invalidate(leaveBalanceProvider),
              data: (balances) => Row(
                children: [
                  for (final b in balances.take(3)) ...[
                    Expanded(
                      child: _BalanceTile(value: b.remaining.toStringAsFixed(b.remaining % 1 == 0 ? 0 : 1), label: b.leaveTypeName),
                    ),
                    if (b != balances.take(3).last) const SizedBox(width: 8),
                  ],
                ],
              ),
            ),
            if (canApprove) ...[
              const SectionLabel('Awaiting your approval'),
              AsyncView(
                value: ref.watch(leaveApprovalsProvider),
                onRetry: () => ref.read(leaveApprovalsProvider.notifier).refresh(),
                data: (approvals) {
                  if (approvals.isEmpty) {
                    return Padding(
                      padding: const EdgeInsets.symmetric(vertical: 6),
                      child: Text('Nothing pending your review.', style: TextStyle(color: colors.textFaint, fontSize: 12.5)),
                    );
                  }
                  return Column(
                    children: approvals.map((r) => _ApprovalCard(request: r, onDecide: _decide)).toList(),
                  );
                },
              ),
            ],
            const SectionLabel('Apply for leave'),
            Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: colors.surfaceAlt,
                border: Border.all(color: colors.borderSoft),
                borderRadius: BorderRadius.circular(18),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text('Type', style: TextStyle(fontSize: 11.5, fontWeight: FontWeight.w700, color: colors.textMuted)),
                  const SizedBox(height: 6),
                  typesAsync.when(
                    data: (types) => DropdownButtonFormField<String>(
                      initialValue: _leaveTypeId,
                      isExpanded: true,
                      items: types
                          .map((t) => DropdownMenuItem(value: t.id, child: Text(t.name)))
                          .toList(),
                      onChanged: (v) => setState(() => _leaveTypeId = v),
                      decoration: const InputDecoration(),
                    ),
                    loading: () => const LinearProgressIndicator(),
                    error: (e, _) => Text('Could not load leave types', style: TextStyle(color: colors.danger, fontSize: 12)),
                  ),
                  const SizedBox(height: 14),
                  Text('Dates', style: TextStyle(fontSize: 11.5, fontWeight: FontWeight.w700, color: colors.textMuted)),
                  const SizedBox(height: 6),
                  InkWell(
                    onTap: _pickDateRange,
                    borderRadius: BorderRadius.circular(12),
                    child: Container(
                      width: double.infinity,
                      padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 14),
                      decoration: BoxDecoration(
                        color: colors.surface,
                        border: Border.all(color: colors.border),
                        borderRadius: BorderRadius.circular(12),
                      ),
                      child: Row(
                        children: [
                          Icon(Icons.date_range_rounded, size: 17, color: colors.textMuted),
                          const SizedBox(width: 8),
                          Text(
                            _startDate != null && _endDate != null
                                ? Formatters.dateRange(_startDate!, _endDate!)
                                : 'Select date range',
                            style: TextStyle(
                              fontSize: 13,
                              color: _startDate != null ? colors.text : colors.textFaint,
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),
                  const SizedBox(height: 14),
                  Text('Reason', style: TextStyle(fontSize: 11.5, fontWeight: FontWeight.w700, color: colors.textMuted)),
                  const SizedBox(height: 6),
                  TextField(
                    controller: _reasonController,
                    maxLines: 3,
                    decoration: const InputDecoration(hintText: 'Short reason for your manager'),
                  ),
                  const SizedBox(height: 14),
                  SizedBox(
                    width: double.infinity,
                    child: ElevatedButton(
                      onPressed: _submitting ? null : _submit,
                      child: _submitting
                          ? const SizedBox(
                              width: 18, height: 18, child: CircularProgressIndicator(strokeWidth: 2, color: Colors.white))
                          : const Text('Submit Request'),
                    ),
                  ),
                ],
              ),
            ),
            const SectionLabel('History'),
            AsyncView(
              value: historyAsync,
              onRetry: () => ref.read(myLeaveRequestsProvider.notifier).refresh(),
              data: (requests) => AppListCard(
                emptyLabel: 'No leave requests yet.',
                children: requests
                    .map(
                      (r) => AppListTile(
                        title: r.leaveTypeName,
                        subtitle: Formatters.dateRange(r.startDate, r.endDate),
                        trailing: StatusChip.forLeaveStatus(r.status),
                      ),
                    )
                    .toList(),
              ),
            ),
            const SizedBox(height: 12),
          ],
        ),
      ),
    );
  }
}

class _BalanceTile extends StatelessWidget {
  const _BalanceTile({required this.value, required this.label});

  final String value;
  final String label;

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    return Container(
      padding: const EdgeInsets.symmetric(vertical: 13),
      decoration: BoxDecoration(
        color: colors.surfaceAlt,
        border: Border.all(color: colors.borderSoft),
        borderRadius: BorderRadius.circular(15),
      ),
      child: Column(
        children: [
          Text(value, style: TextStyle(fontFamily: 'monospace', fontSize: 19, fontWeight: FontWeight.w600, color: colors.accent)),
          const SizedBox(height: 3),
          Text(label, maxLines: 1, overflow: TextOverflow.ellipsis, style: TextStyle(fontSize: 10, color: colors.textMuted, fontWeight: FontWeight.w600)),
        ],
      ),
    );
  }
}

class _ApprovalCard extends StatelessWidget {
  const _ApprovalCard({required this.request, required this.onDecide});

  final LeaveRequest request;
  final void Function(LeaveRequest request, {required bool approve}) onDecide;

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    return Container(
      margin: const EdgeInsets.only(bottom: 8),
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: colors.surfaceAlt,
        border: Border.all(color: colors.borderSoft),
        borderRadius: BorderRadius.circular(15),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              InitialsAvatar(request.employeeName, size: 34),
              const SizedBox(width: 10),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text('${request.employeeName} · ${request.leaveTypeName}',
                        style: TextStyle(fontSize: 13, fontWeight: FontWeight.w700, color: colors.text)),
                    const SizedBox(height: 1),
                    Text(
                      '${Formatters.dateRange(request.startDate, request.endDate)}${request.reason != null && request.reason!.isNotEmpty ? ' · ${request.reason}' : ''}',
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                      style: TextStyle(fontSize: 11.5, color: colors.textMuted),
                    ),
                  ],
                ),
              ),
            ],
          ),
          const SizedBox(height: 10),
          Row(
            children: [
              Expanded(
                child: OutlinedButton(
                  style: OutlinedButton.styleFrom(backgroundColor: colors.dangerSoft, foregroundColor: colors.danger),
                  onPressed: () => onDecide(request, approve: false),
                  child: const Text('Decline'),
                ),
              ),
              const SizedBox(width: 8),
              Expanded(
                child: OutlinedButton(
                  onPressed: () => onDecide(request, approve: true),
                  child: const Text('Approve'),
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }
}
