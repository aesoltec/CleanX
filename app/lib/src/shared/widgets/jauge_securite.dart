/// Jauge circulaire du score de sécurité (0–100), avec contraste AA.
library;

import 'package:flutter/material.dart';

import '../../core/theme/theme.dart';

/// Jauge animée : verte ≥ 80, orange 50–79, rouge < 50.
class JaugeSecurite extends StatelessWidget {
  final int score;
  const JaugeSecurite({super.key, required this.score});

  Color _couleur() {
    if (score >= 80) return CleanXCouleurs.vertSecurite;
    if (score >= 50) return CleanXCouleurs.orangeAvertissement;
    return CleanXCouleurs.rougeAlerte;
  }

  @override
  Widget build(BuildContext context) {
    return Semantics(
      label: 'Score de sécurité $score sur 100',
      child: TweenAnimationBuilder<double>(
        tween: Tween(begin: 0, end: score.clamp(0, 100).toDouble()),
        duration: const Duration(milliseconds: 800),
        builder: (context, valeur, _) => CustomPaint(
          size: const Size(140, 140),
          painter: _PeintreJauge(valeur / 100, _couleur()),
          child: Center(
            child: Text(
              '${valeur.round()}',
              style: Theme.of(context)
                  .textTheme
                  .displaySmall
                  ?.copyWith(fontWeight: FontWeight.bold),
            ),
          ),
        ),
      ),
    );
  }
}

class _PeintreJauge extends CustomPainter {
  final double ratio;
  final Color couleur;
  _PeintreJauge(this.ratio, this.couleur);

  @override
  void paint(Canvas canvas, Size size) {
    final centre = size.center(Offset.zero);
    final rayon = size.width / 2 - 10;
    final fond = Paint()
      ..color = couleur.withValues(alpha: 0.15)
      ..style = PaintingStyle.stroke
      ..strokeWidth = 14
      ..strokeCap = StrokeCap.round;
    final valeur = Paint()
      ..color = couleur
      ..style = PaintingStyle.stroke
      ..strokeWidth = 14
      ..strokeCap = StrokeCap.round;
    canvas.drawCircle(centre, rayon, fond);
    canvas.drawArc(
      Rect.fromCircle(center: centre, radius: rayon),
      -3.14159 / 2,
      ratio * 2 * 3.14159,
      false,
      valeur,
    );
  }

  @override
  bool shouldRepaint(_PeintreJauge old) =>
      old.ratio != ratio || old.couleur != couleur;
}
