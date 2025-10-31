<script lang="ts">
    export let title: string | undefined = undefined;
</script>

<div class="border border-base-300 bg-base-200 p-6 shadow-inner">
    {#if title}
        <div
            class="divider divider-start before:bg-base-300 after:bg-base-300 text-sm m-0 mb-4">
            {title}
        </div>
    {/if}
    <slot />
</div>
