<script lang="ts">
    import { onMount } from 'svelte';
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, Settings, KeyRound, Copy, Trash2, Loader2 } from 'lucide-svelte';
    import {
        apiTokenService,
        type ApiToken,
        type CreatedApiToken,
    } from '$lib/services/api-tokens';

    pageTitle.set({
        title: 'API Tokens',
        desc: 'Let scripts and AI agents manage projects and tasks on your behalf.',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Settings', icon: Settings },
        { label: 'API Tokens', icon: KeyRound },
    ]);

    let tokens: ApiToken[] = [];
    let loading = true;
    let creating = false;
    let error = '';
    let name = '';
    let expiresInDays = '90';
    let created: CreatedApiToken | null = null;
    let copied = false;

    async function load() {
        loading = true;
        try {
            tokens = await apiTokenService.list();
        } catch (e: any) {
            error = e.message ?? 'Failed to load tokens';
        } finally {
            loading = false;
        }
    }

    async function create() {
        if (!name.trim()) return;
        creating = true;
        error = '';
        try {
            created = await apiTokenService.create(
                name.trim(),
                expiresInDays ? Number(expiresInDays) : undefined,
            );
            copied = false;
            name = '';
            await load();
        } catch (e: any) {
            error = e.message ?? 'Failed to create token';
        } finally {
            creating = false;
        }
    }

    async function revoke(token: ApiToken) {
        if (!confirm(`Revoke "${token.name}"? Anything using it will stop working.`)) return;
        try {
            await apiTokenService.revoke(token.id);
            if (created?.id === token.id) created = null;
            await load();
        } catch (e: any) {
            error = e.message ?? 'Failed to revoke token';
        }
    }

    async function copySecret() {
        if (!created) return;
        await navigator.clipboard.writeText(created.secret);
        copied = true;
    }

    const fmt = (iso: string | null) => (iso ? new Date(iso).toLocaleString() : '—');

    onMount(load);
</script>

<div class="space-y-6 max-w-3xl">
    <div class="card bg-base-100 border border-base-300 shadow-sm">
        <div class="card-body">
            <h3 class="font-semibold">Create a token</h3>
            <p class="text-sm text-base-content/60">
                A token acts as you, with your permissions, but only on the project API
                (<code>/projects</code>): projects, epics, stories, tasks, assignees, comments and
                attachments. Send it as <code>Authorization: Bearer &lt;token&gt;</code>.
            </p>
            <form class="flex flex-wrap gap-2 mt-2" on:submit|preventDefault={create}>
                <input
                    class="input input-bordered input-sm flex-1 min-w-48"
                    placeholder="Name, e.g. Claude"
                    maxlength="100"
                    bind:value={name} />
                <select class="select select-bordered select-sm" bind:value={expiresInDays}>
                    <option value="30">Expires in 30 days</option>
                    <option value="90">Expires in 90 days</option>
                    <option value="365">Expires in 1 year</option>
                    <option value="">Never expires</option>
                </select>
                <button class="btn btn-sm btn-primary" disabled={creating || !name.trim()}>
                    {#if creating}<Loader2 class="w-4 h-4 animate-spin" />{/if}
                    Generate
                </button>
            </form>
            {#if error}<p class="text-sm text-error mt-2">{error}</p>{/if}
        </div>
    </div>

    {#if created}
        <div class="alert alert-warning items-start">
            <div class="w-full">
                <p class="font-semibold">Copy your token now — it won't be shown again.</p>
                <div class="flex gap-2 mt-2">
                    <input class="input input-bordered input-sm flex-1 font-mono" readonly value={created.secret} />
                    <button class="btn btn-sm" on:click={copySecret}>
                        <Copy class="w-4 h-4" />
                        {copied ? 'Copied' : 'Copy'}
                    </button>
                </div>
            </div>
        </div>
    {/if}

    <div class="card bg-base-100 border border-base-300 shadow-sm">
        <div class="card-body">
            <h3 class="font-semibold">Your tokens</h3>
            {#if loading}
                <span class="loading loading-spinner"></span>
            {:else if tokens.length === 0}
                <p class="text-sm text-base-content/40 italic">No active tokens.</p>
            {:else}
                <div class="overflow-x-auto">
                    <table class="table table-sm">
                        <thead>
                            <tr>
                                <th>Name</th>
                                <th>Token</th>
                                <th>Expires</th>
                                <th>Last used</th>
                                <th></th>
                            </tr>
                        </thead>
                        <tbody>
                            {#each tokens as token (token.id)}
                                <tr>
                                    <td>{token.name}</td>
                                    <td class="font-mono">{token.tokenPrefix}…</td>
                                    <td>{fmt(token.expiresAt)}</td>
                                    <td>{fmt(token.lastUsedAt)}</td>
                                    <td class="text-right">
                                        <button
                                            class="btn btn-ghost btn-xs text-error"
                                            aria-label="Revoke {token.name}"
                                            on:click={() => revoke(token)}>
                                            <Trash2 class="w-4 h-4" />
                                        </button>
                                    </td>
                                </tr>
                            {/each}
                        </tbody>
                    </table>
                </div>
            {/if}
        </div>
    </div>
</div>
