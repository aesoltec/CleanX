/// Écran Paramètres : thème, langue, planification, mises à jour, à propos.
library;

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/engine/moteur.dart';
import '../../core/l10n/arb/app_localizations.dart';
import '../../core/providers/providers.dart';

/// Écran des paramètres.
class SettingsPage extends ConsumerWidget {
  const SettingsPage({super.key});

  /// Dialogue d'ajout de planification (nom, racines, intervalle).
  Future<void> _ajouterPlanification(
      BuildContext context, WidgetRef ref) async {
    final loc = AppLocalizations.of(context);
    final nom = TextEditingController();
    final racines = TextEditingController();
    final intervalle = TextEditingController(text: '168');
    final ok = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: Text(loc.scheduling),
        content: SingleChildScrollView(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              TextField(
                  controller: nom,
                  decoration: InputDecoration(labelText: loc.planName)),
              TextField(
                  controller: racines,
                  decoration:
                      InputDecoration(labelText: loc.rootsLabel)),
              TextField(
                controller: intervalle,
                decoration:
                    InputDecoration(labelText: loc.intervalHours),
                keyboardType: TextInputType.number,
              ),
            ],
          ),
        ),
        actions: [
          TextButton(
              onPressed: () => Navigator.pop(context, false),
              child: Text(loc.cancel)),
          FilledButton(
              onPressed: () => Navigator.pop(context, true),
              child: Text(loc.add)),
        ],
      ),
    );
    if (ok == true) {
      final heures = int.tryParse(intervalle.text.trim()) ?? 168;
      await ref.read(moteurProvider).ajouterPlanification(
            nom.text.trim().isEmpty ? loc.scheduling : nom.text.trim(),
            racines.text
                .split(';')
                .map((s) => s.trim())
                .where((s) => s.isNotEmpty)
                .toList(),
            heures * 3600,
          );
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(loc.ok)),
        );
      }
    }
  }

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final loc = AppLocalizations.of(context);
    final theme = ref.watch(themeProvider);
    final langue = ref.watch(langueProvider);
    final moteur = ref.read(moteurProvider);
    return Scaffold(
      appBar: AppBar(title: Text(loc.settings)),
      body: ListView(
        padding: const EdgeInsets.all(20),
        children: [
          // Thème (bouton segmenté en sous-titre : le `trailing` d'un
          // ListTile est trop étroit sur mobile — cf. BUGS.md B07).
          ListTile(
            leading: const Icon(Icons.palette),
            title: Text(loc.theme),
            subtitle: Padding(
              padding: const EdgeInsets.only(top: 8),
              child: SegmentedButton<ThemeMode>(
                segments: [
                  ButtonSegment(
                      value: ThemeMode.system, label: Text(loc.themeSystem)),
                  ButtonSegment(
                      value: ThemeMode.light, label: Text(loc.themeLight)),
                  ButtonSegment(
                      value: ThemeMode.dark, label: Text(loc.themeDark)),
                ],
                selected: {theme},
                onSelectionChanged: (s) =>
                    ref.read(themeProvider.notifier).state = s.first,
              ),
            ),
          ),
          // Langue.
          ListTile(
            leading: const Icon(Icons.language),
            title: Text(loc.language),
            trailing: DropdownButton<String>(
              value: langue,
              items: const [
                DropdownMenuItem(value: 'fr', child: Text('Français')),
                DropdownMenuItem(value: 'en', child: Text('English')),
              ],
              onChanged: (v) {
                if (v != null) {
                  ref.read(langueProvider.notifier).state = v;
                }
              },
            ),
          ),
          const Divider(),
          // Planifications.
          ListTile(
            leading: const Icon(Icons.schedule),
            title: Text(loc.scheduling),
            trailing: IconButton(
              tooltip: loc.add,
              icon: const Icon(Icons.add),
              onPressed: () => _ajouterPlanification(context, ref),
            ),
          ),
          _ListePlanifications(),
          const Divider(),
          // Mode de décision (P14) : Prudent coché par défaut, opt-in explicites.
          _TuileModeDecision(),
          const Divider(),
          // Mode jeu (v1 manuelle : détection auto de plein écran ticketée).
          _TuileModeJeu(),
          const Divider(),
          // Mises à jour (URL + clé publique Ed25519 via dialogue).
          ListTile(
            leading: const Icon(Icons.system_update),
            title: Text(loc.updateSignatures),
            trailing: IconButton(
              tooltip: loc.updateSignatures,
              icon: const Icon(Icons.refresh),
              onPressed: () async {
                final url = TextEditingController(
                    text: 'https://cleanx.local/depot/v1.json');
                final cle = TextEditingController();
                final ok = await showDialog<bool>(
                  context: context,
                  builder: (context) => AlertDialog(
                    title: Text(loc.updateSignatures),
                    content: Column(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        TextField(
                            controller: url,
                            decoration: InputDecoration(
                                labelText: loc.depotUrl)),
                        TextField(
                            controller: cle,
                            decoration: InputDecoration(
                                labelText: loc.clePublique)),
                      ],
                    ),
                    actions: [
                      TextButton(
                          onPressed: () => Navigator.pop(context, false),
                          child: Text(loc.cancel)),
                      FilledButton(
                          onPressed: () => Navigator.pop(context, true),
                          child: Text(loc.ok)),
                    ],
                  ),
                );
                if (ok != true) return;
                try {
                  final n = await moteur.mettreAJourSignatures(
                      url.text.trim(), cle.text.trim());
                  if (context.mounted) {
                    ScaffoldMessenger.of(context).showSnackBar(
                      SnackBar(
                          content: Text('${loc.signatures} : +$n')),
                    );
                  }
                } catch (e) {
                  if (context.mounted) {
                    ScaffoldMessenger.of(context).showSnackBar(
                      SnackBar(content: Text('$e')),
                    );
                  }
                }
                await ref.read(statutProvider.notifier).charger();
              },
            ),
          ),
          const Divider(),
          // À propos.
          ListTile(
            leading: const Icon(Icons.info_outline),
            title: Text(loc.about),
            subtitle: Text(loc.aboutText),
          ),
          FutureBuilder<String>(
            future: moteur.integriteBinaire(),
            builder: (context, snap) => ListTile(
              leading: const Icon(Icons.fingerprint),
              title: Text(loc.binaryFingerprint),
              subtitle: Text(
                (snap.data ?? '…'),
                style:
                    const TextStyle(fontFamily: 'monospace', fontSize: 11),
              ),
            ),
          ),
        ],
      ),
    );
  }
}

/// Sélecteur du mode de décision (P14) : radios avec descriptions,
/// Prudent coché par défaut. Pas de SegmentedButton : 4 options + textes.
class _TuileModeDecision extends ConsumerWidget {
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final loc = AppLocalizations.of(context);
    final mode = ref.watch(modeDecisionProvider);
    Widget option(ModeDecision valeur, String titre, String description) {
      return RadioListTile<ModeDecision>(
        value: valeur,
        title: Text(titre),
        subtitle:
            Text(description, style: const TextStyle(fontSize: 12)),
        dense: true,
      );
    }

    return Card(
      child: Padding(
        padding: const EdgeInsets.all(8),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            ListTile(
              leading: const Icon(Icons.policy),
              title: Text(loc.modeDecision,
                  style: const TextStyle(fontWeight: FontWeight.bold)),
            ),
            RadioGroup<ModeDecision>(
              groupValue: mode,
              onChanged: (v) {
                if (v != null) {
                  ref.read(modeDecisionProvider.notifier).definir(v);
                }
              },
              child: Column(
                children: [
                  option(ModeDecision.prudent, loc.modePrudent, loc.modePrudentDesc),
                  option(ModeDecision.automatique, loc.modeAuto, loc.modeAutoDesc),
                  option(
                      ModeDecision.agressif, loc.modeAgressif, loc.modeAgressifDesc),
                  option(ModeDecision.silencieux, loc.modeSilencieux,
                      loc.modeSilencieuxDesc),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

/// Interrupteur mode jeu (consommateur Riverpod dédié pour rebuild ciblé).
class _TuileModeJeu extends ConsumerWidget {
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final loc = AppLocalizations.of(context);
    final actif = ref.watch(modeJeuProvider);
    return ListTile(
      leading: const Icon(Icons.sports_esports),
      title: Text(loc.modeJeu),
      subtitle: Text(loc.modeJeuDesc),
      trailing: Switch.adaptive(
        value: actif,
        onChanged: (v) => ref.read(modeJeuProvider.notifier).basculer(v),
      ),
    );
  }
}

/// Liste des planifications avec suppression.
class _ListePlanifications extends ConsumerStatefulWidget {
  @override
  ConsumerState<_ListePlanifications> createState() =>
      __ListePlanificationsState();
}

class __ListePlanificationsState
    extends ConsumerState<_ListePlanifications> {
  late Future<List<PlanificationDto>> _futur;

  @override
  void initState() {
    super.initState();
    _futur = ref.read(moteurProvider).listerPlanifications();
  }

  @override
  Widget build(BuildContext context) {
    final loc = AppLocalizations.of(context);
    return FutureBuilder<List<PlanificationDto>>(
      future: _futur,
      builder: (context, snap) {
        final plans = snap.data ?? [];
        if (plans.isEmpty) {
          return ListTile(title: Text(loc.noItems));
        }
        return Column(
          children: [
            for (final p in plans)
              ListTile(
                title: Text(p.nom),
                subtitle: Text(
                    '${loc.scheduledFor} : ${DateTime.fromMillisecondsSinceEpoch(p.prochaineExec * 1000)}'),
                trailing: IconButton(
                  tooltip: loc.delete,
                  icon: const Icon(Icons.delete_outline),
                  onPressed: () async {
                    await ref
                        .read(moteurProvider)
                        .supprimerPlanification(p.id);
                    setState(() {
                      _futur = ref
                          .read(moteurProvider)
                          .listerPlanifications();
                    });
                  },
                ),
              ),
          ],
        );
      },
    );
  }
}
