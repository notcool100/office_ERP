<script lang="ts">
    import { notifications } from '../stores/notification';
    import { flip } from 'svelte/animate';
    import { fly } from 'svelte/transition';

    const typeClasses = {
        info: 'alert-info',
        success: 'alert-success',
        warning: 'alert-warning',
        error: 'alert-error',
    };
</script>

<div
    class="fixed top-4 right-4 z-[100] flex flex-col gap-2 w-80 pointer-events-none">
    {#each $notifications as notification (notification.id)}
        <div
            animate:flip={{ duration: 300 }}
            transition:fly={{ x: 100, duration: 300 }}
            class="alert shadow-lg pointer-events-auto {typeClasses[
                notification.type
            ]} flex items-start">
            <div class="flex-1">
                {#if notification.title}
                    <h3 class="font-bold text-sm">{notification.title}</h3>
                {/if}
                <div class="text-xs">{notification.message}</div>
            </div>
            <button
                class="btn btn-ghost btn-xs"
                onclick={() => notifications.remove(notification.id)}>
                ✕
            </button>
        </div>
    {/each}
</div>
