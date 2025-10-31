<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, Settings, User } from 'lucide-svelte';
    import PageSection from '../../../../components/PageSection.svelte';

    pageTitle.set({
        title: 'User',
        desc: 'Placeholder page',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Settings', icon: Settings },
        { label: 'User', icon: User }
    ]);
</script>

<PageSection title="User">
    <p>This is the User page.</p>
</PageSection>
