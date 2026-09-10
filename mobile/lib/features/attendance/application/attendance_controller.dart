import 'dart:convert';

import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../models/attendance.dart';
import '../data/attendance_repository.dart';
import '../data/location_service.dart';

class AttendanceController extends AsyncNotifier<AttendanceRecord?> {
  @override
  Future<AttendanceRecord?> build() {
    return ref.read(attendanceRepositoryProvider).today();
  }

  Future<void> refresh() async {
    state = await AsyncValue.guard(() => ref.read(attendanceRepositoryProvider).today());
  }

  /// Captures a fresh GPS fix and submits it with the selfie already taken
  /// by the camera screen. Throws [ApiException]/[LocationException]
  /// directly so the calling screen can show the message inline instead of
  /// through the AsyncValue error path (a failed check-in shouldn't wipe
  /// the "already checked in" card underneath it).
  Future<AttendanceRecord> checkIn(List<int> imageBytes) async {
    final position = await ref.read(locationServiceProvider).getCurrentPosition();
    final record = await ref.read(attendanceRepositoryProvider).checkIn(
          imageBase64: base64Encode(imageBytes),
          latitude: position.latitude,
          longitude: position.longitude,
        );
    state = AsyncValue.data(record);
    return record;
  }

  Future<AttendanceRecord> checkOut() async {
    final position = await ref.read(locationServiceProvider).getCurrentPosition();
    final record = await ref.read(attendanceRepositoryProvider).checkOut(
          latitude: position.latitude,
          longitude: position.longitude,
        );
    state = AsyncValue.data(record);
    return record;
  }
}

final attendanceControllerProvider = AsyncNotifierProvider<AttendanceController, AttendanceRecord?>(
  AttendanceController.new,
);

final attendanceHistoryProvider = FutureProvider.autoDispose<ListAttendanceResponse>((ref) {
  final now = DateTime.now();
  return ref.read(attendanceRepositoryProvider).records(
        startDate: DateTime(now.year, now.month, 1),
        endDate: now,
      );
});
