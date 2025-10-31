<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, ChartPie, ChartColumnIncreasing } from 'lucide-svelte';
    import PageSection from '../../../../components/PageSection.svelte';

    pageTitle.set({
        title: 'Income',
        desc: 'Placeholder page',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Report', icon: ChartPie },
        { label: 'Income', icon: ChartColumnIncreasing }
    ]);
</script>

<PageSection title="Income">
    <p>This is the Income page.</p>
</PageSection>
