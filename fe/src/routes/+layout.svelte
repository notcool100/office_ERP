<script lang="ts">
    import { onMount } from 'svelte';
    import { companySettingsService } from '$lib/services/company-settings';
    import { buildApiUrl } from '$lib/services/api';

    const { children } = $props();

    onMount(async () => {
        try {
            const settings = await companySettingsService.getSettings();
            if (settings.favicon_url) {
                const link = document.querySelector<HTMLLinkElement>('link[rel="icon"]');
                if (link) {
                    link.href = buildApiUrl(settings.favicon_url);
                }
            }
        } catch {
            // keep default bundled favicon
        }
    });
</script>

{@render children()}
