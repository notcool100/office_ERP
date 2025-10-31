<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, ShoppingCart, Receipt } from 'lucide-svelte';
    import PageSection from '../../../../components/PageSection.svelte';

    pageTitle.set({
        title: 'Receipt Voucher',
        desc: 'Placeholder page',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Purchase', icon: ShoppingCart },
        { label: 'Receipt Voucher', icon: Receipt }
    ]);
</script>

<PageSection title="Receipt Voucher">
    <p>This is the Receipt Voucher page.</p>
</PageSection>
