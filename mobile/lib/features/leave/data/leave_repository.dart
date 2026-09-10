import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/network/api_client.dart';
import '../../../core/network/providers.dart';
import '../../../models/leave.dart';

class LeaveRepository {
  LeaveRepository(this._dio);
  final Dio _dio;

  Future<List<LeaveType>> leaveTypes() async {
    try {
      final response = await _dio.get<List<dynamic>>('/mobile/leave/types');
      return response.data!.map((e) => LeaveType.fromJson(e as Map<String, dynamic>)).toList();
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<List<LeaveBalance>> balance() async {
    try {
      final response = await _dio.get<List<dynamic>>('/mobile/leave/balance');
      return response.data!.map((e) => LeaveBalance.fromJson(e as Map<String, dynamic>)).toList();
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<List<LeaveRequest>> myRequests() async {
    try {
      final response = await _dio.get<List<dynamic>>('/mobile/leave/requests');
      return response.data!.map((e) => LeaveRequest.fromJson(e as Map<String, dynamic>)).toList();
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<List<LeaveRequest>> approvals() async {
    try {
      final response = await _dio.get<List<dynamic>>('/mobile/leave/approvals');
      return response.data!.map((e) => LeaveRequest.fromJson(e as Map<String, dynamic>)).toList();
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<LeaveRequest> createRequest({
    required String leaveTypeId,
    required DateTime startDate,
    required DateTime endDate,
    String? reason,
  }) async {
    try {
      final response = await _dio.post<Map<String, dynamic>>(
        '/mobile/leave/requests',
        data: {
          'leaveTypeId': leaveTypeId,
          'startDate': _dateOnly(startDate),
          'endDate': _dateOnly(endDate),
          if (reason != null && reason.isNotEmpty) 'reason': reason,
        },
      );
      return LeaveRequest.fromJson(response.data!);
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<LeaveRequest> approve(String id, {String? notes}) async {
    try {
      final response = await _dio.post<Map<String, dynamic>>(
        '/mobile/leave/requests/$id/approve',
        data: {'notes': ?notes},
      );
      return LeaveRequest.fromJson(response.data!);
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<LeaveRequest> reject(String id, {String? notes}) async {
    try {
      final response = await _dio.post<Map<String, dynamic>>(
        '/mobile/leave/requests/$id/reject',
        data: {'notes': ?notes},
      );
      return LeaveRequest.fromJson(response.data!);
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  String _dateOnly(DateTime d) =>
      '${d.year.toString().padLeft(4, '0')}-${d.month.toString().padLeft(2, '0')}-${d.day.toString().padLeft(2, '0')}';
}

final leaveRepositoryProvider = Provider<LeaveRepository>((ref) {
  return LeaveRepository(ref.watch(dioProvider));
});
