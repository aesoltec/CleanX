/// Écran Journal : filtres par niveau, recherche, export CSV.
library;

import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:path_provider/path_provider.dart';

import '../../core/engine/moteur.dart';
import '../../core/export/rapport_pdf.dart';
import '../../core/l10n/arb/app_localizations.dart';
import '../../core/providers/providers.dart';
import '../../core/theme/theme.dart';

/// Niveaux filtrables (doit rester synchronisé avec `logging::NIVEAUX_VALIDES`).
const _niveaux = ['tous', 'info', 'alerte', 'quarantaine'];

/// Écran du journal d'événements.
class LogsPage extends ConsumerWidget {
  const LogsPage({super.key});

  /// Couleur AA par niveau : erreur du thème actif (rouge adapté clair/sombre),
  /// orange quarantaine, gris info.
  Color _couleurNiveau(BuildContext context, String niveau) {
    switch (niveau) {
      case 'alerte':
        return Theme.of(context).colorScheme.error;
      case 'quarantaine':
        return CleanXCouleurs.orangeTexte(context);
      default:
        return CleanXCouleurs.texteSecondaire(context);
    }
  }

  /// Exporte le rapport PDF (statut + session + quarantaine + logs).
  Future<void> _exporterPdf(BuildContext context, WidgetRef ref) async {
    final loc = AppLocalizations.of(context);
    try {
      final moteur = ref.read(moteurProvider);
      final scan = ref.read(scanProvider);
      final donnees = DonneesRapport(
        statut: await moteur.statut(),
        menacesSession: scan.menaces,
        quarantaine: await moteur.listerQuarantaine(),
        logs: ref.read(logsProvider),
        date: DateTime.now(),
      );
      final octets = await construireRapportPdf(donnees);
      final dir = await getApplicationDocumentsDirectory();
      final cible =
          '${dir.path}${Platform.pathSeparator}cleanx-rapport-${DateTime.now().millisecondsSinceEpoch}.pdf';
      await File(cible).writeAsBytes(octets);
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('${loc.exportPdf} : $cible')),
        );
      }
    } catch (e) {
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('$e')),
        );
      }
    }
  }

  /// Exporte le CSV dans le dossier Documents et affiche le chemin.
  Future<void> _exporter(BuildContext context, WidgetRef ref) async {
    final loc = AppLocalizations.of(context);
    try {
      final csv = await ref.read(moteurProvider).exporterLogsCsv();
      final dir = await getApplicationDocumentsDirectory();
      final cible =
          '${dir.path}${Platform.pathSeparator}cleanx-logs-${DateTime.now().millisecondsSinceEpoch}.csv';
      await File(cible).writeAsString(csv);
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('${loc.exportCsv} : $cible')),
        );
      }
    } catch (e) {
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('$e')),
        );
      }
    }
  }

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final loc = AppLocalizations.of(context);
    final logs = ref.watch(logsProvider);
    final filtre = ref.watch(filtreLogsProvider);
    final recherche = ref.watch(rechercheLogsProvider).toLowerCase();
    final visibles = logs.where((l) {
      final okFiltre = filtre == 'tous' || l.niveau == filtre;
      final okRecherche = recherche.isEmpty ||
          l.message.toLowerCase().contains(recherche) ||
          l.date.contains(recherche);
      return okFiltre && okRecherche;
    }).toList();
    return Scaffold(
      appBar: AppBar(
        title: Text(loc.logs),
        actions: [
          PopupMenuButton<String>(
            tooltip: loc.export,
            icon: const Icon(Icons.download),
            onSelected: (action) {
              if (action == 'csv') {
                _exporter(context, ref);
              } else if (action == 'pdf') {
                _exporterPdf(context, ref);
              }
            },
            itemBuilder: (context) => [
              PopupMenuItem(value: 'csv', child: Text(loc.exportCsv)),
              PopupMenuItem(value: 'pdf', child: Text(loc.exportPdf)),
            ],
          ),
        ],
      ),
      body: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          children: [
            TextField(
              decoration: InputDecoration(
                labelText: loc.search,
                prefixIcon: const Icon(Icons.search),
                border: const OutlineInputBorder(),
              ),
              onChanged: (v) =>
                  ref.read(rechercheLogsProvider.notifier).state = v,
            ),
            const SizedBox(height: 8),
            SingleChildScrollView(
              scrollDirection: Axis.horizontal,
              child: Row(
                children: [
                  for (final n in _niveaux)
                    Padding(
                      padding: const EdgeInsets.only(right: 8),
                      child: ChoiceChip(
                        label: Text(n == 'tous' ? loc.filterAll : n),
                        selected: filtre == n,
                        onSelected: (_) => ref
                            .read(filtreLogsProvider.notifier)
                            .state = n,
                      ),
                    ),
                ],
              ),
            ),
            const SizedBox(height: 8),
            Expanded(
              child: visibles.isEmpty
                  ? Center(child: Text(loc.noItems))
                  : ListView.separated(
                      itemCount: visibles.length,
                      separatorBuilder: (_, __) => const Divider(height: 1),
                      itemBuilder: (context, i) {
                        final l = visibles[i];
                        return _LigneLog(
                            log: l, couleur: _couleurNiveau(context, l.niveau));
                      },
                    ),
            ),
          ],
        ),
      ),
    );
  }
}

/// Ligne de journal (police monospace, couleur selon niveau).
class _LigneLog extends StatelessWidget {
  final EntreeLogDto log;
  final Color couleur;
  const _LigneLog({required this.log, required this.couleur});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 6),
      child: Text(
        '[${log.date}] [${log.niveau}] ${log.message}',
        style: TextStyle(
            fontSize: 12, fontFamily: 'monospace', color: couleur),
      ),
    );
  }
}
