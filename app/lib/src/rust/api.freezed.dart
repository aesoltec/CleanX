// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'api.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EvenementMoteur {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is EvenementMoteur);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EvenementMoteur()';
  }
}

/// @nodoc
class $EvenementMoteurCopyWith<$Res> {
  $EvenementMoteurCopyWith(
      EvenementMoteur _, $Res Function(EvenementMoteur) __);
}

/// Adds pattern-matching-related methods to [EvenementMoteur].
extension EvenementMoteurPatterns on EvenementMoteur {
  /// A variant of `map` that fallback to returning `orElse`.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case _:
  ///     return orElse();
  /// }
  /// ```

  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EvenementMoteur_Journal value)? journal,
    TResult Function(EvenementMoteur_Progression value)? progression,
    TResult Function(EvenementMoteur_Menace value)? menace,
    TResult Function(EvenementMoteur_ScanTermine value)? scanTermine,
    TResult Function(EvenementMoteur_Protection value)? protection,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EvenementMoteur_Journal() when journal != null:
        return journal(_that);
      case EvenementMoteur_Progression() when progression != null:
        return progression(_that);
      case EvenementMoteur_Menace() when menace != null:
        return menace(_that);
      case EvenementMoteur_ScanTermine() when scanTermine != null:
        return scanTermine(_that);
      case EvenementMoteur_Protection() when protection != null:
        return protection(_that);
      case _:
        return orElse();
    }
  }

  /// A `switch`-like method, using callbacks.
  ///
  /// Callbacks receives the raw object, upcasted.
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case final Subclass2 value:
  ///     return ...;
  /// }
  /// ```

  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EvenementMoteur_Journal value) journal,
    required TResult Function(EvenementMoteur_Progression value) progression,
    required TResult Function(EvenementMoteur_Menace value) menace,
    required TResult Function(EvenementMoteur_ScanTermine value) scanTermine,
    required TResult Function(EvenementMoteur_Protection value) protection,
  }) {
    final _that = this;
    switch (_that) {
      case EvenementMoteur_Journal():
        return journal(_that);
      case EvenementMoteur_Progression():
        return progression(_that);
      case EvenementMoteur_Menace():
        return menace(_that);
      case EvenementMoteur_ScanTermine():
        return scanTermine(_that);
      case EvenementMoteur_Protection():
        return protection(_that);
    }
  }

  /// A variant of `map` that fallback to returning `null`.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case _:
  ///     return null;
  /// }
  /// ```

  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EvenementMoteur_Journal value)? journal,
    TResult? Function(EvenementMoteur_Progression value)? progression,
    TResult? Function(EvenementMoteur_Menace value)? menace,
    TResult? Function(EvenementMoteur_ScanTermine value)? scanTermine,
    TResult? Function(EvenementMoteur_Protection value)? protection,
  }) {
    final _that = this;
    switch (_that) {
      case EvenementMoteur_Journal() when journal != null:
        return journal(_that);
      case EvenementMoteur_Progression() when progression != null:
        return progression(_that);
      case EvenementMoteur_Menace() when menace != null:
        return menace(_that);
      case EvenementMoteur_ScanTermine() when scanTermine != null:
        return scanTermine(_that);
      case EvenementMoteur_Protection() when protection != null:
        return protection(_that);
      case _:
        return null;
    }
  }

  /// A variant of `when` that fallback to an `orElse` callback.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case _:
  ///     return orElse();
  /// }
  /// ```

  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? journal,
    TResult Function(
            String scanId, String fichier, BigInt traites, BigInt total)?
        progression,
    TResult Function(
            String fichier, String menace, String action, String? sha256)?
        menace,
    TResult Function(String scanId, BigInt total, BigInt menaces, bool annule)?
        scanTermine,
    TResult Function(bool active)? protection,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EvenementMoteur_Journal() when journal != null:
        return journal(_that.message);
      case EvenementMoteur_Progression() when progression != null:
        return progression(
            _that.scanId, _that.fichier, _that.traites, _that.total);
      case EvenementMoteur_Menace() when menace != null:
        return menace(_that.fichier, _that.menace, _that.action, _that.sha256);
      case EvenementMoteur_ScanTermine() when scanTermine != null:
        return scanTermine(
            _that.scanId, _that.total, _that.menaces, _that.annule);
      case EvenementMoteur_Protection() when protection != null:
        return protection(_that.active);
      case _:
        return orElse();
    }
  }

  /// A `switch`-like method, using callbacks.
  ///
  /// As opposed to `map`, this offers destructuring.
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case Subclass2(:final field2):
  ///     return ...;
  /// }
  /// ```

  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) journal,
    required TResult Function(
            String scanId, String fichier, BigInt traites, BigInt total)
        progression,
    required TResult Function(
            String fichier, String menace, String action, String? sha256)
        menace,
    required TResult Function(
            String scanId, BigInt total, BigInt menaces, bool annule)
        scanTermine,
    required TResult Function(bool active) protection,
  }) {
    final _that = this;
    switch (_that) {
      case EvenementMoteur_Journal():
        return journal(_that.message);
      case EvenementMoteur_Progression():
        return progression(
            _that.scanId, _that.fichier, _that.traites, _that.total);
      case EvenementMoteur_Menace():
        return menace(_that.fichier, _that.menace, _that.action, _that.sha256);
      case EvenementMoteur_ScanTermine():
        return scanTermine(
            _that.scanId, _that.total, _that.menaces, _that.annule);
      case EvenementMoteur_Protection():
        return protection(_that.active);
    }
  }

  /// A variant of `when` that fallback to returning `null`
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case _:
  ///     return null;
  /// }
  /// ```

  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? journal,
    TResult? Function(
            String scanId, String fichier, BigInt traites, BigInt total)?
        progression,
    TResult? Function(
            String fichier, String menace, String action, String? sha256)?
        menace,
    TResult? Function(String scanId, BigInt total, BigInt menaces, bool annule)?
        scanTermine,
    TResult? Function(bool active)? protection,
  }) {
    final _that = this;
    switch (_that) {
      case EvenementMoteur_Journal() when journal != null:
        return journal(_that.message);
      case EvenementMoteur_Progression() when progression != null:
        return progression(
            _that.scanId, _that.fichier, _that.traites, _that.total);
      case EvenementMoteur_Menace() when menace != null:
        return menace(_that.fichier, _that.menace, _that.action, _that.sha256);
      case EvenementMoteur_ScanTermine() when scanTermine != null:
        return scanTermine(
            _that.scanId, _that.total, _that.menaces, _that.annule);
      case EvenementMoteur_Protection() when protection != null:
        return protection(_that.active);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EvenementMoteur_Journal extends EvenementMoteur {
  const EvenementMoteur_Journal({required this.message}) : super._();

  final String message;

  /// Create a copy of EvenementMoteur
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EvenementMoteur_JournalCopyWith<EvenementMoteur_Journal> get copyWith =>
      _$EvenementMoteur_JournalCopyWithImpl<EvenementMoteur_Journal>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EvenementMoteur_Journal &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  @override
  String toString() {
    return 'EvenementMoteur.journal(message: $message)';
  }
}

/// @nodoc
abstract mixin class $EvenementMoteur_JournalCopyWith<$Res>
    implements $EvenementMoteurCopyWith<$Res> {
  factory $EvenementMoteur_JournalCopyWith(EvenementMoteur_Journal value,
          $Res Function(EvenementMoteur_Journal) _then) =
      _$EvenementMoteur_JournalCopyWithImpl;
  @useResult
  $Res call({String message});
}

/// @nodoc
class _$EvenementMoteur_JournalCopyWithImpl<$Res>
    implements $EvenementMoteur_JournalCopyWith<$Res> {
  _$EvenementMoteur_JournalCopyWithImpl(this._self, this._then);

  final EvenementMoteur_Journal _self;
  final $Res Function(EvenementMoteur_Journal) _then;

  /// Create a copy of EvenementMoteur
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? message = null,
  }) {
    return _then(EvenementMoteur_Journal(
      message: null == message
          ? _self.message
          : message // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class EvenementMoteur_Progression extends EvenementMoteur {
  const EvenementMoteur_Progression(
      {required this.scanId,
      required this.fichier,
      required this.traites,
      required this.total})
      : super._();

  final String scanId;
  final String fichier;
  final BigInt traites;
  final BigInt total;

  /// Create a copy of EvenementMoteur
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EvenementMoteur_ProgressionCopyWith<EvenementMoteur_Progression>
      get copyWith => _$EvenementMoteur_ProgressionCopyWithImpl<
          EvenementMoteur_Progression>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EvenementMoteur_Progression &&
            (identical(other.scanId, scanId) || other.scanId == scanId) &&
            (identical(other.fichier, fichier) || other.fichier == fichier) &&
            (identical(other.traites, traites) || other.traites == traites) &&
            (identical(other.total, total) || other.total == total));
  }

  @override
  int get hashCode => Object.hash(runtimeType, scanId, fichier, traites, total);

  @override
  String toString() {
    return 'EvenementMoteur.progression(scanId: $scanId, fichier: $fichier, traites: $traites, total: $total)';
  }
}

/// @nodoc
abstract mixin class $EvenementMoteur_ProgressionCopyWith<$Res>
    implements $EvenementMoteurCopyWith<$Res> {
  factory $EvenementMoteur_ProgressionCopyWith(
          EvenementMoteur_Progression value,
          $Res Function(EvenementMoteur_Progression) _then) =
      _$EvenementMoteur_ProgressionCopyWithImpl;
  @useResult
  $Res call({String scanId, String fichier, BigInt traites, BigInt total});
}

/// @nodoc
class _$EvenementMoteur_ProgressionCopyWithImpl<$Res>
    implements $EvenementMoteur_ProgressionCopyWith<$Res> {
  _$EvenementMoteur_ProgressionCopyWithImpl(this._self, this._then);

  final EvenementMoteur_Progression _self;
  final $Res Function(EvenementMoteur_Progression) _then;

  /// Create a copy of EvenementMoteur
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? scanId = null,
    Object? fichier = null,
    Object? traites = null,
    Object? total = null,
  }) {
    return _then(EvenementMoteur_Progression(
      scanId: null == scanId
          ? _self.scanId
          : scanId // ignore: cast_nullable_to_non_nullable
              as String,
      fichier: null == fichier
          ? _self.fichier
          : fichier // ignore: cast_nullable_to_non_nullable
              as String,
      traites: null == traites
          ? _self.traites
          : traites // ignore: cast_nullable_to_non_nullable
              as BigInt,
      total: null == total
          ? _self.total
          : total // ignore: cast_nullable_to_non_nullable
              as BigInt,
    ));
  }
}

/// @nodoc

class EvenementMoteur_Menace extends EvenementMoteur {
  const EvenementMoteur_Menace(
      {required this.fichier,
      required this.menace,
      required this.action,
      this.sha256})
      : super._();

  final String fichier;
  final String menace;
  final String action;
  final String? sha256;

  /// Create a copy of EvenementMoteur
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EvenementMoteur_MenaceCopyWith<EvenementMoteur_Menace> get copyWith =>
      _$EvenementMoteur_MenaceCopyWithImpl<EvenementMoteur_Menace>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EvenementMoteur_Menace &&
            (identical(other.fichier, fichier) || other.fichier == fichier) &&
            (identical(other.menace, menace) || other.menace == menace) &&
            (identical(other.action, action) || other.action == action) &&
            (identical(other.sha256, sha256) || other.sha256 == sha256));
  }

  @override
  int get hashCode => Object.hash(runtimeType, fichier, menace, action, sha256);

  @override
  String toString() {
    return 'EvenementMoteur.menace(fichier: $fichier, menace: $menace, action: $action, sha256: $sha256)';
  }
}

/// @nodoc
abstract mixin class $EvenementMoteur_MenaceCopyWith<$Res>
    implements $EvenementMoteurCopyWith<$Res> {
  factory $EvenementMoteur_MenaceCopyWith(EvenementMoteur_Menace value,
          $Res Function(EvenementMoteur_Menace) _then) =
      _$EvenementMoteur_MenaceCopyWithImpl;
  @useResult
  $Res call({String fichier, String menace, String action, String? sha256});
}

/// @nodoc
class _$EvenementMoteur_MenaceCopyWithImpl<$Res>
    implements $EvenementMoteur_MenaceCopyWith<$Res> {
  _$EvenementMoteur_MenaceCopyWithImpl(this._self, this._then);

  final EvenementMoteur_Menace _self;
  final $Res Function(EvenementMoteur_Menace) _then;

  /// Create a copy of EvenementMoteur
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? fichier = null,
    Object? menace = null,
    Object? action = null,
    Object? sha256 = freezed,
  }) {
    return _then(EvenementMoteur_Menace(
      fichier: null == fichier
          ? _self.fichier
          : fichier // ignore: cast_nullable_to_non_nullable
              as String,
      menace: null == menace
          ? _self.menace
          : menace // ignore: cast_nullable_to_non_nullable
              as String,
      action: null == action
          ? _self.action
          : action // ignore: cast_nullable_to_non_nullable
              as String,
      sha256: freezed == sha256
          ? _self.sha256
          : sha256 // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc

class EvenementMoteur_ScanTermine extends EvenementMoteur {
  const EvenementMoteur_ScanTermine(
      {required this.scanId,
      required this.total,
      required this.menaces,
      required this.annule})
      : super._();

  final String scanId;
  final BigInt total;
  final BigInt menaces;
  final bool annule;

  /// Create a copy of EvenementMoteur
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EvenementMoteur_ScanTermineCopyWith<EvenementMoteur_ScanTermine>
      get copyWith => _$EvenementMoteur_ScanTermineCopyWithImpl<
          EvenementMoteur_ScanTermine>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EvenementMoteur_ScanTermine &&
            (identical(other.scanId, scanId) || other.scanId == scanId) &&
            (identical(other.total, total) || other.total == total) &&
            (identical(other.menaces, menaces) || other.menaces == menaces) &&
            (identical(other.annule, annule) || other.annule == annule));
  }

  @override
  int get hashCode => Object.hash(runtimeType, scanId, total, menaces, annule);

  @override
  String toString() {
    return 'EvenementMoteur.scanTermine(scanId: $scanId, total: $total, menaces: $menaces, annule: $annule)';
  }
}

/// @nodoc
abstract mixin class $EvenementMoteur_ScanTermineCopyWith<$Res>
    implements $EvenementMoteurCopyWith<$Res> {
  factory $EvenementMoteur_ScanTermineCopyWith(
          EvenementMoteur_ScanTermine value,
          $Res Function(EvenementMoteur_ScanTermine) _then) =
      _$EvenementMoteur_ScanTermineCopyWithImpl;
  @useResult
  $Res call({String scanId, BigInt total, BigInt menaces, bool annule});
}

/// @nodoc
class _$EvenementMoteur_ScanTermineCopyWithImpl<$Res>
    implements $EvenementMoteur_ScanTermineCopyWith<$Res> {
  _$EvenementMoteur_ScanTermineCopyWithImpl(this._self, this._then);

  final EvenementMoteur_ScanTermine _self;
  final $Res Function(EvenementMoteur_ScanTermine) _then;

  /// Create a copy of EvenementMoteur
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? scanId = null,
    Object? total = null,
    Object? menaces = null,
    Object? annule = null,
  }) {
    return _then(EvenementMoteur_ScanTermine(
      scanId: null == scanId
          ? _self.scanId
          : scanId // ignore: cast_nullable_to_non_nullable
              as String,
      total: null == total
          ? _self.total
          : total // ignore: cast_nullable_to_non_nullable
              as BigInt,
      menaces: null == menaces
          ? _self.menaces
          : menaces // ignore: cast_nullable_to_non_nullable
              as BigInt,
      annule: null == annule
          ? _self.annule
          : annule // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc

class EvenementMoteur_Protection extends EvenementMoteur {
  const EvenementMoteur_Protection({required this.active}) : super._();

  final bool active;

  /// Create a copy of EvenementMoteur
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EvenementMoteur_ProtectionCopyWith<EvenementMoteur_Protection>
      get copyWith =>
          _$EvenementMoteur_ProtectionCopyWithImpl<EvenementMoteur_Protection>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EvenementMoteur_Protection &&
            (identical(other.active, active) || other.active == active));
  }

  @override
  int get hashCode => Object.hash(runtimeType, active);

  @override
  String toString() {
    return 'EvenementMoteur.protection(active: $active)';
  }
}

/// @nodoc
abstract mixin class $EvenementMoteur_ProtectionCopyWith<$Res>
    implements $EvenementMoteurCopyWith<$Res> {
  factory $EvenementMoteur_ProtectionCopyWith(EvenementMoteur_Protection value,
          $Res Function(EvenementMoteur_Protection) _then) =
      _$EvenementMoteur_ProtectionCopyWithImpl;
  @useResult
  $Res call({bool active});
}

/// @nodoc
class _$EvenementMoteur_ProtectionCopyWithImpl<$Res>
    implements $EvenementMoteur_ProtectionCopyWith<$Res> {
  _$EvenementMoteur_ProtectionCopyWithImpl(this._self, this._then);

  final EvenementMoteur_Protection _self;
  final $Res Function(EvenementMoteur_Protection) _then;

  /// Create a copy of EvenementMoteur
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? active = null,
  }) {
    return _then(EvenementMoteur_Protection(
      active: null == active
          ? _self.active
          : active // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

// dart format on
