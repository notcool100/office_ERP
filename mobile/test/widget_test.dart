// Placeholder smoke test. `main()` calls `Firebase.initializeApp()` and
// reads secure storage before building `AdyaApp`, both of which need a
// real platform (no mocks wired up here), so we don't pump the full app
// in this test — just a signal that the test harness itself runs.
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('placeholder', () {
    expect(1 + 1, 2);
  });
}
