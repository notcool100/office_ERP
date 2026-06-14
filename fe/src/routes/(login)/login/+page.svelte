<script lang="ts">
    import { userStore } from '$lib/stores/user';
    import { goto } from '$app/navigation';
    import { onMount } from 'svelte';
    import { fade } from 'svelte/transition';
    import { Building2 } from 'lucide-svelte';

    let username = '';
    let password = '';
    let message = '';
    let loading = false;
    let usernameInput: HTMLInputElement | null = null;

    onMount(() => {
        usernameInput?.focus();
    });

    async function handleSubmit() {
        if (!username || !password) return;
        loading = true;
        message = '';
        try {
            await userStore.login(username, password);
            goto('/admin/dashboard');
        } catch {
            message = 'Invalid username or password. Please try again.';
        } finally {
            loading = false;
        }
    }
</script>

<svelte:head>
    <title>Sign In — Office ERP</title>
</svelte:head>

<div class="min-h-screen bg-base-200 flex items-center justify-center px-4">
    <div class="w-full max-w-sm">

        <!-- Logo -->
        <div class="flex flex-col items-center mb-8">
            <div class="w-12 h-12 rounded-xl bg-primary flex items-center justify-center mb-3">
                <Building2 size={24} class="text-primary-content" />
            </div>
            <h1 class="text-2xl font-bold text-base-content">Office ERP</h1>
            <p class="text-sm text-base-content/50 mt-1">Sign in to your workspace</p>
        </div>

        <!-- Card -->
        <div class="card bg-base-100 shadow-sm border border-base-300">
            <div class="card-body gap-4 p-6">
                <form class="space-y-4" on:submit|preventDefault={handleSubmit}>
                    <div class="form-control">
                        <label class="label pb-1" for="username">
                            <span class="label-text font-medium">Username</span>
                        </label>
                        <input
                            id="username"
                            type="text"
                            bind:this={usernameInput}
                            bind:value={username}
                            placeholder="Enter your username"
                            autocomplete="username"
                            class="input input-bordered w-full" />
                    </div>

                    <div class="form-control">
                        <label class="label pb-1" for="password">
                            <span class="label-text font-medium">Password</span>
                        </label>
                        <input
                            id="password"
                            type="password"
                            bind:value={password}
                            placeholder="Enter your password"
                            autocomplete="current-password"
                            class="input input-bordered w-full" />
                    </div>

                    {#if message}
                        <div class="alert alert-error py-2 text-sm" transition:fade>
                            {message}
                        </div>
                    {/if}

                    <button
                        class="btn btn-primary w-full"
                        type="submit"
                        disabled={loading || !username || !password}>
                        {#if loading}
                            <span class="loading loading-spinner loading-sm"></span>
                        {/if}
                        Sign In
                    </button>
                </form>
            </div>
        </div>

        <p class="text-center text-xs text-base-content/40 mt-6">
            Office ERP &copy; {new Date().getFullYear()}
        </p>
    </div>
</div>
