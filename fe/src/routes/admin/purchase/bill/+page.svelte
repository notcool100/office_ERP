<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, ShoppingCart, FileText } from 'lucide-svelte';
    import PageSection from '../../../../components/PageSection.svelte';

    pageTitle.set({
        title: 'Bill',
        desc: 'Placeholder page',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Purchase', icon: ShoppingCart },
        { label: 'Bill', icon: FileText }
    ]);
</script>

<PageSection title="Bill">
    <p>This is the Bill page.</p>
</PageSection>
