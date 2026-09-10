<script lang="ts">
    import { onDestroy } from 'svelte';
    import {
        messagingService,
        type ChannelMediaItem,
    } from '$lib/services/messaging';
    import { formatFileSize } from '$lib/services/documents';
    import { X, Image, FileText, Download } from 'lucide-svelte';

    let {
        channelId,
        open = $bindable(false),
    }: { channelId: string; open: boolean } = $props();

    let items: ChannelMediaItem[] = $state([]);
    let loading = $state(false);
    let activeTab: 'media' | 'files' = $state('media');
    let imageUrls: Record<string, string> = $state({});

    const images = $derived(items.filter((i) => i.is_image));
    const files = $derived(items.filter((i) => !i.is_image));

    $effect(() => {
        if (open) {
            load();
        }
    });

    async function load() {
        loading = true;
        try {
            items = await messagingService.listChannelMedia(channelId);
            for (const item of items) {
                if (item.is_image && !imageUrls[item.id]) {
                    messagingService
                        .fetchAttachmentBlob(channelId, item.id)
                        .then((url) => (imageUrls[item.id] = url))
                        .catch((error) =>
                            console.error('Failed to load thumbnail:', error),
                        );
                }
            }
        } catch (error) {
            console.error('Failed to load shared media:', error);
        } finally {
            loading = false;
        }
    }

    async function downloadFile(item: ChannelMediaItem) {
        try {
            const url =
                imageUrls[item.id] ??
                (await messagingService.fetchAttachmentBlob(channelId, item.id));
            const link = document.createElement('a');
            link.href = url;
            link.download = item.file_name;
            link.click();
        } catch (error) {
            console.error('Failed to download attachment:', error);
        }
    }

    onDestroy(() => {
        for (const url of Object.values(imageUrls)) {
            URL.revokeObjectURL(url);
        }
    });
</script>

{#if open}
    <div class="fixed inset-0 z-[150]">
        <!-- Backdrop -->
        <button
            type="button"
            class="absolute inset-0 bg-black/30 cursor-default"
            aria-label="Close shared media panel"
            onclick={() => (open = false)}></button>

        <!-- Panel -->
        <div
            class="absolute right-0 top-0 h-full w-full max-w-sm bg-base-100 border-l border-base-300 shadow-2xl flex flex-col">
            <div
                class="h-16 shrink-0 border-b border-base-300 flex items-center justify-between px-4">
                <h3 class="font-bold text-lg">Shared Media</h3>
                <button
                    type="button"
                    class="btn btn-ghost btn-sm btn-square"
                    onclick={() => (open = false)}
                    aria-label="Close">
                    <X class="w-5 h-5" />
                </button>
            </div>

            <div class="tabs tabs-boxed m-3 bg-base-200">
                <button
                    type="button"
                    class="tab flex-1 gap-1.5 {activeTab === 'media' ? 'tab-active' : ''}"
                    onclick={() => (activeTab = 'media')}>
                    <Image class="w-4 h-4" />
                    Media ({images.length})
                </button>
                <button
                    type="button"
                    class="tab flex-1 gap-1.5 {activeTab === 'files' ? 'tab-active' : ''}"
                    onclick={() => (activeTab = 'files')}>
                    <FileText class="w-4 h-4" />
                    Files ({files.length})
                </button>
            </div>

            <div class="flex-1 overflow-y-auto px-3 pb-4">
                {#if loading}
                    <div class="flex justify-center p-8">
                        <span class="loading loading-spinner loading-md"></span>
                    </div>
                {:else if activeTab === 'media'}
                    {#if images.length === 0}
                        <p class="text-sm text-base-content/50 italic text-center py-8">
                            No shared images or videos yet.
                        </p>
                    {:else}
                        <div class="grid grid-cols-3 gap-2">
                            {#each images as item (item.id)}
                                <button
                                    type="button"
                                    class="aspect-square rounded-lg overflow-hidden border border-base-300 bg-base-200 flex items-center justify-center"
                                    title={item.file_name}
                                    onclick={() => downloadFile(item)}>
                                    {#if imageUrls[item.id]}
                                        <img
                                            src={imageUrls[item.id]}
                                            alt={item.file_name}
                                            class="w-full h-full object-cover" />
                                    {:else}
                                        <span class="loading loading-spinner loading-xs"></span>
                                    {/if}
                                </button>
                            {/each}
                        </div>
                    {/if}
                {:else if files.length === 0}
                    <p class="text-sm text-base-content/50 italic text-center py-8">
                        No shared files yet.
                    </p>
                {:else}
                    <ul class="space-y-1.5">
                        {#each files as item (item.id)}
                            <li>
                                <button
                                    type="button"
                                    class="w-full flex items-center gap-3 p-2.5 rounded-lg hover:bg-base-200 transition-colors text-left"
                                    onclick={() => downloadFile(item)}>
                                    <FileText class="w-5 h-5 text-base-content/60 shrink-0" />
                                    <span class="min-w-0 flex-1">
                                        <span class="block text-sm font-medium truncate"
                                            >{item.file_name}</span>
                                        <span class="block text-xs text-base-content/50">
                                            {formatFileSize(item.file_size)}
                                            {#if item.sender_name}
                                                &middot; {item.sender_name}
                                            {/if}
                                        </span>
                                    </span>
                                    <Download class="w-4 h-4 text-base-content/40 shrink-0" />
                                </button>
                            </li>
                        {/each}
                    </ul>
                {/if}
            </div>
        </div>
    </div>
{/if}
