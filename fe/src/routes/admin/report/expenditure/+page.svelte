<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, ChartPie, ChartColumnDecreasing } from 'lucide-svelte';
    import PageSection from '../../../../components/PageSection.svelte';

    pageTitle.set({
        title: 'Expenditure',
        desc: 'Placeholder page',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Report', icon: ChartPie },
        { label: 'Expenditure', icon: ChartColumnDecreasing }
    ]);
</script>

<PageSection title="Expenditure">
    <p>This is the Expenditure page.</p>
</PageSection>
