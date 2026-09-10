import 'package:flutter/material.dart';

import '../theme/app_colors.dart';

/// The recurring "icon + title/subtitle + trailing" row used everywhere in
/// the mockup (today's schedule, meetings list, documents, notifications).
/// A list of these inside [AppListCard] reproduces the bordered card with
/// dividers between rows from the web design.
class AppListTile extends StatelessWidget {
  const AppListTile({
    super.key,
    this.leading,
    required this.title,
    this.subtitle,
    this.trailing,
    this.onTap,
    this.dense = false,
  });

  final Widget? leading;
  final String title;
  final String? subtitle;
  final Widget? trailing;
  final VoidCallback? onTap;
  final bool dense;

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    final content = Padding(
      padding: EdgeInsets.symmetric(horizontal: 14, vertical: dense ? 10 : 12),
      child: Row(
        children: [
          if (leading != null) ...[leading!, const SizedBox(width: 11)],
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisSize: MainAxisSize.min,
              children: [
                Text(
                  title,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  style: TextStyle(fontSize: 13.5, fontWeight: FontWeight.w600, color: colors.text),
                ),
                if (subtitle != null) ...[
                  const SizedBox(height: 1),
                  Text(
                    subtitle!,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: TextStyle(fontSize: 12, color: colors.textMuted),
                  ),
                ],
              ],
            ),
          ),
          if (trailing != null) ...[const SizedBox(width: 8), trailing!],
        ],
      ),
    );

    if (onTap == null) return content;
    return InkWell(onTap: onTap, child: content);
  }
}

/// Bordered card container with dividers between [AppListTile] children —
/// the `.list` class from the web mockup.
class AppListCard extends StatelessWidget {
  const AppListCard({super.key, required this.children, this.emptyLabel});

  final List<Widget> children;
  final String? emptyLabel;

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);

    if (children.isEmpty) {
      return Container(
        decoration: BoxDecoration(
          color: colors.surfaceAlt,
          border: Border.all(color: colors.borderSoft),
          borderRadius: BorderRadius.circular(16),
        ),
        padding: const EdgeInsets.symmetric(vertical: 26, horizontal: 10),
        alignment: Alignment.center,
        child: Text(
          emptyLabel ?? 'Nothing to show here.',
          textAlign: TextAlign.center,
          style: TextStyle(fontSize: 12.5, color: colors.textFaint),
        ),
      );
    }

    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: Container(
        decoration: BoxDecoration(
          color: colors.surfaceAlt,
          border: Border.all(color: colors.borderSoft),
          borderRadius: BorderRadius.circular(16),
        ),
        child: Column(
          children: [
            for (int i = 0; i < children.length; i++) ...[
              children[i],
              if (i != children.length - 1) Divider(height: 1, color: colors.borderSoft),
            ],
          ],
        ),
      ),
    );
  }
}

/// A row icon badge — the small rounded-square icon used to the left of
/// most list tiles.
class RowIcon extends StatelessWidget {
  const RowIcon(this.icon, {super.key, this.color});

  final IconData icon;
  final Color? color;

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    return Container(
      width: 36,
      height: 36,
      decoration: BoxDecoration(
        color: colors.surface,
        borderRadius: BorderRadius.circular(11),
        border: Border.all(color: colors.borderSoft),
      ),
      alignment: Alignment.center,
      child: Icon(icon, size: 18, color: color ?? colors.accent),
    );
  }
}
