<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, ChartColumn, FileText } from 'lucide-svelte';
    import PageSection from '../../../../components/PageSection.svelte';

    pageTitle.set({
        title: 'Payment Voucher',
        desc: 'Placeholder page',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Sales', icon: ChartColumn },
        { label: 'Payment Voucher', icon: FileText }
    ]);
</script>

<PageSection title="Payment Voucher">
    <p>This is the Payment Voucher page.</p>
</PageSection>
