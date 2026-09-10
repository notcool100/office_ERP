import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/network/api_client.dart';
import '../../../core/theme/app_colors.dart';
import '../../../core/utils/formatters.dart';
import '../../../core/widgets/async_view.dart';
import '../../../features/auth/application/session_controller.dart';
import '../../../models/messaging.dart';
import '../application/thread_controller.dart';
import '../data/messaging_repository.dart';
import 'authenticated_attachment_image.dart';

const _quickReactions = ['👍', '❤️', '😂', '🎉', '👀'];

class ChatThreadScreen extends ConsumerStatefulWidget {
  const ChatThreadScreen({super.key, required this.channelId, this.channel});

  final String channelId;
  final Channel? channel;

  @override
  ConsumerState<ChatThreadScreen> createState() => _ChatThreadScreenState();
}

class _ChatThreadScreenState extends ConsumerState<ChatThreadScreen> {
  final _inputController = TextEditingController();
  final _scrollController = ScrollController();
  final List<ComposerAttachment> _pendingFiles = [];
  bool _sending = false;

  @override
  void dispose() {
    _inputController.dispose();
    _scrollController.dispose();
    super.dispose();
  }

  void _scrollToBottom() {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (_scrollController.hasClients) {
        _scrollController.jumpTo(_scrollController.position.maxScrollExtent);
      }
    });
  }

  Future<void> _pickFiles() async {
    final result = await FilePicker.platform.pickFiles(allowMultiple: true, withData: true);
    if (result == null) return;
    setState(() {
      for (final f in result.files) {
        if (f.bytes != null) {
          _pendingFiles.add(ComposerAttachment(fileName: f.name, bytes: f.bytes!));
        }
      }
    });
  }

  Future<void> _send() async {
    final text = _inputController.text.trim();
    if (text.isEmpty && _pendingFiles.isEmpty) return;

    setState(() => _sending = true);
    final files = List<ComposerAttachment>.from(_pendingFiles);
    _inputController.clear();
    _pendingFiles.clear();

    try {
      await ref.read(threadControllerProvider(widget.channelId).notifier).send(
            content: text,
            files: files,
          );
      _scrollToBottom();
    } catch (e) {
      final message = e is ApiException ? e.message : e.toString();
      if (mounted) ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
    } finally {
      if (mounted) setState(() => _sending = false);
    }
  }

  Future<void> _showReactionPicker(String messageId) async {
    final emoji = await showModalBottomSheet<String>(
      context: context,
      builder: (context) => SafeArea(
        child: Padding(
          padding: const EdgeInsets.all(20),
          child: Wrap(
            spacing: 14,
            children: _quickReactions
                .map(
                  (e) => InkWell(
                    borderRadius: BorderRadius.circular(99),
                    onTap: () => Navigator.of(context).pop(e),
                    child: Padding(
                      padding: const EdgeInsets.all(8),
                      child: Text(e, style: const TextStyle(fontSize: 26)),
                    ),
                  ),
                )
                .toList(),
          ),
        ),
      ),
    );
    if (emoji != null) {
      try {
        await ref.read(threadControllerProvider(widget.channelId).notifier).toggleReaction(messageId, emoji);
      } catch (e) {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(e.toString())));
        }
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    final messagesAsync = ref.watch(threadControllerProvider(widget.channelId));
    final myUserId = ref.watch(sessionControllerProvider).value?.user.id;
    final title = widget.channel?.displayName() ?? 'Chat';

    ref.listen(threadControllerProvider(widget.channelId), (prev, next) {
      if (next.hasValue && (prev?.value?.length ?? 0) < (next.value?.length ?? 0)) {
        _scrollToBottom();
      }
    });

    return Scaffold(
      appBar: AppBar(title: Text(title)),
      body: Column(
        children: [
          Expanded(
            child: AsyncView(
              value: messagesAsync,
              onRetry: () => ref.read(threadControllerProvider(widget.channelId).notifier).refresh(),
              data: (messages) {
                if (messages.isEmpty) {
                  return const EmptyState(icon: Icons.chat_bubble_outline, title: 'No messages yet');
                }
                WidgetsBinding.instance.addPostFrameCallback((_) {
                  if (_scrollController.hasClients && _scrollController.offset == 0) {
                    _scrollController.jumpTo(_scrollController.position.maxScrollExtent);
                  }
                });
                return ListView.builder(
                  controller: _scrollController,
                  padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
                  itemCount: messages.length,
                  itemBuilder: (context, index) {
                    final message = messages[index];
                    final isMine = message.senderId == myUserId;
                    return _MessageBubble(
                      message: message,
                      isMine: isMine,
                      channelId: widget.channelId,
                      onLongPress: () => _showReactionPicker(message.id),
                      onToggleReaction: (emoji) => ref
                          .read(threadControllerProvider(widget.channelId).notifier)
                          .toggleReaction(message.id, emoji),
                    );
                  },
                );
              },
            ),
          ),
          if (_pendingFiles.isNotEmpty)
            SizedBox(
              height: 60,
              child: ListView.builder(
                scrollDirection: Axis.horizontal,
                padding: const EdgeInsets.symmetric(horizontal: 14),
                itemCount: _pendingFiles.length,
                itemBuilder: (context, index) {
                  final f = _pendingFiles[index];
                  return Container(
                    margin: const EdgeInsets.only(right: 8, top: 8, bottom: 4),
                    padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 8),
                    decoration: BoxDecoration(
                      color: colors.surfaceAlt,
                      border: Border.all(color: colors.borderSoft),
                      borderRadius: BorderRadius.circular(10),
                    ),
                    child: Row(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        Icon(Icons.insert_drive_file_outlined, size: 14, color: colors.textMuted),
                        const SizedBox(width: 6),
                        ConstrainedBox(
                          constraints: const BoxConstraints(maxWidth: 90),
                          child: Text(f.fileName, overflow: TextOverflow.ellipsis, maxLines: 1, style: const TextStyle(fontSize: 11.5)),
                        ),
                        const SizedBox(width: 4),
                        InkWell(
                          onTap: () => setState(() => _pendingFiles.removeAt(index)),
                          child: Icon(Icons.close_rounded, size: 14, color: colors.textFaint),
                        ),
                      ],
                    ),
                  );
                },
              ),
            ),
          SafeArea(
            top: false,
            child: Padding(
              padding: const EdgeInsets.fromLTRB(10, 8, 10, 10),
              child: Row(
                children: [
                  IconButton(
                    icon: Icon(Icons.attach_file_rounded, color: colors.textMuted, size: 20),
                    onPressed: _pickFiles,
                  ),
                  Expanded(
                    child: TextField(
                      controller: _inputController,
                      minLines: 1,
                      maxLines: 4,
                      textCapitalization: TextCapitalization.sentences,
                      decoration: InputDecoration(
                        hintText: 'Message...',
                        contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
                        border: OutlineInputBorder(borderRadius: BorderRadius.circular(24), borderSide: BorderSide.none),
                        filled: true,
                        fillColor: colors.surfaceAlt,
                      ),
                    ),
                  ),
                  const SizedBox(width: 6),
                  IconButton.filled(
                    onPressed: _sending ? null : _send,
                    icon: _sending
                        ? const SizedBox(width: 16, height: 16, child: CircularProgressIndicator(strokeWidth: 2, color: Colors.white))
                        : const Icon(Icons.arrow_upward_rounded, size: 18),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class _MessageBubble extends StatelessWidget {
  const _MessageBubble({
    required this.message,
    required this.isMine,
    required this.channelId,
    required this.onLongPress,
    required this.onToggleReaction,
  });

  final Message message;
  final bool isMine;
  final String channelId;
  final VoidCallback onLongPress;
  final void Function(String emoji) onToggleReaction;

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);

    return Align(
      alignment: isMine ? Alignment.centerRight : Alignment.centerLeft,
      child: ConstrainedBox(
        constraints: BoxConstraints(maxWidth: MediaQuery.of(context).size.width * 0.78),
        child: Column(
          crossAxisAlignment: isMine ? CrossAxisAlignment.end : CrossAxisAlignment.start,
          children: [
            if (!isMine && message.senderName != null)
              Padding(
                padding: const EdgeInsets.only(left: 4, bottom: 2),
                child: Text(message.senderName!, style: TextStyle(fontSize: 10.5, color: colors.textFaint, fontWeight: FontWeight.w600)),
              ),
            GestureDetector(
              onLongPress: onLongPress,
              child: Container(
                margin: const EdgeInsets.symmetric(vertical: 3),
                padding: const EdgeInsets.all(4),
                decoration: BoxDecoration(
                  color: isMine ? colors.accent : colors.surfaceAlt,
                  border: isMine ? null : Border.all(color: colors.borderSoft),
                  borderRadius: BorderRadius.only(
                    topLeft: const Radius.circular(16),
                    topRight: const Radius.circular(16),
                    bottomLeft: Radius.circular(isMine ? 16 : 4),
                    bottomRight: Radius.circular(isMine ? 4 : 16),
                  ),
                ),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    for (final attachment in message.attachments)
                      Padding(
                        padding: const EdgeInsets.all(4),
                        child: ClipRRect(
                          borderRadius: BorderRadius.circular(12),
                          child: attachment.isImage
                              ? AuthenticatedAttachmentImage(
                                  channelId: channelId,
                                  attachmentId: attachment.id,
                                  width: 200,
                                  height: 160,
                                )
                              : Container(
                                  width: 200,
                                  padding: const EdgeInsets.all(10),
                                  color: (isMine ? Colors.white : colors.surface).withValues(alpha: 0.5),
                                  child: Row(
                                    children: [
                                      Icon(Icons.insert_drive_file_rounded, size: 20, color: isMine ? Colors.white : colors.accent),
                                      const SizedBox(width: 8),
                                      Expanded(
                                        child: Column(
                                          crossAxisAlignment: CrossAxisAlignment.start,
                                          children: [
                                            Text(
                                              attachment.fileName,
                                              maxLines: 1,
                                              overflow: TextOverflow.ellipsis,
                                              style: TextStyle(fontSize: 12, fontWeight: FontWeight.w600, color: isMine ? Colors.white : colors.text),
                                            ),
                                            Text(
                                              Formatters.fileSize(attachment.fileSize),
                                              style: TextStyle(fontSize: 10.5, color: isMine ? Colors.white70 : colors.textMuted),
                                            ),
                                          ],
                                        ),
                                      ),
                                    ],
                                  ),
                                ),
                        ),
                      ),
                    if (message.content.isNotEmpty)
                      Padding(
                        padding: const EdgeInsets.symmetric(horizontal: 9, vertical: 6),
                        child: Text(
                          message.content,
                          style: TextStyle(fontSize: 13.5, height: 1.4, color: isMine ? Colors.white : colors.text),
                        ),
                      ),
                  ],
                ),
              ),
            ),
            if (message.reactions.isNotEmpty)
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 4),
                child: Wrap(
                  spacing: 5,
                  children: message.reactions
                      .map(
                        (r) => InkWell(
                          borderRadius: BorderRadius.circular(99),
                          onTap: () => onToggleReaction(r.emoji),
                          child: Container(
                            padding: const EdgeInsets.symmetric(horizontal: 7, vertical: 3),
                            decoration: BoxDecoration(
                              color: r.reactedByMe ? colors.accentSoft : colors.surfaceAlt,
                              border: Border.all(color: r.reactedByMe ? colors.accent : colors.borderSoft),
                              borderRadius: BorderRadius.circular(99),
                            ),
                            child: Text('${r.emoji} ${r.count}', style: const TextStyle(fontSize: 11)),
                          ),
                        ),
                      )
                      .toList(),
                ),
              ),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 4, vertical: 2),
              child: Text(Formatters.relativeTime(message.createdAt), style: TextStyle(fontSize: 9.5, color: colors.textFaint)),
            ),
          ],
        ),
      ),
    );
  }
}
