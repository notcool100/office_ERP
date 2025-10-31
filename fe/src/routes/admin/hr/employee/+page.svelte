<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, Users, UserCheck } from 'lucide-svelte';
    import PageSection from '../../../../components/PageSection.svelte';

    pageTitle.set({
        title: 'Employee',
        desc: 'Placeholder page',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'HR', icon: Users },
        { label: 'Employee', icon: UserCheck }
    ]);
</script>

<PageSection title="Employee">
    <p>This is the Employee page.</p>
</PageSection>
