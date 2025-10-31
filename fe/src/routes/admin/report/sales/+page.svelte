<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, ChartPie, ChartBarStacked } from 'lucide-svelte';
    import PageSection from '../../../../components/PageSection.svelte';

    pageTitle.set({
        title: 'Sales',
        desc: 'Placeholder page',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Report', icon: ChartPie },
        { label: 'Sales', icon: ChartBarStacked }
    ]);
</script>

<PageSection title="Sales">
    <p>This is the Sales page.</p>
</PageSection>
