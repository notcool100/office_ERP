<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, ShoppingCart, ClipboardList } from 'lucide-svelte';
    import PageSection from '../../../../components/PageSection.svelte';

    pageTitle.set({
        title: 'Purchase Order',
        desc: 'Placeholder page',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Purchase', icon: ShoppingCart },
        { label: 'Purchase Order', icon: ClipboardList }
    ]);
</script>

<PageSection title="Purchase Order">
    <p>This is the Purchase Order page.</p>
</PageSection>
