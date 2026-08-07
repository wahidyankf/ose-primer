import 'package:flutter_test/flutter_test.dart';
import 'package:web/web.dart';

import 'package:crud_fe_dart_flutterweb/pages/home_page.dart' as home_page;

void main() {
  test('landing page renders its reference role for readers', () {
    final root = document.createElement('div') as HTMLDivElement;
    document.body?.appendChild(root);

    try {
      home_page.render(root);

      // @covers specs/apps/crud/behavior/crud-web/gherkin/reference/reference-role.feature:Landing page explains its reference role
      expect(root.textContent, contains('A reusable reference application'));
      expect(
        root.textContent,
        contains(
          "Explore this working example, then adapt it to fit your team's product.",
        ),
      );
    } finally {
      root.remove();
    }
  });
}
