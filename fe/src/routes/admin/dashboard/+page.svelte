<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { userStore } from '$lib/stores/user';
    import { Home, LayoutDashboard, Users, CalendarCheck, ClipboardList, TrendingUp } from 'lucide-svelte';
    import { onMount } from 'svelte';
    import { api } from '$lib/services/api';

    pageTitle.set({ title: 'Dashboard', desc: 'Workspace overview' });
    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Dashboard', icon: LayoutDashboard },
    ]);

    let employeeCount = 0;
    let pendingLeave = 0;
    let openProjects = 0;

    onMount(async () => {
        try {
            const [empRes, leaveRes, projRes] = await Promise.allSettled([
                api.get('/employees'),
                api.get('/leave/requests?status=pending'),
                api.get('/projects'),
            ]);
            if (empRes.status === 'fulfilled' && empRes.value.ok) {
                const data = await empRes.value.json();
                employeeCount = data?.total ?? (Array.isArray(data?.employees) ? data.employees.length : (Array.isArray(data) ? data.length : 0));
            }
            if (leaveRes.status === 'fulfilled' && leaveRes.value.ok) {
                const data = await leaveRes.value.json();
                pendingLeave = data?.total ?? (Array.isArray(data?.requests) ? data.requests.length : 0);
            }
            if (projRes.status === 'fulfilled' && projRes.value.ok) {
                const data = await projRes.value.json();
                openProjects = Array.isArray(data) ? data.filter((p: any) => p.status === 'active').length : 0;
            }
        } catch {
            // non-critical
        }
    });

    $: greeting = (() => {
        const h = new Date().getHours();
        if (h < 12) return 'Good morning';
        if (h < 17) return 'Good afternoon';
        return 'Good evening';
    })();
</script>

<div class="p-6 space-y-6">

    <!-- Welcome -->
    <div>
        <h1 class="text-2xl font-bold">{greeting}, {$userStore.user?.userName ?? 'there'}</h1>
        <p class="text-base-content/50 text-sm mt-1">Here's what's happening in your workspace today.</p>
    </div>

    <!-- KPI Cards -->
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
        <div class="stat bg-base-100 rounded-box border border-base-300 p-5">
            <div class="stat-figure text-primary"><Users size={28} /></div>
            <div class="stat-title">Employees</div>
            <div class="stat-value">{employeeCount}</div>
            <div class="stat-desc">Total active staff</div>
        </div>
        <div class="stat bg-base-100 rounded-box border border-base-300 p-5">
            <div class="stat-figure text-warning"><CalendarCheck size={28} /></div>
            <div class="stat-title">Pending Leave</div>
            <div class="stat-value">{pendingLeave}</div>
            <div class="stat-desc">Awaiting approval</div>
        </div>
        <div class="stat bg-base-100 rounded-box border border-base-300 p-5">
            <div class="stat-figure text-success"><ClipboardList size={28} /></div>
            <div class="stat-title">Active Projects</div>
            <div class="stat-value">{openProjects}</div>
            <div class="stat-desc">Currently in progress</div>
        </div>
    </div>

    <!-- Quick links -->
    <div>
        <h2 class="text-sm font-semibold text-base-content/50 uppercase tracking-wider mb-3">Quick Access</h2>
        <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-3">
            {#each [
                { label: 'Leave Requests', href: '/admin/hr/leave', icon: CalendarCheck, color: 'text-warning' },
                { label: 'Employees', href: '/admin/hr/employee', icon: Users, color: 'text-primary' },
                { label: 'Projects', href: '/admin/projects', icon: ClipboardList, color: 'text-success' },
                { label: 'Daily Log', href: '/admin/daily-log', icon: TrendingUp, color: 'text-info' },
            ] as link}
                <a href={link.href}
                    class="flex items-center gap-3 p-4 bg-base-100 rounded-box border border-base-300 hover:border-primary hover:shadow-sm transition-all group">
                    <svelte:component this={link.icon} size={20} class="{link.color} shrink-0" />
                    <span class="text-sm font-medium group-hover:text-primary transition-colors">{link.label}</span>
                </a>
            {/each}
        </div>
    </div>
</div>
