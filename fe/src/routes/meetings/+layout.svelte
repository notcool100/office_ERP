<script lang="ts">
    import '../../app.css';
    import { onMount } from 'svelte';
    import Sidebar from '../../components/layout/Sidebar.svelte';
    import StatusBar from '../../components/layout/StatusBar.svelte';
    import { userStore } from '$lib/stores/user';
    import { goto } from '$app/navigation';

    const { children } = $props();

    onMount(async () => {
        await userStore.init();
        if (!$userStore.isAuthenticated) {
            goto('/login');
        }
    });
</script>

<div class="h-screen bg-base-300 text-base-content flex flex-col font-mono text-sm">
    <div class="flex flex-1 overflow-hidden">
        <Sidebar />

        <!-- Main content -->
        <div class="flex-1 bg-base-200 flex flex-col overflow-hidden">
            {@render children()}
        </div>
    </div>
    <StatusBar />
</div>
