/// Dashboard : statut global animé, jauge de score, cartes d'état.
library;

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/l10n/arb/app_localizations.dart';
import '../../core/providers/providers.dart';
import '../../core/theme/theme.dart';
import '../../shared/widgets/jauge_securite.dart';

/// Tableau de bord principal.
class DashboardPage extends ConsumerWidget {
  const DashboardPage({super.key});

  /// Score 0–100 : 100 si protégé, −5/menace, −40 si non protégé.
  int _score(bool protection, int menaces) {
    var s = protection ? 100 : 60;
    s -= menaces * 5;
    return s.clamp(0, 100);
  }

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final loc = AppLocalizations.of(context);
    final statut = ref.watch(statutProvider);
    final protection = ref.watch(protectionProvider);
    return Scaffold(
      appBar: AppBar(title: Text(loc.dashboard)),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            statut.when(
              loading: () =>
                  const Center(child: CircularProgressIndicator()),
              error: (e, _) => Card(
                child: ListTile(
                  leading: const Icon(Icons.cloud_off,
                      color: CleanXCouleurs.rougeAlerte),
                  title: Text(loc.engineOffline),
                  subtitle: Text('$e'),
                  trailing: IconButton(
                    tooltip: loc.retry,
                    icon: const Icon(Icons.refresh),
                    onPressed: () =>
                        ref.read(statutProvider.notifier).charger(),
                  ),
                ),
              ),
              data: (s) {
                final protege = protection;
                final score = _score(protege, s.menaces);
                return Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    Card(
                      child: Padding(
                        padding: const EdgeInsets.all(24),
                        child: Row(
                          children: [
                            JaugeSecurite(score: score),
                            const SizedBox(width: 20),
                            Expanded(
                              child: Column(
                                crossAxisAlignment:
                                    CrossAxisAlignment.start,
                                children: [
                                  Text(
                                    protege
                                        ? loc.protected
                                        : loc.unprotected,
                                    style: Theme.of(context)
                                        .textTheme
                                        .titleLarge
                                        ?.copyWith(
                                            fontWeight: FontWeight.bold),
                                  ),
                                  const SizedBox(height: 4),
                                  Text(
                                    '${loc.securityScore} : $score/100',
                                    style: TextStyle(
                                        color: CleanXCouleurs
                                            .texteSecondaire(context)),
                                  ),
                                  const SizedBox(height: 12),
                                  Switch.adaptive(
                                    value: protege,
                                    onChanged: (v) => ref
                                        .read(protectionProvider.notifier)
                                        .basculer(v),
                                  ),
                                ],
                              ),
                            ),
                          ],
                        ),
                      ),
                    ),
                    const SizedBox(height: 16),
                    Wrap(
                      spacing: 12,
                      runSpacing: 12,
                      children: [
                        _CarteEtat(
                            titre: loc.signatures,
                            valeur: '${s.signatures}',
                            icone: Icons.fingerprint),
                        _CarteEtat(
                            titre: loc.threats,
                            valeur: '${s.menaces}',
                            icone: Icons.warning_amber_rounded),
                        _CarteEtat(
                            titre: loc.filesAnalyzed,
                            valeur: '${s.fichiersAnalyses}',
                            icone: Icons.description),
                        _CarteProcessus(),
                      ],
                    ),
                  ],
                );
              },
            ),
          ],
        ),
      ),
    );
  }
}

/// Carte "processus suspects" (base anti-rootkit, FutureProvider dédié).
class _CarteProcessus extends ConsumerWidget {
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final loc = AppLocalizations.of(context);
    final suspects = ref.watch(processusSuspectsProvider);
    return SizedBox(
      width: 180,
      child: Card(
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Icon(Icons.memory, color: CleanXCouleurs.vertSecurite),
              const SizedBox(height: 8),
              Text(
                suspects.when(
                  loading: () => '…',
                  error: (_, __) => '?',
                  data: (l) => '${l.length}',
                ),
                style: Theme.of(context)
                    .textTheme
                    .headlineSmall
                    ?.copyWith(fontWeight: FontWeight.bold),
              ),
              Text(loc.processusSuspects,
                  style: TextStyle(
                      color: CleanXCouleurs.texteSecondaire(context),
                      fontSize: 12)),
            ],
          ),
        ),
      ),
    );
  }
}

/// Petite carte métrique (responsive via Wrap parent).
class _CarteEtat extends StatelessWidget {
  final String titre;
  final String valeur;
  final IconData icone;
  const _CarteEtat(
      {required this.titre, required this.valeur, required this.icone});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 180,
      child: Card(
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Icon(icone, color: CleanXCouleurs.vertSecurite),
              const SizedBox(height: 8),
              Text(valeur,
                  style: Theme.of(context)
                      .textTheme
                      .headlineSmall
                      ?.copyWith(fontWeight: FontWeight.bold)),
              Text(titre,
                  style: TextStyle(
                      color:
                          CleanXCouleurs.texteSecondaire(context),
                      fontSize: 12)),
            ],
          ),
        ),
      ),
    );
  }
}
