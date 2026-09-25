/// Écran Quarantaine : liste des isolés, restaurer / supprimer (avec confirmation).
library;

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/engine/moteur.dart';
import '../../core/l10n/arb/app_localizations.dart';
import '../../core/providers/providers.dart';
import '../../core/theme/theme.dart';

/// Écran de quarantaine.
class QuarantinePage extends ConsumerWidget {
  const QuarantinePage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final loc = AppLocalizations.of(context);
    final quarantaine = ref.watch(quarantaineProvider);
    return Scaffold(
      appBar: AppBar(title: Text(loc.quarantine)),
      body: Padding(
        padding: const EdgeInsets.all(20),
        child: quarantaine.when(
          loading: () => const Center(child: CircularProgressIndicator()),
          error: (e, _) => Center(child: Text('$e')),
          data: (fichiers) {
            if (fichiers.isEmpty) {
              return Center(child: Text(loc.quarantineEmpty));
            }
            return ListView.separated(
              itemCount: fichiers.length,
              separatorBuilder: (_, __) => const SizedBox(height: 8),
              itemBuilder: (context, i) =>
                  _CarteQuarantaine(fichier: fichiers[i]),
            );
          },
        ),
      ),
    );
  }
}

/// Carte d'un fichier isolé avec ses actions (confirmées par dialogue).
class _CarteQuarantaine extends ConsumerWidget {
  final FichierQuarantaineDto fichier;
  const _CarteQuarantaine({required this.fichier});

  /// Dialogue de confirmation générique.
  Future<bool> _confirmer(
      BuildContext context, String titre, String message) async {
    final loc = AppLocalizations.of(context);
    final reponse = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: Text(titre),
        content: Text(message),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context, false),
            child: Text(loc.cancel),
          ),
          FilledButton(
            onPressed: () => Navigator.pop(context, true),
            child: Text(loc.confirm),
          ),
        ],
      ),
    );
    return reponse ?? false;
  }

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final loc = AppLocalizations.of(context);
    final notifier = ref.read(quarantaineProvider.notifier);
    return Card(
      child: ListTile(
        leading: const Icon(Icons.folder_off,
            color: CleanXCouleurs.orangeAvertissement),
        title: Text(fichier.nom,
            style: const TextStyle(fontWeight: FontWeight.bold)),
        subtitle: Text(
          '${fichier.date}\n${fichier.raison} (score ${fichier.score})\n${fichier.origine}',
          style: const TextStyle(fontSize: 12),
        ),
        isThreeLine: true,
        trailing: PopupMenuButton<String>(
          onSelected: (action) async {
            if (action == 'restaurer') {
              final ok = await _confirmer(
                  context, loc.restore, loc.restoreConfirm);
              if (ok) {
                await notifier.restaurer(fichier.id, fichier.origine);
              }
            } else if (action == 'supprimer') {
              final ok = await _confirmer(
                  context, loc.delete, loc.deleteConfirm);
              if (ok) {
                await notifier.supprimer(fichier.id);
              }
            }
          },
          itemBuilder: (context) => [
            PopupMenuItem(value: 'restaurer', child: Text(loc.restore)),
            PopupMenuItem(value: 'supprimer', child: Text(loc.delete)),
          ],
        ),
      ),
    );
  }
}
