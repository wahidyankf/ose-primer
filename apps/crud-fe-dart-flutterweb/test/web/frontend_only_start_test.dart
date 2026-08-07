import 'package:flutter_test/flutter_test.dart';
import 'package:web/web.dart';

import 'package:crud_fe_dart_flutterweb/config/backend_mode.dart';
import 'package:crud_fe_dart_flutterweb/models/health.dart';
import 'package:crud_fe_dart_flutterweb/pages/home_page.dart' as home_page;

void main() {
  test(
    'frontend-only start explains the next step without checking health',
    () async {
      final root = document.createElement('div') as HTMLDivElement;
      document.body?.appendChild(root);
      var healthRequests = 0;

      try {
        home_page.render(
          root,
          backendEnabled: false,
          healthCheck: () async {
            healthRequests += 1;
            return const HealthResponse(status: 'UP');
          },
        );

        // @covers specs/apps/crud/behavior/crud-web/gherkin/health/health-status.feature:Frontend-only reference start does not request an unavailable backend
        expect(root.textContent, contains(frontendOnlyStartGuidance));
        expect(healthRequests, isZero);
      } finally {
        root.remove();
      }
    },
  );

  test('configured start checks backend health', () async {
    final root = document.createElement('div') as HTMLDivElement;
    document.body?.appendChild(root);
    var healthRequests = 0;

    try {
      home_page.render(
        root,
        backendEnabled: true,
        healthCheck: () async {
          healthRequests += 1;
          return const HealthResponse(status: 'UP');
        },
      );

      await Future<void>.delayed(Duration.zero);

      expect(healthRequests, 1);
      expect(root.textContent, contains('Backend Status'));
      expect(root.textContent, contains('UP'));
    } finally {
      root.remove();
    }
  });
}
