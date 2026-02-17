<script lang="ts">
    import { onMount } from 'svelte';
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import {
        Home,
        Settings,
        ShieldCheck,
        Building2,
        Briefcase,
        Search,
        Shield,
    } from 'lucide-svelte';
    import PageSection from '../../../../components/PageSection.svelte';
    import PermissionModal from '../../../../components/PermissionModal.svelte';
    import {
        departmentService,
        type Department,
    } from '$lib/services/department';
    import { positionService, type Position } from '$lib/services/position';
    import {
        navigationStore,
        canCreate,
        canUpdate,
    } from '$lib/stores/navigation';

    pageTitle.set({
        title: 'Permissions',
        desc: 'Assign navigation access by department or position',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Settings', icon: Settings },
        { label: 'Permissions', icon: ShieldCheck },
    ]);

    let departments: Department[] = [];
    let positions: Position[] = [];
    let loading = true;
    let deptSearch = '';
    let posSearch = '';

    let showPermissionModal = false;
    let permissionDepartment: Department | null = null;
    let permissionPosition: Position | null = null;
    const navPath = '/admin/settings/permissions';

    async function loadData() {
        loading = true;
        try {
            const [deptResponse, posResponse] = await Promise.all([
                departmentService.getAll(),
                positionService.getAll(),
            ]);
            departments = deptResponse;
            positions = posResponse;
        } catch (error) {
            console.error('Failed to load permissions data:', error);
        } finally {
            loading = false;
        }
    }

    function openDepartmentPermissions(dept: Department) {
        permissionDepartment = dept;
        permissionPosition = null;
        showPermissionModal = true;
    }

    function openPositionPermissions(pos: Position) {
        permissionPosition = pos;
        permissionDepartment = null;
        showPermissionModal = true;
    }

    function closePermissionModal() {
        showPermissionModal = false;
        permissionDepartment = null;
        permissionPosition = null;
    }

    onMount(() => {
        loadData();
    });

    $: filteredDepartments = departments.filter((dept) => {
        const term = deptSearch.trim().toLowerCase();
        if (!term) return true;
        return (
            dept.name.toLowerCase().includes(term) ||
            (dept.description || '').toLowerCase().includes(term)
        );
    });

    $: filteredPositions = positions.filter((pos) => {
        const term = posSearch.trim().toLowerCase();
        if (!term) return true;
        return (
            pos.name.toLowerCase().includes(term) ||
            (pos.description || '').toLowerCase().includes(term)
        );
    });

    $: canManagePermissions =
        canCreate(navPath, $navigationStore) ||
        canUpdate(navPath, $navigationStore);
</script>

<PageSection>
    {#if loading}
        <div class="flex justify-center py-10">
            <span class="loading loading-spinner loading-lg"></span>
        </div>
    {:else}
        <div class="grid grid-cols-1 xl:grid-cols-2 gap-6">
            <div class="space-y-3">
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-2 text-sm font-semibold">
                        <Building2 class="w-4 h-4" /> Departments
                    </div>
                </div>

                <div class="flex gap-2">
                    <input
                        type="text"
                        placeholder="Search departments..."
                        bind:value={deptSearch}
                        class="input input-bordered flex-1" />
                    <button class="btn btn-square" aria-label="Search departments">
                        <Search size={18} />
                    </button>
                </div>

                <div class="overflow-x-auto">
                    <table class="table table-zebra">
                        <thead>
                            <tr>
                                <th>Name</th>
                                <th>Status</th>
                                <th class="text-right">Actions</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#if filteredDepartments.length === 0}
                                <tr>
                                    <td colspan="3" class="text-center opacity-60">
                                        No departments found
                                    </td>
                                </tr>
                            {:else}
                                {#each filteredDepartments as dept}
                                    <tr>
                                        <td>
                                            <div class="font-medium">
                                                {dept.name}
                                            </div>
                                            <div class="text-xs opacity-60">
                                                {dept.description || 'No description'}
                                            </div>
                                        </td>
                                        <td>
                                            <span
                                                class="badge badge-sm {dept.is_active
                                                    ? 'badge-success'
                                                    : 'badge-ghost'}">
                                                {dept.is_active ? 'Active' : 'Inactive'}
                                            </span>
                                        </td>
                                        <td class="text-right">
                                            <button
                                                class="btn btn-xs btn-ghost"
                                                disabled={!canManagePermissions}
                                                title={!canManagePermissions
                                                    ? 'No permission to manage'
                                                    : 'Manage permissions'}
                                                on:click={() =>
                                                    openDepartmentPermissions(dept)}>
                                                <Shield class="w-4 h-4 mr-1" />
                                                Manage
                                            </button>
                                        </td>
                                    </tr>
                                {/each}
                            {/if}
                        </tbody>
                    </table>
                </div>
            </div>

            <div class="space-y-3">
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-2 text-sm font-semibold">
                        <Briefcase class="w-4 h-4" /> Positions
                    </div>
                </div>

                <div class="flex gap-2">
                    <input
                        type="text"
                        placeholder="Search positions..."
                        bind:value={posSearch}
                        class="input input-bordered flex-1" />
                    <button class="btn btn-square" aria-label="Search positions">
                        <Search size={18} />
                    </button>
                </div>

                <div class="overflow-x-auto">
                    <table class="table table-zebra">
                        <thead>
                            <tr>
                                <th>Name</th>
                                <th>Status</th>
                                <th class="text-right">Actions</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#if filteredPositions.length === 0}
                                <tr>
                                    <td colspan="3" class="text-center opacity-60">
                                        No positions found
                                    </td>
                                </tr>
                            {:else}
                                {#each filteredPositions as pos}
                                    <tr>
                                        <td>
                                            <div class="font-medium">{pos.name}</div>
                                            <div class="text-xs opacity-60">
                                                {pos.description || 'No description'}
                                            </div>
                                        </td>
                                        <td>
                                            <span
                                                class="badge badge-sm {pos.is_active
                                                    ? 'badge-success'
                                                    : 'badge-ghost'}">
                                                {pos.is_active ? 'Active' : 'Inactive'}
                                            </span>
                                        </td>
                                        <td class="text-right">
                                            <button
                                                class="btn btn-xs btn-ghost"
                                                disabled={!canManagePermissions}
                                                title={!canManagePermissions
                                                    ? 'No permission to manage'
                                                    : 'Manage permissions'}
                                                on:click={() =>
                                                    openPositionPermissions(pos)}>
                                                <Shield class="w-4 h-4 mr-1" />
                                                Manage
                                            </button>
                                        </td>
                                    </tr>
                                {/each}
                            {/if}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    {/if}
</PageSection>

{#if showPermissionModal && permissionDepartment}
    <PermissionModal
        title="Permissions for {permissionDepartment.name}"
        departmentId={permissionDepartment.id}
        on:close={closePermissionModal} />
{/if}

{#if showPermissionModal && permissionPosition}
    <PermissionModal
        title="Permissions for {permissionPosition.name}"
        positionId={permissionPosition.id}
        on:close={closePermissionModal} />
{/if}
