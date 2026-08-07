import 'dart:io';

import 'package:flutter_test/flutter_test.dart';

import '../gherkin_helper.dart';

void main() {
  describeFeature(
    '../../specs/apps/crud/behavior/crud-web/gherkin/reference/reference-role.feature',
    (feature) {
      feature.scenario('Landing page explains its reference role', (s) {
        s.given('the demo application is running', () async {});

        late String pageSource;

        s.when('a reader opens the demo application', () async {
          pageSource = await File('lib/pages/home_page.dart').readAsString();
        });

        // @covers specs/apps/crud/behavior/crud-web/gherkin/reference/reference-role.feature:Landing page explains its reference role
        s.then(
          'the landing page should identify itself as a reusable reference application',
          () async {
            expect(pageSource, contains('A reusable reference application'));
          },
        );

        s.and(
          "it should explain that the example can be adapted for a team's product",
          () async {
            expect(
              pageSource,
              contains(
                'Explore this working example, then adapt it to fit your team',
              ),
            );
            expect(pageSource, contains('product.'));
          },
        );
      });
    },
  );
}
