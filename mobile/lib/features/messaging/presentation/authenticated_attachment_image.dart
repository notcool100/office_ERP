import 'dart:typed_data';

import 'package:dio/dio.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:shimmer/shimmer.dart';

import '../../../core/network/providers.dart';
import '../../../core/theme/app_colors.dart';

/// `/messaging/channels/{channelId}/attachments/{id}` requires a Bearer
/// token, so a plain `Image.network` (which sends no auth header) can't
/// load it — this fetches the bytes through the app's own Dio instance
/// (token refresh included) and renders them once they arrive.
class AuthenticatedAttachmentImage extends ConsumerWidget {
  const AuthenticatedAttachmentImage({
    super.key,
    required this.channelId,
    required this.attachmentId,
    this.width,
    this.height,
    this.fit = BoxFit.cover,
  });

  final String channelId;
  final String attachmentId;
  final double? width;
  final double? height;
  final BoxFit fit;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final colors = AppColors.of(context);
    final dio = ref.watch(dioProvider);

    return FutureBuilder(
      future: dio.get<List<int>>(
        '/messaging/channels/$channelId/attachments/$attachmentId',
        options: Options(responseType: ResponseType.bytes),
      ),
      builder: (context, snapshot) {
        if (snapshot.connectionState != ConnectionState.done) {
          return Shimmer.fromColors(
            baseColor: colors.surfaceAlt,
            highlightColor: colors.borderSoft,
            child: Container(width: width, height: height, color: colors.surfaceAlt),
          );
        }
        if (snapshot.hasError || snapshot.data?.data == null) {
          return Container(
            width: width,
            height: height,
            color: colors.surfaceAlt,
            alignment: Alignment.center,
            child: Icon(Icons.broken_image_outlined, color: colors.textFaint, size: 22),
          );
        }
        return Image.memory(
          Uint8List.fromList(snapshot.data!.data!),
          width: width,
          height: height,
          fit: fit,
        );
      },
    );
  }
}
