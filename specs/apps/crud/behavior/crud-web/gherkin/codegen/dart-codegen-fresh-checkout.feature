@codegen
Feature: Dart OpenAPI codegen on fresh checkout

  As a developer working on a clean checkout
  I want the Dart codegen target to produce a resolvable package
  So that the CI gate passes without pre-generated contracts committed to git

  Scenario: Fresh Dart codegen yields a usable crud_contracts package
    Given apps/crud-fe-dart-flutterweb/generated-contracts does not exist
    When nx run crud-fe-dart-flutterweb:codegen runs with --skip-nx-cache
    Then a pubspec.yaml exists at apps/crud-fe-dart-flutterweb/generated-contracts/
    And flutter pub get for crud-fe-dart-flutterweb resolves crud_contracts without error
    And nx run crud-fe-dart-flutterweb:lint exits 0
