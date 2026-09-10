import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/network/api_client.dart';
import '../../../core/network/providers.dart';
import '../../../models/calendar_event.dart';

class CalendarRepository {
  CalendarRepository(this._dio);
  final Dio _dio;

  Future<List<CalendarEvent>> listEvents({required DateTime start, required DateTime end}) async {
    try {
      final response = await _dio.get<List<dynamic>>(
        '/calendar/events',
        queryParameters: {
          'start_date': _dateOnly(start),
          'end_date': _dateOnly(end),
        },
      );
      return response.data!
          .map((e) => CalendarEvent.fromJson(e as Map<String, dynamic>))
          .toList();
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<CalendarEvent> createEvent(CalendarEvent draft) async {
    try {
      final response = await _dio.post<Map<String, dynamic>>(
        '/calendar/events',
        data: draft.toCreateJson(),
      );
      return CalendarEvent.fromJson(response.data!);
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  Future<void> deleteEvent(String id) async {
    try {
      await _dio.delete('/calendar/events/$id');
    } catch (e) {
      throw ApiClient.toApiException(e);
    }
  }

  String _dateOnly(DateTime d) =>
      '${d.year.toString().padLeft(4, '0')}-${d.month.toString().padLeft(2, '0')}-${d.day.toString().padLeft(2, '0')}';
}

final calendarRepositoryProvider = Provider<CalendarRepository>((ref) {
  return CalendarRepository(ref.watch(dioProvider));
});
