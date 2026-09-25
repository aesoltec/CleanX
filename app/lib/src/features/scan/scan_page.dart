/// Écran Scan : rapide/complet/personnalisé, progression temps réel,
/// pause/reprise/arrêt, liste des menaces détectées.
library;

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/engine/moteur.dart';
import '../../core/l10n/arb/app_localizations.dart';
import '../../core/providers/providers.dart';
import '../../core/theme/theme.dart';

/// Écran d'analyse.
class ScanPage extends ConsumerStatefulWidget {
  const ScanPage({super.key});

  @override
  ConsumerState<ScanPage> createState() => _ScanPageState();
}

class _ScanPageState extends ConsumerState<ScanPage> {
  final _racinesControleur = TextEditingController();

  @override
  void dispose() {
    _racinesControleur.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final loc = AppLocalizations.of(context);
    final scan = ref.watch(scanProvider);
    final notifier = ref.read(scanProvider.notifier);
    final moteur = ref.read(moteurProvider);
    return Scaffold(
      appBar: AppBar(title: Text(loc.scan)),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            if (!scan.enCours) ...[
              Row(
                children: [
                  Expanded(
                    child: ElevatedButton.icon(
                      onPressed: () => notifier.lancer(moteur.scanRapide),
                      icon: const Icon(Icons.flash_on),
                      label: Text(loc.quickScan),
                      // Couleurs AA via le thème (vertBouton 5,46).
                      style: ElevatedButton.styleFrom(
                        padding:
                            const EdgeInsets.symmetric(vertical: 16),
                      ),
                    ),
                  ),
                  const SizedBox(width: 12),
                  Expanded(
                    child: OutlinedButton.icon(
                      onPressed: () => notifier.lancer(moteur.scanComplet),
                      icon: const Icon(Icons.storage),
                      label: Text(loc.fullScan),
                      style: OutlinedButton.styleFrom(
                        padding:
                            const EdgeInsets.symmetric(vertical: 16),
                      ),
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 12),
              Row(
                children: [
                  Expanded(
                    child: TextField(
                      controller: _racinesControleur,
                      decoration: InputDecoration(
                        labelText: loc.rootsLabel,
                        border: const OutlineInputBorder(),
                      ),
                    ),
                  ),
                  const SizedBox(width: 12),
                  ElevatedButton(
                    onPressed: () {
                      final racines = _racinesControleur.text
                          .split(';')
                          .map((s) => s.trim())
                          .where((s) => s.isNotEmpty)
                          .toList();
                      if (racines.isNotEmpty) {
                        notifier.lancer(
                            () => moteur.scanPersonnalise(racines));
                      }
                    },
                    child: Text(loc.customScan),
                  ),
                ],
              ),
            ] else ...[
              // Contrôles pendant le scan.
              Row(
                children: [
                  Expanded(
                    // Pause en contour orange (texte blanc sur orange = 2,5
                    // insuffisant → bouton outlined, texte orange 6,65/7,57).
                    child: OutlinedButton.icon(
                      onPressed: notifier.basculerPause,
                      icon: Icon(scan.enPause
                          ? Icons.play_arrow
                          : Icons.pause),
                      label: Text(scan.enPause
                          ? loc.resumeScan
                          : loc.pauseScan),
                      style: OutlinedButton.styleFrom(
                        foregroundColor:
                            CleanXCouleurs.orangeTexte(context),
                        side: BorderSide(
                            color: CleanXCouleurs.orangeTexte(context)),
                        padding:
                            const EdgeInsets.symmetric(vertical: 16),
                      ),
                    ),
                  ),
                  const SizedBox(width: 12),
                  Expanded(
                    child: ElevatedButton.icon(
                      onPressed: notifier.arreter,
                      icon: const Icon(Icons.stop),
                      label: Text(loc.stopScan),
                      style: ElevatedButton.styleFrom(
                        // Blanc sur rougeBouton = 5,62 (AA).
                        backgroundColor: CleanXCouleurs.rougeBouton,
                        foregroundColor: Colors.white,
                        padding:
                            const EdgeInsets.symmetric(vertical: 16),
                      ),
                    ),
                  ),
                ],
              ),
            ],
            const SizedBox(height: 16),
            Card(
              child: Padding(
                padding: const EdgeInsets.all(20),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    ClipRRect(
                      borderRadius: BorderRadius.circular(8),
                      child: LinearProgressIndicator(
                        value: scan.enCours || scan.progression > 0
                            ? scan.progression
                            : 0,
                        minHeight: 10,
                      ),
                    ),
                    const SizedBox(height: 8),
                    Text(
                      scan.enCours
                          ? '${(scan.progression * 100).toStringAsFixed(1)} % — ${scan.traites}/${scan.total}\n${loc.fileInProgress} : ${scan.fichier}'
                          : scan.erreur ?? loc.waitingScan,
                      style: TextStyle(
                          color: CleanXCouleurs.texteSecondaire(context),
                          fontSize: 12),
                    ),
                    if (scan.annule == true)
                      Padding(
                        padding: const EdgeInsets.only(top: 8),
                        child: Text(loc.scanStopped,
                            style: TextStyle(
                                color: CleanXCouleurs.orangeTexte(context))),
                      ),
                  ],
                ),
              ),
            ),
            const SizedBox(height: 16),
            if (scan.menaces.isNotEmpty) ...[
              Text(loc.threats,
                  style: Theme.of(context).textTheme.titleMedium),
              const SizedBox(height: 8),
              for (final m in scan.menaces) _CarteMenace(menace: m),
            ],
          ],
        ),
      ),
    );
  }
}

/// Carte d'une menace détectée pendant le scan.
class _CarteMenace extends StatelessWidget {
  final MenaceMoteur menace;
  const _CarteMenace({required this.menace});

  @override
  Widget build(BuildContext context) {
    return Card(
      color: CleanXCouleurs.rougeAlerte.withValues(alpha: 0.08),
      child: ListTile(
        leading: const Icon(Icons.warning,
            color: CleanXCouleurs.rougeAlerte),
        title: Text(menace.fichier,
            style: const TextStyle(fontFamily: 'monospace', fontSize: 12)),
        subtitle: Text('${menace.menace}\n${menace.action}'),
        isThreeLine: true,
      ),
    );
  }
}
