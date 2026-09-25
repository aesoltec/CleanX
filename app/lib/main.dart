/// CleanX 2.0 — entrée applicative.
///
/// Le moteur est injecté via `moteurProvider` : [MoteurSimule] par défaut
/// (démo + tests, zéro lib native). Après génération des bindings
/// (`flutter_rust_bridge_codegen generate`), lancez avec
/// `--dart-define=CLEANX_FRB=true` pour le moteur Rust réel
/// (nécessite la lib native : voir `scripts/build_windows.ps1`).
library;

import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'src/core/engine/moteur.dart';
import 'src/core/engine/moteur_frb.dart';
import 'src/core/engine/moteur_simule.dart';
import 'src/core/l10n/arb/app_localizations.dart';
import 'src/core/providers/providers.dart';
import 'src/core/theme/theme.dart';
import 'src/features/coquille/coquille.dart';

/// `true` avec `--dart-define=CLEANX_FRB=true` (moteur Rust réel).
// ignore: unused_element — câblé avec `moteur_frb.dart` après génération des bindings.
const _frbActif = bool.fromEnvironment('CLEANX_FRB');

void main() {
  // Moteur réel (FFI) avec --dart-define=CLEANX_FRB=true, simule sinon.
  final MoteurCleanX moteur = _frbActif ? MoteurFrb() : MoteurSimule();
  runApp(
    ProviderScope(
      overrides: [moteurProvider.overrideWithValue(moteur)],
      child: const CleanXApp(),
    ),
  );
}

/// Racine Material 3 : thèmes, locales FR/EN, coquille responsive.
class CleanXApp extends ConsumerWidget {
  const CleanXApp({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final theme = ref.watch(themeProvider);
    final langue = ref.watch(langueProvider);
    assert(() {
      // Rappel compile-time du câblage FRB (voir `moteur_frb.dart`).
      if (_frbActif) debugPrint('CleanX : mode FFI demandé');
      return true;
    }());
    return MaterialApp(
      title: 'CleanX',
      debugShowCheckedModeBanner: false,
      theme: CleanXTheme.clair(),
      darkTheme: CleanXTheme.sombre(),
      themeMode: theme,
      locale: Locale(langue),
      localizationsDelegates: const [
        AppLocalizations.delegate,
        GlobalMaterialLocalizations.delegate,
        GlobalWidgetsLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate,
      ],
      supportedLocales: const [Locale('fr'), Locale('en')],
      home: const Coquille(),
    );
  }
}
