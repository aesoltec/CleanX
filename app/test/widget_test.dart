// Test de fumée : la coquille démarre avec le moteur simulé.
import 'package:cleanx_ui/main.dart';
import 'package:cleanx_ui/src/core/engine/moteur_simule.dart';
import 'package:cleanx_ui/src/core/providers/providers.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  testWidgets('CleanX démarre sur le dashboard', (tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [moteurProvider.overrideWithValue(MoteurSimule())],
        child: const CleanXApp(),
      ),
    );
    await tester.pumpAndSettle();
    // Destination de navigation + contenu dashboard chargés.
    expect(find.text('Tableau de bord'), findsWidgets);
    expect(find.textContaining('Score de sécurité'), findsOneWidget);
  });
}
