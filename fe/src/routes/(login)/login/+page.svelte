<script lang="ts">
    import { userStore } from '$lib/stores/user';
    import { goto } from '$app/navigation';
    import { onMount } from 'svelte';
    import { fade } from 'svelte/transition';

    let username = '';
    let password = '';
    let message = '';
    let usernameInput: HTMLInputElement | null = null;

    onMount(() => {
        usernameInput?.focus();
    });

    async function handleSubmit() {
        try {
            await userStore.login(username, password);
            goto('/admin/dashboard');
        } catch (e) {
            message = 'Invalid credentials';
        }
    }
</script>

<svelte:head>
    <title>Login - XenLedger</title>
    <meta name="description" content="Sign in to your XenLedger workspace" />
</svelte:head>

<div
    class="min-h-screen bg-base-200 text-base-content font-mono flex items-center justify-center px-4">
    <div
        class="w-full max-w-md border border-base-300 bg-base-100/80 shadow-xl rounded-lg p-6">
        <!-- Header -->
        <div class="mb-6 text-center">
            <h1 class="text-2xl font-semibold tracking-wide">
                XenLedger Login
            </h1>
            <p class="text-sm text-base-content/60 mt-1">
                Please enter your credentials to continue
            </p>
        </div>

        <!-- Login Form -->
        <form class="space-y-4" on:submit|preventDefault={handleSubmit}>
            <div>
                <label class="text-sm block mb-1" for="username"
                    >:username</label>
                <input
                    id="username"
                    type="text"
                    bind:this={usernameInput}
                    bind:value={username}
                    placeholder="admin@xenledger.dev"
                    autocomplete="username"
                    class="w-full input input-bordered bg-base-100 border-base-300 placeholder:text-base-content/40 text-sm" />
            </div>

            <div>
                <label class="text-sm block mb-1" for="password"
                    >:password</label>
                <input
                    id="password"
                    type="password"
                    bind:value={password}
                    placeholder="••••••••"
                    autocomplete="current-password"
                    class="w-full input input-bordered bg-base-100 border-base-300 placeholder:text-base-content/40 text-sm" />
            </div>

            <!-- Submit -->
            <button
                class="btn btn-primary w-full text-sm tracking-wide"
                type="submit">
                Sign In
            </button>

            {#if message}
                <p class="text-error text-sm" role="status" transition:fade>
                    {message}
                </p>
            {/if}
        </form>

        <!-- Optional Footer -->
        <div class="text-right text-xs mt-2">
            <a href="/forgot-password" class="link link-hover text-primary">
                Forgot password?
            </a>
        </div>
    </div>
</div>
