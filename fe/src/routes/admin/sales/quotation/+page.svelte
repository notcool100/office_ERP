<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, ChartColumn, File } from 'lucide-svelte';
    import PageSection from '../../../../components/PageSection.svelte';

    pageTitle.set({
        title: 'Quotation',
        desc: 'Placeholder page',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Sales', icon: ChartColumn },
        { label: 'Quotation', icon: File }
    ]);
</script>

<PageSection title="Quotation">
    <p>This is the Quotation page.</p>
</PageSection>
