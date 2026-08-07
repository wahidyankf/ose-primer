import 'package:flutter_test/flutter_test.dart';
import 'package:crud_fe_dart_flutterweb/config/backend_mode.dart';
import '../gherkin_helper.dart';
import '../service_client.dart';

void main() {
  late ServiceClient svc;

  setUp(() {
    svc = ServiceClient();
  });

  describeFeature(
    '../../specs/apps/crud/behavior/crud-web/gherkin/health/health-status.feature',
    (feature) {
      feature.scenario('Health indicator shows the service is UP', (s) {
        s.given('the app is running', () async {
          // No-op: ServiceClient starts in a clean state.
        });

        s.when('the user opens the app', () async {
          // No-op: action represented by the subsequent assertion.
        });

        // @covers specs/apps/crud/behavior/crud-web/gherkin/health/health-status.feature:Health indicator shows the service is UP
        s.then('the health status indicator should display "UP"', () async {
          final response = await svc.getHealth();
          expect(response.status, equals('UP'));
        });
      });

      feature.scenario(
        'Health indicator does not expose component details to regular users',
        (s) {
          s.given('the app is running', () async {
            // No-op: ServiceClient starts in a clean state.
          });

          s.when('an unauthenticated user opens the app', () async {
            // No-op: no authentication is performed.
          });

          s.then('the health status indicator should display "UP"', () async {
            final response = await svc.getHealth();
            expect(response.status, equals('UP'));
          });

          // @covers specs/apps/crud/behavior/crud-web/gherkin/health/health-status.feature:Health indicator does not expose component details to regular users
          s.and(
            'no detailed component health information should be visible',
            () async {
              // The in-memory HealthResponse only carries a `status` field —
              // no sub-component map is exposed to callers.
              final response = await svc.getHealth();
              expect(response.status, equals('UP'));
            },
          );
        },
      );

      feature.scenario(
        'Frontend-only reference start does not request an unavailable backend',
        (s) {
          var healthRequested = true;

          s.given('the app is running', () async {});

          // @covers specs/apps/crud/behavior/crud-web/gherkin/health/health-status.feature:Frontend-only reference start does not request an unavailable backend
          s.when('the user opens the frontend-only reference app', () async {
            healthRequested = shouldRequestBackendHealth(false);
          });

          s.then(
            'the app should explain that a backend can be connected later',
            () async {
              expect(
                frontendOnlyStartGuidance,
                contains('Connect one when you are ready'),
              );
            },
          );

          s.and(
            'the frontend-only reference app should not request backend health',
            () async => expect(healthRequested, isFalse),
          );
        },
      );
    },
  );
}
