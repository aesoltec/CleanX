# rust_builder — intégration mobile (Android/iOS)

## Option A (recommandée v1) : scripts + cargo-ndk
- Android : `scripts/build_android.sh` (jniLibs) — sans plugin.
- iOS : cibles `aarch64-apple-ios` + `x86_64-apple-ios`, `.a` statique liée via
  Xcode (`scripts/build_ios.sh` — ticket Cycle 6, machine macOS requise).

## Option B : cargokit
Si le projet adopte [cargokit](https://github.com/irondash/cargokit) :
1. `flutter pub add cargokit` (côté `app/`) ;
2. déplacer la logique `scripts/build_android.sh` dans `rust_builder/cargokit.yaml` ;
3. référencer le plugin dans `app/pubspec.yaml` (`cargokit:`) et
   `android/settings.gradle`.
- **Décision** : reporté (ADR à venir) — l'option A couvre le besoin v1 sans
  dépendance supplémentaire ; le `crate-type` inclut déjà `staticlib` pour iOS.
