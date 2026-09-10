import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/network/api_client.dart';
import '../../../core/network/providers.dart';
import '../../../models/attendance.dart';

class AttendanceRepository {
  AttendanceRepository(this._dio);
  final Dio _dio;

  Future<AttendanceRecord?> today() async {
    try {
      final response = await _dio.get<dynamic>('/mobile/attendance/today');
      if (response.data == null) return null;
      return AttendanceRecord.fromJson(response.data as Map<String, dynamic>);
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<ListAttendanceResponse> records({DateTime? startDate, DateTime? endDate}) async {
    try {
      final response = await _dio.get<Map<String, dynamic>>(
        '/mobile/attendance/records',
        queryParameters: {
          if (startDate != null) 'startDate': _dateOnly(startDate),
          if (endDate != null) 'endDate': _dateOnly(endDate),
        },
      );
      return ListAttendanceResponse.fromJson(response.data!);
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<AttendanceRecord> checkIn({
    required String imageBase64,
    required double latitude,
    required double longitude,
    String? notes,
  }) async {
    try {
      final response = await _dio.post<Map<String, dynamic>>(
        '/mobile/attendance/check-in',
        data: {
          'image': imageBase64,
          'latitude': latitude,
          'longitude': longitude,
          if (notes != null) 'notes': notes,
        },
      );
      return AttendanceRecord.fromJson(response.data!);
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<AttendanceRecord> checkOut({
    required double latitude,
    required double longitude,
    String? notes,
  }) async {
    try {
      final response = await _dio.post<Map<String, dynamic>>(
        '/mobile/attendance/check-out',
        data: {
          'latitude': latitude,
          'longitude': longitude,
          if (notes != null) 'notes': notes,
        },
      );
      return AttendanceRecord.fromJson(response.data!);
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  String _dateOnly(DateTime d) =>
      '${d.year.toString().padLeft(4, '0')}-${d.month.toString().padLeft(2, '0')}-${d.day.toString().padLeft(2, '0')}';
}

final attendanceRepositoryProvider = Provider<AttendanceRepository>((ref) {
  return AttendanceRepository(ref.watch(dioProvider));
});
