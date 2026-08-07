import '../gherkin_helper.dart';

void main() {
  describeFeature(
    '../../specs/apps/crud/behavior/crud-web/gherkin/reference/reference-role.feature',
    (feature) {
      feature.scenario('Landing page explains its reference role', (s) {
        s.given('the demo application is running', () async {});
        s.when('a reader opens the demo application', () async {});
        s.then(
          'the landing page should identify itself as a reusable reference application',
          () async {},
        );
        s.and(
          "it should explain that the example can be adapted for a team's product",
          () async {},
        );
      });
    },
  );
}
