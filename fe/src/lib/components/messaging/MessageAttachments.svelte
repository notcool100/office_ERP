<script lang="ts">
    import { onDestroy, onMount } from 'svelte';
    import { messagingService, type Attachment } from '$lib/services/messaging';
    import { formatFileSize } from '$lib/services/documents';
    import { FileText, Download } from 'lucide-svelte';

    let { channelId, attachments }: { channelId: string; attachments: Attachment[] } = $props();

    let imageUrls: Record<string, string> = $state({});
    let lightboxUrl: string | null = $state(null);

    onMount(async () => {
        for (const a of attachments) {
            if (!a.is_image) continue;
            try {
                imageUrls[a.id] = await messagingService.fetchAttachmentBlob(channelId, a.id);
            } catch (error) {
                console.error('Failed to load attachment image:', error);
            }
        }
    });

    onDestroy(() => {
        for (const url of Object.values(imageUrls)) {
            URL.revokeObjectURL(url);
        }
    });

    async function downloadFile(a: Attachment) {
        try {
            const url = await messagingService.fetchAttachmentBlob(channelId, a.id);
            const link = document.createElement('a');
            link.href = url;
            link.download = a.file_name;
            link.click();
            URL.revokeObjectURL(url);
        } catch (error) {
            console.error('Failed to download attachment:', error);
        }
    }
</script>

{#if attachments.length > 0}
    <div class="mt-2 flex flex-wrap gap-2">
        {#each attachments as a (a.id)}
            {#if a.is_image}
                {#if imageUrls[a.id]}
                    <button
                        type="button"
                        class="block rounded-lg overflow-hidden border border-base-300 hover:opacity-90 transition-opacity"
                        onclick={() => (lightboxUrl = imageUrls[a.id])}>
                        <img
                            src={imageUrls[a.id]}
                            alt={a.file_name}
                            class="max-w-[260px] max-h-[220px] object-cover" />
                    </button>
                {:else}
                    <div
                        class="w-32 h-24 rounded-lg border border-base-300 bg-base-200 flex items-center justify-center">
                        <span class="loading loading-spinner loading-sm"></span>
                    </div>
                {/if}
            {:else}
                <button
                    type="button"
                    class="flex items-center gap-2 px-3 py-2 rounded-lg border border-base-300 bg-base-100 hover:bg-base-200 transition-colors text-left max-w-xs"
                    onclick={() => downloadFile(a)}>
                    <FileText class="w-5 h-5 text-base-content/60 shrink-0" />
                    <span class="min-w-0">
                        <span class="block text-sm font-medium truncate">{a.file_name}</span>
                        <span class="block text-xs text-base-content/50"
                            >{formatFileSize(a.file_size)}</span>
                    </span>
                    <Download class="w-4 h-4 text-base-content/40 shrink-0" />
                </button>
            {/if}
        {/each}
    </div>
{/if}

<!-- Image lightbox -->
{#if lightboxUrl}
    <div
        class="fixed inset-0 z-[200] bg-black/80 flex items-center justify-center p-8"
        role="button"
        tabindex="0"
        onclick={() => (lightboxUrl = null)}
        onkeydown={(e) => e.key === 'Escape' && (lightboxUrl = null)}>
        <img src={lightboxUrl} alt="Attachment preview" class="max-w-full max-h-full rounded-lg shadow-2xl" />
    </div>
{/if}
