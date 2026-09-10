# Adya Mobile

Flutter companion app for the Adya Technologies ERP — attendance (GPS + face
photo + geofence), calendar, meetings, messaging, leave, profile,
notifications, and read-only documents, all backed by the same Rust/axum API
the web app uses (see `../be`).

## Getting started

```bash
flutter pub get
flutter run --dart-define=API_BASE_URL=https://api-office.adyatech.com.np
```

Without `--dart-define`, the app defaults to
`https://api-office.adyatech.com.np`. For a local backend on an Android
emulator, use the emulator's host-loopback address instead:

```bash
flutter run --dart-define=API_BASE_URL=http://10.0.2.2:3117
```

## Architecture

- **State management:** Riverpod (`flutter_riverpod`), plain
  `AsyncNotifier`/`Notifier` classes — no code generation, so there's no
  `build_runner` step.
- **Navigation:** `go_router`, with a `StatefulShellRoute` for the five
  bottom tabs (Home, Calendar, Attendance, Chat, More) and everything else
  (Meetings, Leave, Documents, Notifications, Profile, chat threads) pushed
  full-screen on the root navigator.
- **Networking:** one shared `Dio` instance (`lib/core/network/api_client.dart`)
  that attaches the bearer token, and single-flights `/auth/refresh` so
  several 401s in flight at once don't each trigger their own refresh call.
- **Layout:** `lib/features/<feature>/{data,application,presentation}` —
  `data` talks to the API, `application` holds Riverpod controllers/state,
  `presentation` is widgets/screens.
- **Backend contract:** almost every screen calls a purpose-built
  `/mobile/...` endpoint added to the Rust backend (`be/src/api/mobile/`)
  rather than the admin-oriented endpoints the web console uses — those are
  gated by nav-permission RBAC and expect the caller to name whose records
  they want, neither of which fits "show me my own attendance/leave/
  schedule". The one exception is document browsing, which reuses the
  existing `/documents/{category}` service directly with the same
  permission check the web app's router applies.

## Push notifications (Firebase Cloud Messaging)

Push is wired end to end (device registration, foreground banners via
`flutter_local_notifications`, tap-through data) but ships **unconfigured** —
this repo does not include Firebase project credentials. Until you add them,
the app runs completely normally: the notifications tab still lists
everything (populated over `/notifications` + a live `/ws/notifications`
socket while the app is open), it just won't wake a backgrounded phone.

To turn push on:

1. Create a Firebase project (or reuse an existing one) and add an Android
   app with package name `com.adyatech.adya_mobile`.
2. Download `google-services.json` into `android/app/`.
3. Uncomment the two `com.google.gms.google-services` lines — one in
   `android/settings.gradle.kts`, one in `android/app/build.gradle.kts`.
4. On the backend, set one of:
   - `FCM_SERVICE_ACCOUNT_PATH=/path/to/service-account.json`, or
   - `FCM_SERVICE_ACCOUNT_JSON='{...}'` (the file's contents inline)

   plus `FCM_PROJECT_ID` (optional — it's read from the service account file
   if omitted). See `be/src/push.rs`.
5. For iOS, add an iOS app in the Firebase console, drop
   `GoogleService-Info.plist` into `ios/Runner/`, and in Xcode enable the
   **Push Notifications** and **Background Modes → Remote notifications**
   capabilities, then upload an APNs auth key to Firebase.

## CI builds (GitHub Actions)

Two workflows, both under `.github/workflows/`:

- **`mobile-apk.yml`** — an installable APK for handing to testers directly.
  Signed with the debug keystore (Android requires *some* signature, even
  for a throwaway build); runs on every push to `main` touching `mobile/`,
  on `mobile-v*` tags, and on demand. Needs one secret: `GOOGLE_SERVICES_JSON`.
- **`mobile-aab.yml`** — a properly signed Android App Bundle for the Play
  Store. Runs on `mobile-v*` tags and on demand. Needs `GOOGLE_SERVICES_JSON`
  plus four more secrets for release signing (below).

### Play Store releases

Google requires every Play Store app to be signed with the same key for
the life of the app — generate this **once** and never lose it:

```bash
keytool -genkeypair -v \
  -keystore upload-keystore.jks \
  -storetype JKS \
  -keyalg RSA -keysize 2048 -validity 10000 \
  -alias adya-upload
```

It'll prompt for a keystore password, your name/org details, and a key
password (can be the same as the keystore password). **Back this file up
somewhere durable and private that isn't just this machine** — a password
manager's file storage, an encrypted drive, etc. If you lose it, you lose
the ability to ever ship an update to that Play Store listing again; Google
cannot recover or reset it for you.

Then add these repo secrets (Settings → Secrets and variables → Actions):

| Secret | Value |
|---|---|
| `ANDROID_KEYSTORE_BASE64` | `base64 -w0 upload-keystore.jks` |
| `ANDROID_KEYSTORE_PASSWORD` | the keystore password you set above |
| `ANDROID_KEY_ALIAS` | `adya-upload` (or whatever `-alias` you used) |
| `ANDROID_KEY_PASSWORD` | the key password you set above |

Once those four plus `GOOGLE_SERVICES_JSON` are set, push a tag to trigger
a release build:

```bash
git tag mobile-v1.0.0
git push origin mobile-v1.0.0
```

Download the `.aab` from the workflow run's Artifacts section and upload it
to the Play Console yourself — Google requires the *first* release of a new
app listing to go through the console UI regardless of how it was built;
there's no way to automate around that one. Once the app exists in the
Play Console, publishing *subsequent* updates can be automated on top of
this workflow (e.g. with `r0adkll/upload-google-play`), which needs a Play
Developer API service account created from the console — a reasonable next
step once you've done the first manual upload, not before.

Locally, `flutter build appbundle`/`flutter build apk --release` continue
to work without any of this — they just fall back to debug signing, same
as before this was added, since `android/key.properties` won't exist on
your machine unless you create it yourself.

## Permissions requested at runtime

- **Camera** — check-in/out selfie.
- **Location (while in use)** — geofence validation for check-in/out.
- **Biometrics** — optional app-lock (Profile → Security), independent of
  login; the session token doesn't change when this is toggled.
- **Notifications** — Android 13+/iOS prompt, requested once a device
  registers for push.

## Known v1 scope decisions

- **Meeting video calls:** the backend's meetings are ad-hoc WebRTC calls
  signaled over `/ws/meetings/{id}` from the web app. The mobile app
  surfaces meeting details, participants, minutes (read-only, rendered from
  the same Tiptap HTML the web editor saves), and attachments, but does not
  implement an in-app WebRTC call UI — that's a materially separate scope
  from what was asked for ("meeting details: participants, link/location").
- **Reactions:** `message_reactions` existed as a schema-only, unused table
  before this change. Toggling an emoji reaction is now a real endpoint
  (`POST /messaging/channels/{channelId}/messages/{messageId}/reactions`),
  used by both the message list and the mobile app.
- **Client document picking:** Client Documents needs to know *which*
  client; the app adds a small `/mobile/documents/clients` endpoint (name +
  id only) gated on the same Client Documents permission, so a user doesn't
  need the separate Client Management permission just to browse files.
