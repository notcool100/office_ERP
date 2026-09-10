<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, FileText, Building2 } from 'lucide-svelte';
    import PageSection from '../../../../components/PageSection.svelte';
    import DocumentBrowser from '$lib/components/documents/DocumentBrowser.svelte';

    pageTitle.set({
        title: 'Company Documents',
        desc: 'PAN, registration, and other company-wide documents',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Documents', icon: FileText },
        { label: 'Company Documents', icon: Building2 },
    ]);
</script>

<PageSection title="Company Documents">
    <DocumentBrowser
        category="company"
        navPath="/admin/documents/company"
        rootLabel="Company Documents" />
</PageSection>
