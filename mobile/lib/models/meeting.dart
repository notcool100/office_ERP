class Meeting {
  Meeting({
    required this.id,
    required this.title,
    this.channelId,
    this.hostId,
    this.hostName,
    required this.status,
    required this.createdAt,
    this.endedAt,
  });

  final String id;
  final String title;
  final String? channelId;
  final String? hostId;
  final String? hostName;
  final String status;
  final DateTime createdAt;
  final DateTime? endedAt;

  bool get isActive => status == 'active';

  factory Meeting.fromJson(Map<String, dynamic> json) => Meeting(
        id: json['id'] as String,
        title: json['title'] as String,
        channelId: json['channel_id'] as String?,
        hostId: json['host_id'] as String?,
        hostName: json['host_name'] as String?,
        status: json['status'] as String,
        createdAt: DateTime.parse(json['created_at'] as String).toLocal(),
        endedAt:
            json['ended_at'] != null ? DateTime.parse(json['ended_at'] as String).toLocal() : null,
      );
}

class MeetingParticipant {
  MeetingParticipant({
    required this.userId,
    required this.displayName,
    required this.email,
    required this.role,
    required this.joinedAt,
  });

  final String userId;
  final String displayName;
  final String email;
  final String role;
  final DateTime joinedAt;

  factory MeetingParticipant.fromJson(Map<String, dynamic> json) => MeetingParticipant(
        userId: json['user_id'] as String,
        displayName: json['display_name'] as String,
        email: json['email'] as String,
        role: json['role'] as String,
        joinedAt: DateTime.parse(json['joined_at'] as String).toLocal(),
      );
}

class MeetingAttachment {
  MeetingAttachment({
    required this.id,
    required this.meetingId,
    this.uploadedById,
    this.uploadedByName,
    required this.category,
    required this.fileName,
    required this.contentType,
    required this.fileSize,
    required this.createdAt,
  });

  final String id;
  final String meetingId;
  final String? uploadedById;
  final String? uploadedByName;
  final String category;
  final String fileName;
  final String contentType;
  final int fileSize;
  final DateTime createdAt;

  factory MeetingAttachment.fromJson(Map<String, dynamic> json) => MeetingAttachment(
        id: json['id'] as String,
        meetingId: json['meeting_id'] as String,
        uploadedById: json['uploaded_by'] as String?,
        uploadedByName: json['uploaded_by_name'] as String?,
        category: json['category'] as String,
        fileName: json['file_name'] as String,
        contentType: json['content_type'] as String,
        fileSize: json['file_size'] as int,
        createdAt: DateTime.parse(json['created_at'] as String).toLocal(),
      );
}

class MeetingMinutes {
  MeetingMinutes({
    required this.meetingId,
    required this.content,
    this.updatedById,
    this.updatedByName,
    required this.updatedAt,
  });

  final String meetingId;
  final String content;
  final String? updatedById;
  final String? updatedByName;
  final DateTime updatedAt;

  factory MeetingMinutes.fromJson(Map<String, dynamic> json) => MeetingMinutes(
        meetingId: json['meeting_id'] as String,
        content: json['content'] as String,
        updatedById: json['updated_by'] as String?,
        updatedByName: json['updated_by_name'] as String?,
        updatedAt: DateTime.parse(json['updated_at'] as String).toLocal(),
      );
}
