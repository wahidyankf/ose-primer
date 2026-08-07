/// Whether this compiled frontend should contact its backend.
///
/// A reader can run the reference app without configuring a backend first.
/// Configured builds opt in with `--dart-define=BACKEND_ENABLED=true`.
const frontendBackendEnabled = bool.fromEnvironment(
  'BACKEND_ENABLED',
  defaultValue: false,
);

const frontendOnlyStartGuidance =
    'This frontend-only reference starts without a backend. Connect one when you are ready to explore the full flow.';

bool shouldRequestBackendHealth(bool backendEnabled) => backendEnabled;
