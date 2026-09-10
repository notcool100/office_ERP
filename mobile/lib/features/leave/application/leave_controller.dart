import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../models/leave.dart';
import '../data/leave_repository.dart';

final leaveTypesProvider = FutureProvider.autoDispose<List<LeaveType>>((ref) {
  return ref.watch(leaveRepositoryProvider).leaveTypes();
});

final leaveBalanceProvider = FutureProvider.autoDispose<List<LeaveBalance>>((ref) {
  return ref.watch(leaveRepositoryProvider).balance();
});

class MyLeaveRequestsController extends AsyncNotifier<List<LeaveRequest>> {
  @override
  Future<List<LeaveRequest>> build() {
    return ref.read(leaveRepositoryProvider).myRequests();
  }

  Future<void> refresh() async {
    state = await AsyncValue.guard(() => ref.read(leaveRepositoryProvider).myRequests());
  }

  Future<LeaveRequest> submit({
    required String leaveTypeId,
    required DateTime startDate,
    required DateTime endDate,
    String? reason,
  }) async {
    final created = await ref.read(leaveRepositoryProvider).createRequest(
          leaveTypeId: leaveTypeId,
          startDate: startDate,
          endDate: endDate,
          reason: reason,
        );
    state = AsyncValue.data([created, ...state.value ?? []]);
    ref.invalidate(leaveBalanceProvider);
    return created;
  }
}

final myLeaveRequestsProvider = AsyncNotifierProvider<MyLeaveRequestsController, List<LeaveRequest>>(
  MyLeaveRequestsController.new,
);

class LeaveApprovalsController extends AsyncNotifier<List<LeaveRequest>> {
  @override
  Future<List<LeaveRequest>> build() {
    return ref.read(leaveRepositoryProvider).approvals();
  }

  Future<void> refresh() async {
    state = await AsyncValue.guard(() => ref.read(leaveRepositoryProvider).approvals());
  }

  Future<void> approve(String id, {String? notes}) async {
    await ref.read(leaveRepositoryProvider).approve(id, notes: notes);
    state = AsyncValue.data((state.value ?? []).where((r) => r.id != id).toList());
  }

  Future<void> reject(String id, {String? notes}) async {
    await ref.read(leaveRepositoryProvider).reject(id, notes: notes);
    state = AsyncValue.data((state.value ?? []).where((r) => r.id != id).toList());
  }
}

final leaveApprovalsProvider = AsyncNotifierProvider<LeaveApprovalsController, List<LeaveRequest>>(
  LeaveApprovalsController.new,
);
