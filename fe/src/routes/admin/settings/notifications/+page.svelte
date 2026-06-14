<script lang="ts">
    import { onMount } from 'svelte';
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, Settings, Bell, X, Plus, Loader2 } from 'lucide-svelte';
    import {
        notificationSettingsService,
        type NotificationSetting,
    } from '$lib/services/notification-settings';
    import { userService } from '$lib/services/user-service';
    import type { User } from '$lib/types/user';

    pageTitle.set({ title: 'Notification Settings', desc: 'Configure who receives automated email notifications.' });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Settings', icon: Settings },
        { label: 'Notifications', icon: Bell },
    ]);

    let settings: NotificationSetting[] = [];
    let users: User[] = [];
    let loading = true;
    let saving: Record<string, boolean> = {};
    let errors: Record<string, string> = {};
    let success: Record<string, boolean> = {};

    // Per-setting draft state
    let drafts: Record<string, string[]> = {};
    let inputs: Record<string, string> = {};

    async function load() {
        loading = true;
        try {
            [settings, users] = await Promise.all([
                notificationSettingsService.list(),
                userService.getAll(),
            ]);
            for (const s of settings) {
                drafts[s.key] = [...s.emails];
                inputs[s.key] = '';
            }
        } catch (e: any) {
            console.error(e);
        } finally {
            loading = false;
        }
    }

    function addEmail(key: string) {
        const val = (inputs[key] ?? '').trim().toLowerCase();
        if (!val || !val.includes('@')) return;
        if (!drafts[key].includes(val)) {
            drafts[key] = [...drafts[key], val];
        }
        inputs[key] = '';
    }

    function addEmailOnEnter(key: string, e: KeyboardEvent) {
        if (e.key === 'Enter') {
            e.preventDefault();
            addEmail(key);
        }
    }

    function removeEmail(key: string, email: string) {
        drafts[key] = drafts[key].filter((e) => e !== email);
    }

    function addFromUser(key: string, userEmail: string) {
        if (!userEmail) return;
        const email = userEmail.trim().toLowerCase();
        if (email && !drafts[key].includes(email)) {
            drafts[key] = [...drafts[key], email];
        }
    }

    async function save(key: string) {
        saving[key] = true;
        errors[key] = '';
        success[key] = false;
        try {
            const updated = await notificationSettingsService.update(key, drafts[key]);
            drafts[key] = [...updated.emails];
            success[key] = true;
            setTimeout(() => { success[key] = false; }, 3000);
        } catch (e: any) {
            errors[key] = e.message ?? 'Failed to save';
        } finally {
            saving[key] = false;
        }
    }

    $: usersWithEmail = users.filter((u) => u.email);

    onMount(load);
</script>

{#if loading}
    <div class="flex justify-center p-12">
        <span class="loading loading-spinner loading-lg"></span>
    </div>
{:else}
    <div class="space-y-6 max-w-2xl">
        {#each settings as setting (setting.key)}
            <div class="card bg-base-100 border border-base-300 shadow-sm">
                <div class="card-body">
                    <div class="flex items-start gap-3 mb-4">
                        <div class="p-2 rounded-lg bg-primary/10 text-primary mt-0.5">
                            <Bell class="w-5 h-5" />
                        </div>
                        <div>
                            <h3 class="font-semibold text-base">{setting.label}</h3>
                            {#if setting.description}
                                <p class="text-sm text-base-content/60 mt-0.5">{setting.description}</p>
                            {/if}
                        </div>
                    </div>

                    <!-- Current recipients -->
                    <div class="mb-4">
                        <p class="text-xs font-medium text-base-content/50 uppercase tracking-wider mb-2">
                            Recipients ({drafts[setting.key]?.length ?? 0})
                        </p>
                        {#if drafts[setting.key]?.length}
                            <div class="flex flex-wrap gap-2">
                                {#each drafts[setting.key] as email}
                                    <span class="badge badge-outline gap-1.5 py-3 px-3 text-sm">
                                        {email}
                                        <button
                                            type="button"
                                            class="hover:text-error transition-colors"
                                            on:click={() => removeEmail(setting.key, email)}
                                            aria-label="Remove {email}">
                                            <X class="w-3 h-3" />
                                        </button>
                                    </span>
                                {/each}
                            </div>
                        {:else}
                            <p class="text-sm text-base-content/40 italic">
                                No recipients configured — notifications will not be sent.
                            </p>
                        {/if}
                    </div>

                    <!-- Add by typing email -->
                    <div class="flex gap-2 mb-3">
                        <input
                            type="email"
                            class="input input-bordered input-sm flex-1"
                            placeholder="Add email address..."
                            bind:value={inputs[setting.key]}
                            on:keydown={(e) => addEmailOnEnter(setting.key, e)} />
                        <button
                            type="button"
                            class="btn btn-sm btn-outline"
                            on:click={() => addEmail(setting.key)}>
                            <Plus class="w-4 h-4" />
                            Add
                        </button>
                    </div>

                    <!-- Pick from existing users -->
                    {#if usersWithEmail.length}
                        <div class="mb-4">
                            <label class="text-xs text-base-content/50 mb-1 block">
                                Or pick a system user:
                            </label>
                            <select
                                class="select select-bordered select-sm w-full"
                                value=""
                                on:change={(e) => {
                                    addFromUser(setting.key, (e.target as HTMLSelectElement).value);
                                    (e.target as HTMLSelectElement).value = '';
                                }}>
                                <option value="" disabled>— select user —</option>
                                {#each usersWithEmail as u}
                                    <option value={u.email}>{u.userName} ({u.email})</option>
                                {/each}
                            </select>
                        </div>
                    {/if}

                    <!-- Footer -->
                    <div class="flex items-center justify-between pt-2 border-t border-base-200">
                        {#if errors[setting.key]}
                            <span class="text-sm text-error">{errors[setting.key]}</span>
                        {:else if success[setting.key]}
                            <span class="text-sm text-success">Saved!</span>
                        {:else}
                            <span></span>
                        {/if}
                        <button
                            class="btn btn-sm btn-primary"
                            disabled={saving[setting.key]}
                            on:click={() => save(setting.key)}>
                            {#if saving[setting.key]}
                                <Loader2 class="w-4 h-4 animate-spin" />
                            {/if}
                            Save
                        </button>
                    </div>
                </div>
            </div>
        {/each}

        {#if settings.length === 0}
            <div class="text-center text-base-content/40 py-12">
                No notification settings found.
            </div>
        {/if}
    </div>
{/if}
