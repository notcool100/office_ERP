import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/network/api_client.dart';
import '../../../core/theme/app_colors.dart';
import '../../../core/widgets/async_view.dart';
import '../../../core/widgets/initials_avatar.dart';
import '../../../core/widgets/section_label.dart';
import '../../../features/auth/application/session_controller.dart';
import '../../../features/auth/data/auth_repository.dart';
import '../../../features/auth/data/biometric_prefs.dart';
import '../../../models/user.dart';
import '../data/profile_repository.dart';

class ProfileScreen extends ConsumerStatefulWidget {
  const ProfileScreen({super.key});

  @override
  ConsumerState<ProfileScreen> createState() => _ProfileScreenState();
}

class _ProfileScreenState extends ConsumerState<ProfileScreen> {
  int _tab = 0;

  Future<void> _editContact() async {
    final bootstrap = ref.read(sessionControllerProvider).value;
    if (bootstrap == null) return;

    final emailController = TextEditingController(text: bootstrap.person.email ?? '');
    final phoneController = TextEditingController(text: bootstrap.person.phone ?? '');

    final saved = await showModalBottomSheet<bool>(
      context: context,
      isScrollControlled: true,
      builder: (sheetContext) => Padding(
        padding: EdgeInsets.only(left: 20, right: 20, top: 20, bottom: 20 + MediaQuery.of(sheetContext).viewInsets.bottom),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('Edit contact info', style: Theme.of(sheetContext).textTheme.titleLarge),
            const SizedBox(height: 16),
            TextField(controller: emailController, decoration: const InputDecoration(labelText: 'Email')),
            const SizedBox(height: 12),
            TextField(controller: phoneController, decoration: const InputDecoration(labelText: 'Phone')),
            const SizedBox(height: 16),
            SizedBox(
              width: double.infinity,
              child: ElevatedButton(
                onPressed: () => Navigator.of(sheetContext).pop(true),
                child: const Text('Save'),
              ),
            ),
          ],
        ),
      ),
    );

    if (saved == true) {
      try {
        await ref.read(profileRepositoryProvider).updateProfile(
              email: emailController.text.trim(),
              phone: phoneController.text.trim(),
            );
        await ref.read(sessionControllerProvider.notifier).refresh();
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Profile updated')));
        }
      } catch (e) {
        final message = e is ApiException ? e.message : e.toString();
        if (mounted) ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
      }
    }
  }

  Future<void> _changePassword() async {
    final currentController = TextEditingController();
    final newController = TextEditingController();

    final submit = await showModalBottomSheet<bool>(
      context: context,
      isScrollControlled: true,
      builder: (sheetContext) => Padding(
        padding: EdgeInsets.only(left: 20, right: 20, top: 20, bottom: 20 + MediaQuery.of(sheetContext).viewInsets.bottom),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('Change password', style: Theme.of(sheetContext).textTheme.titleLarge),
            const SizedBox(height: 16),
            TextField(controller: currentController, obscureText: true, decoration: const InputDecoration(labelText: 'Current password')),
            const SizedBox(height: 12),
            TextField(controller: newController, obscureText: true, decoration: const InputDecoration(labelText: 'New password')),
            const SizedBox(height: 16),
            SizedBox(
              width: double.infinity,
              child: ElevatedButton(onPressed: () => Navigator.of(sheetContext).pop(true), child: const Text('Update password')),
            ),
          ],
        ),
      ),
    );

    if (submit == true) {
      try {
        await ref.read(authRepositoryProvider).changePassword(
              currentPassword: currentController.text,
              newPassword: newController.text,
            );
        if (mounted) ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Password changed')));
      } catch (e) {
        final message = e is ApiException ? e.message : e.toString();
        if (mounted) ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    final session = ref.watch(sessionControllerProvider);
    final biometricEnabled = ref.watch(biometricLockEnabledProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Profile'),
        actions: [
          IconButton(icon: const Icon(Icons.edit_rounded, size: 19), onPressed: _editContact),
        ],
      ),
      body: AsyncView(
        value: session,
        data: (bootstrapOrNull) {
          if (bootstrapOrNull == null) return const SizedBox.shrink();
          final bootstrap = bootstrapOrNull;
          return ListView(
            padding: const EdgeInsets.symmetric(horizontal: 18),
            children: [
              const SizedBox(height: 10),
              Center(
                child: Column(
                  children: [
                    InitialsAvatar(bootstrap.person.fullName, size: 76, seed: bootstrap.user.id),
                    const SizedBox(height: 10),
                    Text(bootstrap.person.fullName, style: Theme.of(context).textTheme.headlineSmall),
                    const SizedBox(height: 2),
                    Text(
                      [bootstrap.employee?.position, bootstrap.employee?.department].where((s) => s != null).join(' · '),
                      style: TextStyle(fontSize: 12.5, color: colors.textMuted),
                    ),
                    if (bootstrap.employee != null) ...[
                      const SizedBox(height: 8),
                      Container(
                        padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
                        decoration: BoxDecoration(
                          color: colors.surface,
                          border: Border.all(color: colors.borderSoft),
                          borderRadius: BorderRadius.circular(99),
                        ),
                        child: Text(
                          bootstrap.employee!.employeeCode,
                          style: TextStyle(fontFamily: 'monospace', fontSize: 11, fontWeight: FontWeight.w700, color: colors.textMuted),
                        ),
                      ),
                    ],
                  ],
                ),
              ),
              const SizedBox(height: 16),
              _TabSwitcher(value: _tab, onChanged: (v) => setState(() => _tab = v)),
              const SizedBox(height: 8),
              if (_tab == 0) _InfoTab(bootstrap: bootstrap) else const _ScheduleTab(),
              const SectionLabel('Security'),
              biometricEnabled.when(
                data: (enabled) => Container(
                  decoration: BoxDecoration(color: colors.surfaceAlt, border: Border.all(color: colors.borderSoft), borderRadius: BorderRadius.circular(16)),
                  child: SwitchListTile(
                    value: enabled,
                    onChanged: (v) async {
                      await ref.read(biometricPrefsProvider).setEnabled(v);
                      ref.invalidate(biometricLockEnabledProvider);
                    },
                    title: const Text('Biometric app lock', style: TextStyle(fontSize: 13.5, fontWeight: FontWeight.w600)),
                    subtitle: const Text('Require Face ID / fingerprint to reopen the app', style: TextStyle(fontSize: 11.5)),
                  ),
                ),
                loading: () => const SizedBox.shrink(),
                error: (_, _) => const SizedBox.shrink(),
              ),
              const SizedBox(height: 10),
              OutlinedButton(onPressed: _changePassword, child: const Text('Change password')),
              const SizedBox(height: 20),
              OutlinedButton(
                style: OutlinedButton.styleFrom(backgroundColor: colors.dangerSoft, foregroundColor: colors.danger),
                onPressed: () => ref.read(sessionControllerProvider.notifier).logout(),
                child: const Row(mainAxisAlignment: MainAxisAlignment.center, children: [
                  Icon(Icons.logout_rounded, size: 16),
                  SizedBox(width: 8),
                  Text('Log Out'),
                ]),
              ),
              const SizedBox(height: 24),
            ],
          );
        },
      ),
    );
  }
}

class _TabSwitcher extends StatelessWidget {
  const _TabSwitcher({required this.value, required this.onChanged});
  final int value;
  final ValueChanged<int> onChanged;

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    Widget seg(int index, String label) {
      final selected = value == index;
      return Expanded(
        child: InkWell(
          onTap: () => onChanged(index),
          borderRadius: BorderRadius.circular(9),
          child: Container(
            padding: const EdgeInsets.symmetric(vertical: 8),
            decoration: BoxDecoration(color: selected ? colors.surface : null, borderRadius: BorderRadius.circular(9)),
            alignment: Alignment.center,
            child: Text(label, style: TextStyle(fontSize: 12.5, fontWeight: FontWeight.w700, color: selected ? colors.accent : colors.textMuted)),
          ),
        ),
      );
    }

    return Container(
      padding: const EdgeInsets.all(3),
      decoration: BoxDecoration(color: colors.surfaceAlt, border: Border.all(color: colors.borderSoft), borderRadius: BorderRadius.circular(12)),
      child: Row(children: [seg(0, 'Info'), seg(1, 'Schedule')]),
    );
  }
}

class _InfoTab extends StatelessWidget {
  const _InfoTab({required this.bootstrap});
  final BootstrapResponse bootstrap;

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    final rows = <(IconData, String, String)>[
      (Icons.mail_outline_rounded, 'Email', bootstrap.person.email ?? bootstrap.user.email),
      (Icons.phone_outlined, 'Phone', bootstrap.person.phone ?? bootstrap.user.phone),
      if (bootstrap.employee != null)
        (Icons.event_available_outlined, 'Hire date', '${bootstrap.employee!.hireDate.year}-${bootstrap.employee!.hireDate.month.toString().padLeft(2, '0')}-${bootstrap.employee!.hireDate.day.toString().padLeft(2, '0')}'),
      if (bootstrap.employee?.managerName != null) (Icons.supervisor_account_outlined, 'Manager', bootstrap.employee!.managerName!),
    ];

    return Container(
      decoration: BoxDecoration(color: colors.surfaceAlt, border: Border.all(color: colors.borderSoft), borderRadius: BorderRadius.circular(16)),
      padding: const EdgeInsets.symmetric(horizontal: 14),
      child: Column(
        children: [
          for (int i = 0; i < rows.length; i++) ...[
            Padding(
              padding: const EdgeInsets.symmetric(vertical: 11),
              child: Row(
                children: [
                  Icon(rows[i].$1, size: 15, color: colors.textMuted),
                  const SizedBox(width: 9),
                  Text(rows[i].$2, style: TextStyle(fontSize: 12, color: colors.textMuted)),
                  const Spacer(),
                  Flexible(
                    child: Text(rows[i].$3, textAlign: TextAlign.right, style: TextStyle(fontSize: 13, fontWeight: FontWeight.w600, color: colors.text)),
                  ),
                ],
              ),
            ),
            if (i != rows.length - 1) Divider(height: 1, color: colors.borderSoft),
          ],
        ],
      ),
    );
  }
}

class _ScheduleTab extends ConsumerWidget {
  const _ScheduleTab();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final colors = AppColors.of(context);
    final asyncSchedule = ref.watch(scheduleProvider);

    return AsyncView(
      value: asyncSchedule,
      onRetry: () => ref.invalidate(scheduleProvider),
      data: (days) => Container(
        decoration: BoxDecoration(color: colors.surfaceAlt, border: Border.all(color: colors.borderSoft), borderRadius: BorderRadius.circular(16)),
        padding: const EdgeInsets.symmetric(horizontal: 14),
        child: Column(
          children: [
            for (int i = 0; i < days.length; i++) ...[
              Padding(
                padding: const EdgeInsets.symmetric(vertical: 11),
                child: Row(
                  children: [
                    Text(days[i].dayName, style: TextStyle(fontSize: 12.5, fontWeight: FontWeight.w600, color: colors.textMuted)),
                    const Spacer(),
                    Text(
                      days[i].isWorking && days[i].startTime != null
                          ? '${days[i].startTime} – ${days[i].endTime}'
                          : 'Off',
                      style: TextStyle(fontSize: 12.5, fontFamily: 'monospace', color: colors.text),
                    ),
                  ],
                ),
              ),
              if (i != days.length - 1) Divider(height: 1, color: colors.borderSoft),
            ],
          ],
        ),
      ),
    );
  }
}
