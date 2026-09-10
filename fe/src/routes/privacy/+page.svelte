<script lang="ts">
    import logo from '$lib/images/logo.jpeg';

    const sections = [
        { id: 'collect', label: 'What we collect' },
        { id: 'use', label: 'How we use it' },
        { id: 'share', label: 'Who we share it with' },
        { id: 'security', label: 'Security' },
        { id: 'retention', label: 'Retention' },
        { id: 'rights', label: 'Your choices' },
        { id: 'children', label: 'Children' },
        { id: 'changes', label: 'Changes' },
        { id: 'contact', label: 'Contact' }
    ];

    const dataCategories = [
        {
            icon: 'M12 8.5a3.3 3.3 0 1 0 0-6.6 3.3 3.3 0 0 0 0 6.6ZM5.3 19.5a6.7 6.7 0 0 1 13.4 0',
            title: 'Account & profile',
            body: 'Name, work email, phone, employee ID, department, position, and manager — sourced from your employer\'s existing HR records, not collected fresh by the app.',
            why: 'To sign you in and show your own record.'
        },
        {
            icon: 'M12 21s6.5-6.1 6.5-11A6.5 6.5 0 0 0 5.5 10c0 4.9 6.5 11 6.5 11Zm0-8.9a2.1 2.1 0 1 0 0-4.2 2.1 2.1 0 0 0 0 4.2Z',
            title: 'Precise location',
            body: 'GPS coordinates captured at the moment you check in or check out.',
            why: 'To confirm you\'re within the office geofence. Not tracked in the background or at any other time.'
        },
        {
            icon: 'M4 8.5A1.5 1.5 0 0 1 5.5 7h2l1-2h7l1 2h2A1.5 1.5 0 0 1 20 8.5v9A1.5 1.5 0 0 1 18.5 19h-13A1.5 1.5 0 0 1 4 17.5ZM12 15.7a3.2 3.2 0 1 0 0-6.4 3.2 3.2 0 0 0 0 6.4Z',
            title: 'Check-in photo',
            body: 'A single photo taken by your camera at check-in/out time, stored with that attendance record.',
            why: 'HR verification that attendance was recorded by the account holder.'
        },
        {
            icon: 'M12 2a5 5 0 0 0-5 5v3a3 3 0 0 0 3 3h4a3 3 0 0 0 3-3V7a5 5 0 0 0-5-5ZM6 12v1a6 6 0 0 0 12 0v-1M12 19v3',
            title: 'Biometric unlock',
            body: 'If you turn on Face ID / fingerprint app-lock, your device\'s own operating system handles the scan.',
            why: 'Convenience re-entry into the app. Adya never receives or stores any biometric data — the app only ever gets a yes/no answer from your OS.',
            badge: 'Never leaves device'
        },
        {
            icon: 'M4 6a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H9l-4 3.5V16H6a2 2 0 0 1-2-2Z',
            title: 'Messages & files',
            body: 'Text, images, and file attachments you send in the app\'s internal channels and direct messages.',
            why: 'To deliver and display your conversations. Visible to other members of the same channel, same as any workplace chat tool.'
        },
        {
            icon: 'M3.5 8h17v11a1.8 1.8 0 0 1-1.8 1.8H5.3A1.8 1.8 0 0 1 3.5 19ZM9 8V6a1.4 1.4 0 0 1 1.4-1.4h3.2A1.4 1.4 0 0 1 15 6v2',
            title: 'Leave & calendar',
            body: 'Leave requests (type, dates, reason), approval decisions, and calendar events you view or create.',
            why: 'To process your leave requests and show your schedule. Visible to your manager and HR as part of normal approval workflow.'
        },
        {
            icon: 'M4 7a1.5 1.5 0 0 1 1.5-1.5h4l1.6 2H18.5A1.5 1.5 0 0 1 20 9v8a1.5 1.5 0 0 1-1.5 1.5h-13A1.5 1.5 0 0 1 4 17Z',
            title: 'Company documents',
            body: 'Read access to whichever company or client documents your account has been granted permission to view.',
            why: 'Lets you browse and open files you\'re already authorized to see on the web console.'
        },
        {
            icon: 'M12 4.5a5 5 0 0 0-5 5v3.2c0 .6-.2 1.2-.6 1.7L5 16h14l-1.4-1.6a2.6 2.6 0 0 1-.6-1.7V9.5a5 5 0 0 0-5-5ZM10 18.5a2 2 0 0 0 4 0',
            title: 'Device & push token',
            body: 'A push-notification token issued by Google (Firebase Cloud Messaging) or Apple for your device, plus device model and OS version.',
            why: 'To deliver notifications when the app isn\'t open. Removed from our records when you log out.'
        }
    ];

    const shareRows = [
        {
            who: 'Your employer (Adya Technologies)',
            what: 'Everything listed above',
            why: 'The app is an internal company system — your employer is the data controller.'
        },
        {
            who: 'Google Firebase Cloud Messaging',
            what: 'Device push token, notification title/body',
            why: 'Delivering push notifications to Android devices.'
        },
        {
            who: 'Apple Push Notification service',
            what: 'Device push token, notification title/body',
            why: 'Delivering push notifications to iOS devices.'
        }
    ];

    const rights = [
        {
            title: 'Camera & location permission',
            body: 'Can be revoked in your device settings at any time. Doing so disables face-photo and GPS check-in — you can still be marked present manually by HR.'
        },
        {
            title: 'Push notifications',
            body: 'Can be turned off from your device\'s notification settings at any time; in-app notifications keep working either way.'
        },
        {
            title: 'Biometric app-lock',
            body: 'Entirely optional, off by default, and can be switched off in Profile → Security at any time.'
        },
        {
            title: 'Access, correction, or deletion',
            body: 'Contact HR or IT (below) to request a copy of your data, correct it, or ask about deletion — subject to records your employer must legally retain.'
        }
    ];
</script>

<svelte:head>
    <title>Privacy Policy — Adya</title>
    <meta
        name="description"
        content="How the Adya companion app collects and uses attendance, location, messaging, and account data." />
</svelte:head>

<div class="min-h-screen bg-base-200">
    <div class="mx-auto max-w-3xl px-4 py-14 sm:py-16">
        <!-- Header -->
        <header class="border-b border-base-300 pb-7">
            <div class="mb-5 flex items-center gap-3">
                <img src={logo} alt="Adya Technologies" class="h-9 w-9 rounded-lg object-cover" />
                <div>
                    <div class="text-sm font-bold leading-tight">Adya Technologies</div>
                    <div class="text-xs text-base-content/50">Companion mobile app</div>
                </div>
            </div>
            <h1 class="text-4xl font-extrabold tracking-tight text-balance">Privacy Policy</h1>
            <div class="mt-3 flex flex-wrap gap-x-5 gap-y-1.5 text-sm text-base-content/60">
                <span>Effective <span class="font-mono font-medium text-base-content">September 11, 2026</span></span>
                <span>Version <span class="font-mono font-medium text-base-content">1.0</span></span>
                <span>Applies to <span class="font-medium text-base-content">the Adya mobile app</span> (Android &amp; iOS)</span>
            </div>
        </header>

        <!-- Plain-language summary -->
        <div class="alert mt-7 flex-col items-start gap-3 bg-primary/10 text-base-content">
            <span class="text-xs font-bold uppercase tracking-wider text-primary">In plain terms</span>
            <p class="text-sm leading-relaxed">
                Adya is an internal companion app issued by Adya Technologies to its own employees for attendance,
                scheduling, internal messaging, and HR requests. This policy explains what the app collects and why.
            </p>
            <ul class="list-disc space-y-1.5 pl-5 text-sm leading-relaxed">
                <li>Your GPS location and a check-in photo are used <strong>only</strong> to confirm you're at the office when you clock in or out.</li>
                <li>Face ID / fingerprint unlock never leaves your device — Adya's servers never see it.</li>
                <li>We don't sell your data, and there are no advertising or analytics trackers in this app.</li>
                <li>Everything the app collects is tied to your employment and is visible to your employer, the same as any other HR system.</li>
            </ul>
        </div>

        <!-- Jump nav -->
        <nav class="my-7 flex flex-wrap gap-2 border-y border-base-300 py-4" aria-label="Sections">
            {#each sections as s}
                <a href="#{s.id}" class="badge badge-lg badge-outline hover:badge-primary">{s.label}</a>
            {/each}
        </nav>

        <!-- 01 What we collect -->
        <section id="collect" class="scroll-mt-6 border-b border-base-300 py-9">
            <h2 class="flex items-center gap-2.5 text-xl font-extrabold">
                <span class="font-mono text-sm font-semibold text-base-content/40">01</span>
                What we collect
            </h2>
            <p class="mt-3 max-w-prose text-sm leading-relaxed text-base-content/80">
                The app collects the categories below. Each is collected only where the corresponding feature is
                used — declining a permission simply turns off that one feature rather than the whole app.
            </p>

            <div class="mt-5 grid grid-cols-1 gap-3 sm:grid-cols-2">
                {#each dataCategories as c}
                    <div class="card border border-base-300 bg-base-100 shadow-sm">
                        <div class="card-body gap-2 p-4">
                            <div class="flex items-center gap-2.5">
                                <div class="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg bg-primary/10 text-primary">
                                    <svg viewBox="0 0 24 24" class="h-4 w-4" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d={c.icon} /></svg>
                                </div>
                                <span class="text-sm font-bold">{c.title}</span>
                                {#if c.badge}
                                    <span class="badge badge-success badge-sm ml-auto">{c.badge}</span>
                                {/if}
                            </div>
                            <p class="text-xs leading-relaxed text-base-content/70">{c.body}</p>
                            <p class="mt-1.5 border-t border-dashed border-base-300 pt-2 text-xs text-base-content/50">
                                <span class="font-semibold text-base-content/60">Why:</span> {c.why}
                            </p>
                        </div>
                    </div>
                {/each}
            </div>

            <h3 class="mt-6 text-sm font-bold">What we deliberately do not collect</h3>
            <p class="mt-1.5 max-w-prose text-sm leading-relaxed text-base-content/80">
                The app has no advertising SDKs, no third-party analytics or tracking libraries, and no background
                location tracking outside the moment you tap check-in or check-out.
            </p>
        </section>

        <!-- 02 How we use it -->
        <section id="use" class="scroll-mt-6 border-b border-base-300 py-9">
            <h2 class="flex items-center gap-2.5 text-xl font-extrabold">
                <span class="font-mono text-sm font-semibold text-base-content/40">02</span>
                How we use this information
            </h2>
            <ul class="mt-3 max-w-prose list-disc space-y-2 pl-5 text-sm leading-relaxed text-base-content/80">
                <li><strong class="text-base-content">Attendance verification</strong> — comparing check-in location against your office's registered coordinates, and storing the check-in photo as an auditable record.</li>
                <li><strong class="text-base-content">Authentication</strong> — issuing and refreshing the login session that keeps you signed in.</li>
                <li><strong class="text-base-content">HR workflows</strong> — routing leave requests to the right manager, applying approvals/rejections, and reflecting your schedule.</li>
                <li><strong class="text-base-content">Internal communication</strong> — delivering messages, meeting details, and reactions to the right people.</li>
                <li><strong class="text-base-content">Notifications</strong> — telling you about new messages, meeting activity, and leave status changes, including via push when the app is backgrounded.</li>
            </ul>
        </section>

        <!-- 03 Who we share it with -->
        <section id="share" class="scroll-mt-6 border-b border-base-300 py-9">
            <h2 class="flex items-center gap-2.5 text-xl font-extrabold">
                <span class="font-mono text-sm font-semibold text-base-content/40">03</span>
                Who we share it with
            </h2>
            <p class="mt-3 max-w-prose text-sm leading-relaxed text-base-content/80">
                We do not sell data or share it with advertisers. Information only flows to:
            </p>
            <div class="mt-4 overflow-x-auto rounded-lg border border-base-300">
                <table class="table">
                    <thead>
                        <tr class="bg-base-100">
                            <th>Recipient</th>
                            <th>What they receive</th>
                            <th>Purpose</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each shareRows as r}
                            <tr>
                                <td class="text-sm font-medium">{r.who}</td>
                                <td class="text-sm text-base-content/70">{r.what}</td>
                                <td class="text-sm text-base-content/70">{r.why}</td>
                            </tr>
                        {/each}
                    </tbody>
                </table>
            </div>
            <p class="mt-4 max-w-prose text-sm leading-relaxed text-base-content/80">
                No other third party — no ad network, no analytics vendor, no data broker — receives information
                from this app.
            </p>
        </section>

        <!-- 04 Security -->
        <section id="security" class="scroll-mt-6 border-b border-base-300 py-9">
            <h2 class="flex items-center gap-2.5 text-xl font-extrabold">
                <span class="font-mono text-sm font-semibold text-base-content/40">04</span>
                Security
            </h2>
            <ul class="mt-3 max-w-prose list-disc space-y-2 pl-5 text-sm leading-relaxed text-base-content/80">
                <li>All traffic between the app and our servers is encrypted in transit (HTTPS/TLS).</li>
                <li>Passwords are stored as salted <code class="font-mono text-xs">Argon2</code> hashes — never in plain text, and never visible to app developers.</li>
                <li>Your login session token is stored in your device's hardware-backed secure storage (Android Keystore / iOS Keychain), not in plain app storage.</li>
                <li>Access to employee records inside the app is scoped: you can see your own attendance, leave, and schedule, and only the messages, documents, and reports your role is granted.</li>
            </ul>
        </section>

        <!-- 05 Retention -->
        <section id="retention" class="scroll-mt-6 border-b border-base-300 py-9">
            <h2 class="flex items-center gap-2.5 text-xl font-extrabold">
                <span class="font-mono text-sm font-semibold text-base-content/40">05</span>
                Data retention
            </h2>
            <p class="mt-3 max-w-prose text-sm leading-relaxed text-base-content/80">
                Employment records — attendance, leave history, check-in photos — are retained for the duration of
                your employment and afterward only as long as your employer's record-keeping policy or applicable
                labor law requires. Messages and files persist until deleted by a channel member or an
                administrator. A push token is deleted the moment you log out of the app on that device.
            </p>
        </section>

        <!-- 06 Your choices -->
        <section id="rights" class="scroll-mt-6 border-b border-base-300 py-9">
            <h2 class="flex items-center gap-2.5 text-xl font-extrabold">
                <span class="font-mono text-sm font-semibold text-base-content/40">06</span>
                Your choices
            </h2>
            <div class="mt-4 flex flex-col gap-2.5">
                {#each rights as r}
                    <div class="rounded-lg border border-base-300 bg-base-100 p-3.5">
                        <div class="text-sm font-bold">{r.title}</div>
                        <div class="mt-0.5 text-sm text-base-content/70">{r.body}</div>
                    </div>
                {/each}
            </div>
        </section>

        <!-- 07 Children -->
        <section id="children" class="scroll-mt-6 border-b border-base-300 py-9">
            <h2 class="flex items-center gap-2.5 text-xl font-extrabold">
                <span class="font-mono text-sm font-semibold text-base-content/40">07</span>
                Children's privacy
            </h2>
            <p class="mt-3 max-w-prose text-sm leading-relaxed text-base-content/80">
                This app is an internal workforce tool issued only to employees of Adya Technologies and is not
                directed at, or knowingly used by, children.
            </p>
        </section>

        <!-- 08 Changes -->
        <section id="changes" class="scroll-mt-6 border-b border-base-300 py-9">
            <h2 class="flex items-center gap-2.5 text-xl font-extrabold">
                <span class="font-mono text-sm font-semibold text-base-content/40">08</span>
                Changes to this policy
            </h2>
            <p class="mt-3 max-w-prose text-sm leading-relaxed text-base-content/80">
                If this policy changes, we'll update the effective date above and notify staff through the usual
                internal channels before the change takes effect.
            </p>
        </section>

        <!-- 09 Contact -->
        <section id="contact" class="scroll-mt-6 py-9">
            <h2 class="flex items-center gap-2.5 text-xl font-extrabold">
                <span class="font-mono text-sm font-semibold text-base-content/40">09</span>
                Contact us
            </h2>
            <p class="mt-3 text-sm leading-relaxed text-base-content/80">
                Questions about this policy or how your data is handled:
            </p>
            <div class="card mt-4 border border-base-300 bg-base-100">
                <div class="card-body p-5">
                    <div class="text-xs font-bold uppercase tracking-wider text-base-content/40">Privacy contact</div>
                    <a href="mailto:privacy@adyatech.com.np" class="text-lg font-bold text-primary hover:underline">privacy@adyatech.com.np</a>
                    <div class="text-sm text-base-content/60">Adya Technologies</div>
                </div>
            </div>
        </section>

        <footer class="mt-8 flex flex-wrap justify-between gap-2 border-t border-base-300 pt-5 text-xs text-base-content/40">
            <span>© 2026 Adya Technologies</span>
            <span>Adya mobile app — Privacy Policy v1.0</span>
        </footer>
    </div>
</div>
