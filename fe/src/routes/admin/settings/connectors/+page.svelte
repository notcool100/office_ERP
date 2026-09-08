<script lang="ts">
    import { onMount } from 'svelte';
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, Settings, Plug, Github, Loader2 } from 'lucide-svelte';
    import {
        githubConnectorService,
        type GithubConnection,
    } from '$lib/services/github-connector';

    pageTitle.set({ title: 'Connectors', desc: 'Connect third-party services to this instance.' });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Settings', icon: Settings },
        { label: 'Connectors', icon: Plug },
    ]);

    let connection: GithubConnection | null = null;
    let loading = true;
    let token = '';
    let connecting = false;
    let disconnecting = false;
    let confirmingDisconnect = false;
    let error = '';
    let success = '';

    async function load() {
        loading = true;
        try {
            connection = await githubConnectorService.getConnection();
        } catch (e: any) {
            console.error(e);
        } finally {
            loading = false;
        }
    }

    function formatDate(value: string | null) {
        if (!value) return '-';
        return new Date(value).toLocaleString();
    }

    async function connect() {
        if (!token.trim()) return;
        connecting = true;
        error = '';
        success = '';
        try {
            connection = await githubConnectorService.connect(token.trim());
            token = '';
            success = 'GitHub account connected.';
            setTimeout(() => { success = ''; }, 3000);
        } catch (e: any) {
            error = e.message ?? 'Failed to connect GitHub account';
        } finally {
            connecting = false;
        }
    }

    async function disconnect() {
        disconnecting = true;
        error = '';
        try {
            await githubConnectorService.disconnect();
            connection = { connected: false, github_username: null, connected_at: null };
            confirmingDisconnect = false;
        } catch (e: any) {
            error = e.message ?? 'Failed to disconnect GitHub account';
        } finally {
            disconnecting = false;
        }
    }

    onMount(load);
</script>

{#if loading}
    <div class="flex justify-center p-12">
        <span class="loading loading-spinner loading-lg"></span>
    </div>
{:else}
    <div class="space-y-6 max-w-2xl">
        <div class="card bg-base-100 border border-base-300 shadow-sm">
            <div class="card-body">
                <div class="flex items-start gap-3 mb-4">
                    <div class="p-2 rounded-lg bg-primary/10 text-primary mt-0.5">
                        <Github class="w-5 h-5" />
                    </div>
                    <div>
                        <h3 class="font-semibold text-base">GitHub</h3>
                        <p class="text-sm text-base-content/60 mt-0.5">
                            Connect a GitHub account so projects can be linked to repositories
                            and synced with issues.
                        </p>
                    </div>
                </div>

                {#if connection?.connected}
                    <div class="mb-4 space-y-1">
                        <div class="text-sm">
                            Connected as
                            <span class="font-medium">{connection.github_username}</span>
                        </div>
                        <div class="text-xs text-base-content/50">
                            Connected on {formatDate(connection.connected_at)}
                        </div>
                    </div>

                    <div class="flex items-center justify-between pt-2 border-t border-base-200">
                        {#if error}
                            <span class="text-sm text-error">{error}</span>
                        {:else}
                            <span></span>
                        {/if}
                        {#if confirmingDisconnect}
                            <div class="flex items-center gap-2">
                                <span class="text-sm text-base-content/60">Disconnect GitHub?</span>
                                <button
                                    class="btn btn-sm btn-ghost"
                                    disabled={disconnecting}
                                    on:click={() => (confirmingDisconnect = false)}>
                                    Cancel
                                </button>
                                <button
                                    class="btn btn-sm btn-error"
                                    disabled={disconnecting}
                                    on:click={disconnect}>
                                    {#if disconnecting}
                                        <Loader2 class="w-4 h-4 animate-spin" />
                                    {/if}
                                    Confirm
                                </button>
                            </div>
                        {:else}
                            <button
                                class="btn btn-sm btn-error btn-outline"
                                on:click={() => (confirmingDisconnect = true)}>
                                Disconnect
                            </button>
                        {/if}
                    </div>
                {:else}
                    <div class="form-control mb-3">
                        <label class="label" for="github-token">
                            <span class="label-text">Personal Access Token</span>
                        </label>
                        <input
                            id="github-token"
                            type="password"
                            class="input input-bordered"
                            placeholder="ghp_xxxxxxxxxxxxxxxxxxxx"
                            bind:value={token} />
                    </div>

                    <div class="flex items-center justify-between pt-2 border-t border-base-200">
                        {#if error}
                            <span class="text-sm text-error">{error}</span>
                        {:else if success}
                            <span class="text-sm text-success">{success}</span>
                        {:else}
                            <span></span>
                        {/if}
                        <button
                            class="btn btn-sm btn-primary"
                            disabled={connecting || !token.trim()}
                            on:click={connect}>
                            {#if connecting}
                                <Loader2 class="w-4 h-4 animate-spin" />
                            {/if}
                            Connect
                        </button>
                    </div>
                {/if}
            </div>
        </div>
    </div>
{/if}
