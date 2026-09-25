/// État global Riverpod : moteur, statut, scan, protection, quarantaine, logs.
///
/// Chaque notifier dépend du contrat [MoteurCleanX] (injecté via
/// `moteurProvider`, surchargé en prod/tests) : l'UI ne connaît jamais
/// l'implémentation (simulée ou FFI réelle).
library;

import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../engine/moteur.dart';

/// Moteur injecté (surcharge obligatoire dans `main` et les tests).
final moteurProvider = Provider<MoteurCleanX>(
  (ref) => throw UnimplementedError('moteurProvider non surchargé'),
);

// ------------------------------------------------------------- Statut
/// Notifier du statut global (dashboard).
class StatutNotifier extends StateNotifier<AsyncValue<StatutMoteur>> {
  final MoteurCleanX _moteur;
  StatutNotifier(this._moteur) : super(const AsyncLoading());

  /// Charge (ou recharge) le statut depuis le moteur.
  Future<void> charger() async {
    state = const AsyncLoading();
    state = await AsyncValue.guard(() => _moteur.statut());
  }
}

final statutProvider =
    StateNotifierProvider<StatutNotifier, AsyncValue<StatutMoteur>>(
  (ref) => StatutNotifier(ref.watch(moteurProvider)),
);

// ------------------------------------------------------------- Scan
/// État observable d'un scan.
class ScanState {
  final bool enCours;
  final bool enPause;
  final double progression;
  final String fichier;
  final int traites;
  final int total;
  final List<MenaceMoteur> menaces;
  final String? scanId;
  final bool? annule;
  final String? erreur;

  const ScanState({
    this.enCours = false,
    this.enPause = false,
    this.progression = 0,
    this.fichier = '',
    this.traites = 0,
    this.total = 0,
    this.menaces = const [],
    this.scanId,
    this.annule,
    this.erreur,
  });

  ScanState copier({
    bool? enCours,
    bool? enPause,
    double? progression,
    String? fichier,
    int? traites,
    int? total,
    List<MenaceMoteur>? menaces,
    String? scanId,
    bool? annule,
    String? erreur,
  }) {
    return ScanState(
      enCours: enCours ?? this.enCours,
      enPause: enPause ?? this.enPause,
      progression: progression ?? this.progression,
      fichier: fichier ?? this.fichier,
      traites: traites ?? this.traites,
      total: total ?? this.total,
      menaces: menaces ?? this.menaces,
      scanId: scanId ?? this.scanId,
      annule: annule ?? this.annule,
      erreur: erreur,
    );
  }
}

/// Pilote un scan : souscription au stream moteur, pause locale + moteur,
/// arrêt, journalisation vers [logsProvider].
class ScanNotifier extends StateNotifier<ScanState> {
  final MoteurCleanX _moteur;
  final Ref _ref;
  StreamSubscription<EvenementMoteur>? _abo;

  ScanNotifier(this._moteur, this._ref) : super(const ScanState());

  /// Lance un scan (fabrique = scanRapide/scanComplet/scanPersonnalise).
  Future<void> lancer(
      Stream<EvenementMoteur> Function() fabrique) async {
    if (state.enCours) return;
    await _abo?.cancel();
    state = const ScanState(enCours: true);
    try {
      _abo = fabrique().listen(
        _evenement,
        onError: (Object e) =>
            state = state.copier(enCours: false, erreur: e.toString()),
      );
    } catch (e) {
      state = state.copier(enCours: false, erreur: e.toString());
    }
  }

  void _evenement(EvenementMoteur e) {
    final logs = _ref.read(logsProvider.notifier);
    switch (e) {
      case ProgressionMoteur():
        state = state.copier(
          progression: e.ratio,
          fichier: e.fichier,
          traites: e.traites,
          total: e.total,
          scanId: e.scanId,
        );
      case MenaceMoteur():
        state = state.copier(menaces: [...state.menaces, e]);
        logs.ajouterLocal('quarantaine', '${e.fichier} — ${e.action}');
      case ScanTermineMoteur():
        state = state.copier(enCours: false, enPause: false, annule: e.annule);
        logs.ajouterLocal('info',
            'Scan ${e.scanId} terminé : ${e.total} fichiers, ${e.menaces} menace(s)');
        _ref.read(statutProvider.notifier).charger();
      case JournalMoteur():
        logs.ajouterLocal('info', e.message);
      case ProtectionMoteur():
        break; // hors contexte scan
    }
  }

  /// Demande l'arrêt (le moteur émet `ScanTermine(annule: true)`).
  Future<void> arreter() async {
    try {
      await _moteur.arreterScan();
    } catch (e) {
      state = state.copier(enCours: false, erreur: e.toString());
    }
  }

  /// Bascule pause/reprise (pause locale du stream + consigne moteur).
  Future<void> basculerPause() async {
    if (!state.enCours) return;
    final pause = !state.enPause;
    if (pause) {
      _abo?.pause();
    } else {
      _abo?.resume();
    }
    try {
      await _moteur.suspendreScan(pause);
    } catch (_) {
      // Le simule ignore : la pause locale suffit en démo.
    }
    state = state.copier(enPause: pause);
  }

  @override
  void dispose() {
    _abo?.cancel();
    super.dispose();
  }
}

final scanProvider = StateNotifierProvider<ScanNotifier, ScanState>(
  (ref) => ScanNotifier(ref.watch(moteurProvider), ref),
);

// ------------------------------------------------------------- Protection
/// Interrupteur temps réel : gère la souscription longue au stream moteur.
class ProtectionNotifier extends StateNotifier<bool> {
  final MoteurCleanX _moteur;
  final Ref _ref;
  StreamSubscription<EvenementMoteur>? _abo;

  ProtectionNotifier(this._moteur, this._ref) : super(false);

  /// Synchronise avec le moteur au démarrage.
  Future<void> synchroniser() async {
    try {
      state = (await _moteur.statut()).protection;
    } catch (_) {
      state = false;
    }
  }

  /// Bascule ON/OFF (optimiste, confirmée par les événements).
  Future<void> basculer(bool active) async {
    final logs = _ref.read(logsProvider.notifier);
    if (active) {
      state = true;
      try {
        await _abo?.cancel();
        _abo = _moteur.activerProtection().listen((e) {
          switch (e) {
            case ProtectionMoteur():
              state = e.active;
            case MenaceMoteur():
              logs.ajouterLocal(
                  'quarantaine', 'Temps réel : ${e.fichier} — ${e.action}');
              _ref.read(statutProvider.notifier).charger();
            case JournalMoteur():
              logs.ajouterLocal('info', e.message);
            default:
              break;
          }
        }, onError: (Object e) {
          state = false;
          logs.ajouterLocal('alerte', 'Protection interrompue : $e');
        });
      } catch (e) {
        state = false;
        logs.ajouterLocal('alerte', 'Activation impossible : $e');
      }
    } else {
      try {
        await _moteur.desactiverProtection();
      } catch (_) {
        // Idempotent : on coupe quand même côté UI.
      }
      await _abo?.cancel();
      state = false;
      logs.ajouterLocal('info', 'Protection temps réel désactivée');
      _ref.read(statutProvider.notifier).charger();
    }
  }

  @override
  void dispose() {
    _abo?.cancel();
    super.dispose();
  }
}

final protectionProvider =
    StateNotifierProvider<ProtectionNotifier, bool>(
  (ref) => ProtectionNotifier(ref.watch(moteurProvider), ref),
);

// ------------------------------------------------------------- Quarantaine
/// Liste des fichiers isolés + actions (recharge après chaque action).
class QuarantaineNotifier
    extends StateNotifier<AsyncValue<List<FichierQuarantaineDto>>> {
  final MoteurCleanX _moteur;
  QuarantaineNotifier(this._moteur)
      : super(const AsyncValue.data([]));

  Future<void> charger() async {
    state = const AsyncLoading();
    state = await AsyncValue.guard(() => _moteur.listerQuarantaine());
  }

  Future<void> restaurer(int id, String destination) async {
    await _moteur.restaurerQuarantaine(id, destination);
    await charger();
  }

  Future<void> supprimer(int id) async {
    await _moteur.supprimerQuarantaine(id);
    await charger();
  }
}

final quarantaineProvider =
    StateNotifierProvider<QuarantaineNotifier, AsyncValue<List<FichierQuarantaineDto>>>(
  (ref) => QuarantaineNotifier(ref.watch(moteurProvider)),
);

// ------------------------------------------------------------- Dossiers
/// Dossiers surveillés (persistés côté moteur).
class DossiersNotifier extends StateNotifier<AsyncValue<List<String>>> {
  final MoteurCleanX _moteur;
  DossiersNotifier(this._moteur) : super(const AsyncValue.data([]));

  Future<void> charger() async {
    state = const AsyncLoading();
    state = await AsyncValue.guard(() => _moteur.dossiersSurveilles());
  }

  Future<void> ajouter(String dossier) async {
    state = await AsyncValue.guard(() => _moteur.ajouterDossier(dossier));
  }

  Future<void> retirer(String dossier) async {
    state = await AsyncValue.guard(() => _moteur.retirerDossier(dossier));
  }
}

final dossiersProvider =
    StateNotifierProvider<DossiersNotifier, AsyncValue<List<String>>>(
  (ref) => DossiersNotifier(ref.watch(moteurProvider)),
);

// ------------------------------------------------------------- Logs
/// Journal UI : événements persistés + événements locaux de session.
class LogsNotifier extends StateNotifier<List<EntreeLogDto>> {
  int _compteur = -1;
  LogsNotifier() : super(const []);

  /// Recharge depuis le moteur (préserve les locaux négatifs en tête).
  Future<void> charger(MoteurCleanX moteur) async {
    try {
      final distants = await moteur.listerLogs(200);
      state = [...state.where((l) => l.id < 0), ...distants];
    } catch (_) {
      // Moteur indisponible : on garde le journal local.
    }
  }

  /// Ajoute un événement de session (id local négatif, `HH:MM:SS`).
  void ajouterLocal(String niveau, String message) {
    final maintenant = DateTime.now();
    final heure = [
      maintenant.hour,
      maintenant.minute,
      maintenant.second
    ].map((n) => n.toString().padLeft(2, '0')).join(':');
    state = [
      EntreeLogDto(id: _compteur--, date: heure, niveau: niveau, message: message),
      ...state,
    ].take(300).toList();
  }
}

final logsProvider =
    StateNotifierProvider<LogsNotifier, List<EntreeLogDto>>(
  (ref) => LogsNotifier(),
);

/// Filtre de niveau du journal : tous | info | alerte | quarantaine.
final filtreLogsProvider = StateProvider<String>((ref) => 'tous');

/// Recherche textuelle du journal.
final rechercheLogsProvider = StateProvider<String>((ref) => '');

// ------------------------------------------------------------- Préférences
/// Thème : système | clair | sombre.
final themeProvider = StateProvider<ThemeMode>((ref) => ThemeMode.system);

/// Langue : fr | en.
final langueProvider = StateProvider<String>((ref) => 'fr');

/// Mode jeu : scans suspendus + surveillance sans quarantaine auto.
/// Synchronisé avec le moteur à chaque bascule.
class ModeJeuNotifier extends StateNotifier<bool> {
  final MoteurCleanX _moteur;
  ModeJeuNotifier(this._moteur) : super(false);

  Future<void> synchroniser() async {
    try {
      state = await _moteur.modeJeuActif();
    } catch (_) {
      state = false;
    }
  }

  Future<void> basculer(bool actif) async {
    try {
      await _moteur.modeJeu(actif);
      state = actif;
    } catch (_) {
      // Moteur indisponible : état local conservé tel quel.
    }
  }
}

final modeJeuProvider = StateNotifierProvider<ModeJeuNotifier, bool>(
  (ref) => ModeJeuNotifier(ref.watch(moteurProvider)),
);

/// Processus suspects (base anti-rootkit) : rafraîchi à la demande.
final processusSuspectsProvider =
    FutureProvider<List<ProcessusDto>>((ref) async {
  final tous = await ref.watch(moteurProvider).listerProcessus();
  return tous.where((p) => p.suspect).toList();
});
