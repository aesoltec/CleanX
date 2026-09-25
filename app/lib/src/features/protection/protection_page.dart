/// Écran Protection temps réel : interrupteur animé + dossiers surveillés.
library;

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/l10n/arb/app_localizations.dart';
import '../../core/providers/providers.dart';
import '../../core/theme/theme.dart';

/// Écran de protection temps réel.
class ProtectionPage extends ConsumerStatefulWidget {
  const ProtectionPage({super.key});

  @override
  ConsumerState<ProtectionPage> createState() => _ProtectionPageState();
}

class _ProtectionPageState extends ConsumerState<ProtectionPage> {
  final _dossierControleur = TextEditingController();

  @override
  void dispose() {
    _dossierControleur.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final loc = AppLocalizations.of(context);
    final active = ref.watch(protectionProvider);
    final dossiers = ref.watch(dossiersProvider);
    return Scaffold(
      appBar: AppBar(title: Text(loc.protection)),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Card(
              child: Padding(
                padding: const EdgeInsets.all(24),
                child: Row(
                  children: [
                    // Pulsation quand la protection est active.
                    _IndicateurProtection(active: active),
                    const SizedBox(width: 20),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(loc.realtimeProtection,
                              style: Theme.of(context)
                                  .textTheme
                                  .titleMedium
                                  ?.copyWith(
                                      fontWeight: FontWeight.bold)),
                          Text(
                            active ? loc.protected : loc.unprotected,
                            style: TextStyle(
                              color: active
                                  ? CleanXCouleurs.vertActif(context)
                                  : CleanXCouleurs.orangeTexte(context),
                            ),
                          ),
                        ],
                      ),
                    ),
                    Switch.adaptive(
                      value: active,
                      onChanged: (v) => ref
                          .read(protectionProvider.notifier)
                          .basculer(v),
                    ),
                  ],
                ),
              ),
            ),
            const SizedBox(height: 16),
            Text(loc.watchedFolders,
                style: Theme.of(context).textTheme.titleMedium),
            const SizedBox(height: 8),
            Row(
              children: [
                Expanded(
                  child: TextField(
                    controller: _dossierControleur,
                    decoration: InputDecoration(
                      labelText: loc.folderPath,
                      border: const OutlineInputBorder(),
                    ),
                  ),
                ),
                const SizedBox(width: 12),
                ElevatedButton(
                  onPressed: () {
                    final d = _dossierControleur.text.trim();
                    if (d.isNotEmpty) {
                      ref.read(dossiersProvider.notifier).ajouter(d);
                      _dossierControleur.clear();
                    }
                  },
                  child: Text(loc.add),
                ),
              ],
            ),
            const SizedBox(height: 8),
            dossiers.when(
              loading: () =>
                  const Center(child: CircularProgressIndicator()),
              error: (e, _) => Text('$e'),
              data: (liste) => Column(
                children: [
                  for (final d in liste)
                    Card(
                      child: ListTile(
                        leading: const Icon(Icons.folder),
                        title: Text(d,
                            style: const TextStyle(
                                fontFamily: 'monospace', fontSize: 12)),
                        trailing: IconButton(
                          tooltip: loc.remove,
                          icon: const Icon(Icons.delete_outline),
                          onPressed: () => ref
                              .read(dossiersProvider.notifier)
                              .retirer(d),
                        ),
                      ),
                    ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

/// Pastille pulsante (accessible : Semantics + animation désactivable par l'OS).
class _IndicateurProtection extends StatelessWidget {
  final bool active;
  const _IndicateurProtection({required this.active});

  @override
  Widget build(BuildContext context) {
    final couleur = active
        ? CleanXCouleurs.vertSecurite
        : CleanXCouleurs.grisTexte;
    return Semantics(
      label: active ? 'Protection active' : 'Protection inactive',
      child: Container(
        width: 56,
        height: 56,
        decoration: BoxDecoration(
          shape: BoxShape.circle,
          color: couleur.withValues(alpha: 0.15),
          border: Border.all(color: couleur, width: 3),
        ),
        child: Icon(
            active ? Icons.shield : Icons.shield_outlined,
            color: couleur),
      ),
    );
  }
}
