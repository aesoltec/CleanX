/// Coquille responsive : rail de navigation (desktop) / onglets (mobile).
///
/// Breakpoints : < 600 mobile, 600–1200 rail compact, > 1200 rail étendu.
library;

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/l10n/arb/app_localizations.dart';
import '../../core/providers/providers.dart';
import '../dashboard/dashboard_page.dart';
import '../logs/logs_page.dart';
import '../protection/protection_page.dart';
import '../quarantine/quarantine_page.dart';
import '../scan/scan_page.dart';
import '../settings/settings_page.dart';

/// Pages dans l'ordre des destinations.
const _pages = [
  DashboardPage(),
  ScanPage(),
  ProtectionPage(),
  QuarantinePage(),
  LogsPage(),
  SettingsPage(),
];

class Coquille extends ConsumerStatefulWidget {
  const Coquille({super.key});

  @override
  ConsumerState<Coquille> createState() => _CoquilleState();
}

class _CoquilleState extends ConsumerState<Coquille> {
  int _index = 0;
  bool _init = false;

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    if (!_init) {
      _init = true;
      // Initialisation moteur + synchronisation état initial.
      Future.microtask(() async {
        final moteur = ref.read(moteurProvider);
        try {
          await moteur.initialiser('');
        } catch (_) {
          // Mode dégradé : le simule ne lève jamais ; le FRB loggue via statut.
        }
        if (mounted) {
          await ref.read(statutProvider.notifier).charger();
          await ref.read(protectionProvider.notifier).synchroniser();
          await ref.read(modeJeuProvider.notifier).synchroniser();
          await ref.read(dossiersProvider.notifier).charger();
          await ref.read(quarantaineProvider.notifier).charger();
          await ref.read(logsProvider.notifier).charger(moteur);
        }
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final loc = AppLocalizations.of(context);
    final destinations = [
      NavigationDestination(icon: const Icon(Icons.dashboard), label: loc.dashboard),
      NavigationDestination(icon: const Icon(Icons.search), label: loc.scan),
      NavigationDestination(icon: const Icon(Icons.shield), label: loc.protection),
      NavigationDestination(
          icon: const Icon(Icons.folder_off), label: loc.quarantine),
      NavigationDestination(icon: const Icon(Icons.list_alt), label: loc.logs),
      NavigationDestination(icon: const Icon(Icons.settings), label: loc.settings),
    ];
    return LayoutBuilder(
      builder: (context, contraintes) {
        final largeur = contraintes.maxWidth;
        if (largeur < 600) {
          // Mobile : onglets bas (Material 3).
          return Scaffold(
            body: IndexedStack(index: _index, children: _pages),
            bottomNavigationBar: NavigationBar(
              selectedIndex: _index,
              destinations: destinations,
              onDestinationSelected: (i) => setState(() => _index = i),
            ),
          );
        }
        // Desktop : rail latéral (étendu au-delà de 1200).
        return Scaffold(
          body: Row(
            children: [
              NavigationRail(
                selectedIndex: _index,
                extended: largeur > 1200,
                onDestinationSelected: (i) => setState(() => _index = i),
                leading: const Padding(
                  padding: EdgeInsets.symmetric(vertical: 16),
                  child: Icon(Icons.shield, size: 32),
                ),
                destinations: [
                  for (final d in destinations)
                    NavigationRailDestination(
                        icon: d.icon, label: Text(d.label)),
                ],
              ),
              const VerticalDivider(width: 1),
              Expanded(
                child: IndexedStack(index: _index, children: _pages),
              ),
            ],
          ),
        );
      },
    );
  }
}
