<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, ChartColumn, FileX } from 'lucide-svelte';
    import PageSection from '../../../../components/PageSection.svelte';

    pageTitle.set({
        title: 'Invoice',
        desc: 'Placeholder page',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Sales', icon: ChartColumn },
        { label: 'Invoice', icon: FileX }
    ]);
</script>

<PageSection title="Invoice">
    <p>This is the Invoice page.</p>
</PageSection>
