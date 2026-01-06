<script lang="ts">
    import { createEventDispatcher, onMount } from 'svelte';
    import {
        navigationService,
        type NavigationItem,
    } from '$lib/services/navigation';
    import {
        permissionService,
        type Permission,
    } from '$lib/services/permissions';

    export let title: string;
    export let departmentId: string | undefined = undefined;
    export let positionId: string | undefined = undefined;

    const dispatch = createEventDispatcher();
    let navigationItems: NavigationItem[] = [];
    let existingPermissions: Permission[] = [];
    let loading = true;
    let saving = false;

    // Local state to track permissions being edited
    // Key: navigation_item_id, Value: Permission object (or partial)
    let permissionsMap: Record<
        string,
        {
            can_create: boolean;
            can_read: boolean;
            can_update: boolean;
            can_delete: boolean;
        }
    > = {};

    async function loadData() {
        loading = true;
        try {
            // 1. Fetch all navigation items
            navigationItems = await navigationService.getAll();

            // 2. Fetch existing permissions for this entity
            existingPermissions = await permissionService.getAll({
                department_id: departmentId,
                position_id: positionId,
            });

            // 3. Populate local state
            permissionsMap = {};
            navigationItems.forEach((nav) => {
                const existing = existingPermissions.find(
                    (p) => p.navigation_item_id === nav.id,
                );
                permissionsMap[nav.id] = {
                    can_create: existing?.can_create ?? false,
                    can_read: existing?.can_read ?? false,
                    can_update: existing?.can_update ?? false,
                    can_delete: existing?.can_delete ?? false,
                };
            });
        } catch (error) {
            console.error('Failed to load permissions data:', error);
        } finally {
            loading = false;
        }
    }

    async function handleSave() {
        saving = true;
        try {
            // Loop through all items and assign permissions
            // Note: In a real app, you might only send diffs.
            // But the API might expect individual calls.
            // Ideally backend supports bulk update, but we'll use individual assign calls for now based on available API.
            // If API handler allows "upsert" logic, we just call assignPermission.

            const promises = navigationItems.map((nav) => {
                const p = permissionsMap[nav.id];
                // Only send if at least one permission is true OR if it existed before (to revoke).
                // But simplifying: just send state for all is safest if backend handles it.
                // Or better: filter those that changed.
                // For this implementation, we will iterate and save all to ensure consistency.

                return permissionService.assign({
                    department_id: departmentId,
                    position_id: positionId,
                    navigation_item_id: nav.id,
                    can_create: p.can_create,
                    can_read: p.can_read,
                    can_update: p.can_update,
                    can_delete: p.can_delete,
                });
            });

            await Promise.all(promises);
            dispatch('close');
        } catch (error) {
            console.error('Failed to save permissions:', error);
            // Optionally show toast/alert
        } finally {
            saving = false;
        }
    }

    onMount(() => {
        loadData();
    });

    // Helper to group items by parent (simple 1-level nesting for display if needed)
    // For now, listing flat or simple sort is okay.
    // Let's sort by display_order.
    $: sortedItems = [...navigationItems].sort(
        (a, b) => a.display_order - b.display_order,
    );
</script>

<div class="modal modal-open">
    <div class="modal-box w-11/12 max-w-5xl">
        <h3 class="font-bold text-lg mb-4">{title}</h3>

        {#if loading}
            <div class="flex justify-center p-8">
                <span class="loading loading-spinner loading-lg"></span>
            </div>
        {:else}
            <div class="overflow-x-auto max-h-[60vh]">
                <table class="table table-zebra w-full shadow-md">
                    <thead class="sticky top-0 bg-base-100 z-10">
                        <tr>
                            <th>Navigation Item</th>
                            <th class="text-center">Create</th>
                            <th class="text-center">Read</th>
                            <th class="text-center">Update</th>
                            <th class="text-center">Delete</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each sortedItems as item}
                            <tr class="hover">
                                <td class="font-medium">
                                    <div class="flex items-center gap-2">
                                        {#if item.icon}
                                            <span class="opacity-70"
                                                >{item.icon}</span>
                                            <!-- You might want to render icon component here -->
                                        {/if}
                                        {item.name}
                                        <span class="text-xs opacity-50 ml-2"
                                            >({item.path})</span>
                                    </div>
                                </td>
                                <td class="text-center">
                                    <input
                                        type="checkbox"
                                        class="checkbox checkbox-sm checkbox-primary"
                                        bind:checked={
                                            permissionsMap[item.id].can_create
                                        } />
                                </td>
                                <td class="text-center">
                                    <input
                                        type="checkbox"
                                        class="checkbox checkbox-sm checkbox-primary"
                                        bind:checked={
                                            permissionsMap[item.id].can_read
                                        } />
                                </td>
                                <td class="text-center">
                                    <input
                                        type="checkbox"
                                        class="checkbox checkbox-sm checkbox-primary"
                                        bind:checked={
                                            permissionsMap[item.id].can_update
                                        } />
                                </td>
                                <td class="text-center">
                                    <input
                                        type="checkbox"
                                        class="checkbox checkbox-sm checkbox-primary"
                                        bind:checked={
                                            permissionsMap[item.id].can_delete
                                        } />
                                </td>
                            </tr>
                        {/each}
                    </tbody>
                </table>
            </div>
        {/if}

        <div class="modal-action">
            <button
                type="button"
                class="btn"
                on:click={() => dispatch('close')}
                disabled={saving}>Cancel</button>
            <button
                type="button"
                class="btn btn-primary"
                on:click={handleSave}
                disabled={saving || loading}>
                {#if saving}
                    <span class="loading loading-spinner loading-xs"></span> Saving...
                {:else}
                    Save Changes
                {/if}
            </button>
        </div>
    </div>
    <form
        method="dialog"
        class="modal-backdrop"
        on:click={() => dispatch('close')}>
        <button>close</button>
    </form>
</div>
