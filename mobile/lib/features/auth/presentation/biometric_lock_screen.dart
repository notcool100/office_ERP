import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/theme/app_colors.dart';
import '../application/app_lock_controller.dart';
import '../application/session_controller.dart';

class BiometricLockScreen extends ConsumerStatefulWidget {
  const BiometricLockScreen({super.key});

  @override
  ConsumerState<BiometricLockScreen> createState() => _BiometricLockScreenState();
}

class _BiometricLockScreenState extends ConsumerState<BiometricLockScreen> {
  bool _attempting = false;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) => _attemptUnlock());
  }

  Future<void> _attemptUnlock() async {
    if (_attempting) return;
    setState(() => _attempting = true);
    await ref.read(appLockControllerProvider.notifier).tryUnlock();
    if (mounted) setState(() => _attempting = false);
  }

  @override
  Widget build(BuildContext context) {
    final colors = AppColors.of(context);
    final name = ref.watch(sessionControllerProvider).value?.person.firstName;

    return Scaffold(
      body: SafeArea(
        child: Center(
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 32),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                Container(
                  width: 76,
                  height: 76,
                  decoration: BoxDecoration(color: colors.accentSoft, shape: BoxShape.circle),
                  alignment: Alignment.center,
                  child: Icon(Icons.fingerprint_rounded, size: 40, color: colors.accent),
                ),
                const SizedBox(height: 20),
                Text(
                  name != null ? 'Welcome back, $name' : 'Adya is locked',
                  style: Theme.of(context).textTheme.headlineSmall,
                  textAlign: TextAlign.center,
                ),
                const SizedBox(height: 6),
                Text(
                  'Unlock with Face ID or fingerprint to continue',
                  textAlign: TextAlign.center,
                  style: TextStyle(color: colors.textMuted, fontSize: 13),
                ),
                const SizedBox(height: 28),
                SizedBox(
                  width: double.infinity,
                  child: ElevatedButton.icon(
                    onPressed: _attempting ? null : _attemptUnlock,
                    icon: const Icon(Icons.lock_open_rounded, size: 18),
                    label: Text(_attempting ? 'Checking…' : 'Unlock'),
                  ),
                ),
                const SizedBox(height: 10),
                TextButton(
                  onPressed: () => ref.read(sessionControllerProvider.notifier).logout(),
                  child: const Text('Log out instead'),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
