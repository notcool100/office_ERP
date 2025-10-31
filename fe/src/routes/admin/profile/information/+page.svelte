<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, CircleUser } from 'lucide-svelte';
    import PageSection from '../../../../components/PageSection.svelte';

    pageTitle.set({
        title: 'Profile Information',
        desc: 'Your personal and account details',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Profile', icon: CircleUser },
    ]);

    let profile = {
        name: 'Bibek Shah',
        email: 'bibek@xenith.com.np',
        phone: '9823066403',
        role: 'Admin',
    };
</script>

<PageSection title="Profile Details">
    <div class="space-y-4">
        <div>
            <span class="text-xs text-base-content/60">Full Name</span>
            <p class="text-base mt-1">{profile.name}</p>
        </div>
        <div>
            <span class="text-xs text-base-content/60">Email</span>
            <p class="text-base mt-1">{profile.email}</p>
        </div>
        <div>
            <span class="text-xs text-base-content/60">Phone</span>
            <p class="text-base mt-1">{profile.phone}</p>
        </div>
        <div>
            <span class="text-xs text-base-content/60">Role</span>
            <p class="text-base mt-1">{profile.role}</p>
        </div>
    </div>

    <div class="mt-6 text-right">
        <button class="btn btn-neutral btn-sm">Edit Profile</button>
    </div>
</PageSection>
