class Channel {
  Channel({
    required this.id,
    required this.name,
    this.description,
    required this.isPrivate,
    required this.createdAt,
    this.createdBy,
    this.dmOtherUserName,
  });

  final String id;
  final String name;
  final String? description;
  final bool isPrivate;
  final DateTime createdAt;
  final String? createdBy;
  final String? dmOtherUserName;

  /// A 1:1 DM channel is created private with exactly the two members; the
  /// backend resolves the other participant's name into `dm_other_user_name`
  /// per-viewer. Group channels and public channels never have it set.
  bool get isDirectMessage => isPrivate && dmOtherUserName != null;

  String displayName() => isDirectMessage ? dmOtherUserName! : name;

  factory Channel.fromJson(Map<String, dynamic> json) => Channel(
        id: json['id'] as String,
        name: json['name'] as String,
        description: json['description'] as String?,
        isPrivate: json['is_private'] as bool? ?? false,
        createdAt: DateTime.parse(json['created_at'] as String).toLocal(),
        createdBy: json['created_by'] as String?,
        dmOtherUserName: json['dm_other_user_name'] as String?,
      );
}

class Attachment {
  Attachment({
    required this.id,
    required this.messageId,
    required this.fileName,
    required this.contentType,
    required this.fileSize,
    required this.isImage,
    required this.createdAt,
  });

  final String id;
  final String messageId;
  final String fileName;
  final String contentType;
  final int fileSize;
  final bool isImage;
  final DateTime createdAt;

  factory Attachment.fromJson(Map<String, dynamic> json) => Attachment(
        id: json['id'] as String,
        messageId: json['message_id'] as String,
        fileName: json['file_name'] as String,
        contentType: json['content_type'] as String,
        fileSize: json['file_size'] as int,
        isImage: json['is_image'] as bool? ?? false,
        createdAt: DateTime.parse(json['created_at'] as String).toLocal(),
      );
}

class ReactionSummary {
  ReactionSummary({required this.emoji, required this.count, required this.reactedByMe});

  final String emoji;
  final int count;
  final bool reactedByMe;

  factory ReactionSummary.fromJson(Map<String, dynamic> json) => ReactionSummary(
        emoji: json['emoji'] as String,
        count: json['count'] as int,
        reactedByMe: json['reacted_by_me'] as bool? ?? false,
      );
}

class Message {
  Message({
    required this.id,
    required this.channelId,
    this.senderId,
    this.senderName,
    required this.content,
    required this.createdAt,
    this.attachments = const [],
    this.reactions = const [],
  });

  final String id;
  final String channelId;
  final String? senderId;
  final String? senderName;
  final String content;
  final DateTime createdAt;
  final List<Attachment> attachments;
  final List<ReactionSummary> reactions;

  Message copyWithReactions(List<ReactionSummary> newReactions) => Message(
        id: id,
        channelId: channelId,
        senderId: senderId,
        senderName: senderName,
        content: content,
        createdAt: createdAt,
        attachments: attachments,
        reactions: newReactions,
      );

  factory Message.fromJson(Map<String, dynamic> json) => Message(
        id: json['id'] as String,
        channelId: json['channel_id'] as String,
        senderId: json['sender_id'] as String?,
        senderName: json['sender_name'] as String?,
        content: json['content'] as String,
        createdAt: DateTime.parse(json['created_at'] as String).toLocal(),
        attachments: (json['attachments'] as List<dynamic>? ?? [])
            .map((e) => Attachment.fromJson(e as Map<String, dynamic>))
            .toList(),
        reactions: (json['reactions'] as List<dynamic>? ?? [])
            .map((e) => ReactionSummary.fromJson(e as Map<String, dynamic>))
            .toList(),
      );
}

class ChannelMember {
  ChannelMember({required this.id, required this.displayName, required this.email});

  final String id;
  final String displayName;
  final String email;

  factory ChannelMember.fromJson(Map<String, dynamic> json) => ChannelMember(
        id: json['id'] as String,
        displayName: json['display_name'] as String,
        email: json['email'] as String,
      );
}

class ChannelMediaItem {
  ChannelMediaItem({
    required this.id,
    required this.messageId,
    required this.fileName,
    required this.contentType,
    required this.fileSize,
    required this.isImage,
    required this.createdAt,
    this.senderId,
    this.senderName,
  });

  final String id;
  final String messageId;
  final String fileName;
  final String contentType;
  final int fileSize;
  final bool isImage;
  final DateTime createdAt;
  final String? senderId;
  final String? senderName;

  factory ChannelMediaItem.fromJson(Map<String, dynamic> json) => ChannelMediaItem(
        id: json['id'] as String,
        messageId: json['message_id'] as String,
        fileName: json['file_name'] as String,
        contentType: json['content_type'] as String,
        fileSize: json['file_size'] as int,
        isImage: json['is_image'] as bool? ?? false,
        createdAt: DateTime.parse(json['created_at'] as String).toLocal(),
        senderId: json['sender_id'] as String?,
        senderName: json['sender_name'] as String?,
      );
}
