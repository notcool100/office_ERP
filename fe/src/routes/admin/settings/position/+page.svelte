<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import {
        Home,
        Settings,
        Briefcase,
        Plus,
        Edit,
        Trash2,
        Shield,
    } from 'lucide-svelte';
    import { onMount } from 'svelte';
    import PageSection from '../../../../components/PageSection.svelte';
    import PermissionModal from '../../../../components/PermissionModal.svelte';
    import { positionService, type Position } from '$lib/services/position';

    pageTitle.set({
        title: 'Positions',
        desc: 'Manage job positions',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Settings', icon: Settings },
        { label: 'Positions', icon: Briefcase },
    ]);

    let positions: Position[] = [];
    let loading = true;
    let showModal = false;
    let showPermissionModal = false;
    let editingPosition: Position | null = null;
    let permissionPosition: Position | null = null;
    let formData = {
        name: '',
        description: '',
    };

    async function loadPositions() {
        loading = true;
        try {
            positions = await positionService.getAll();
        } catch (error) {
            console.error('Failed to load positions:', error);
        } finally {
            loading = false;
        }
    }

    function openCreateModal() {
        editingPosition = null;
        formData = { name: '', description: '' };
        showModal = true;
    }

    function openEditModal(pos: Position) {
        editingPosition = pos;
        formData = {
            name: pos.name,
            description: pos.description || '',
        };
        showModal = true;
    }

    function openPermissionModal(pos: Position) {
        permissionPosition = pos;
        showPermissionModal = true;
    }

    async function handleSubmit() {
        try {
            if (editingPosition) {
                await positionService.update(editingPosition.id, formData);
            } else {
                await positionService.create(formData);
            }
            showModal = false;
            await loadPositions();
        } catch (error) {
            console.error('Failed to save position:', error);
        }
    }

    async function handleDelete(id: string) {
        if (confirm('Are you sure you want to delete this position?')) {
            try {
                await positionService.delete(id);
                await loadPositions();
            } catch (error) {
                console.error('Failed to delete position:', error);
            }
        }
    }

    async function toggleActive(pos: Position) {
        try {
            await positionService.update(pos.id, { is_active: !pos.is_active });
            await loadPositions();
        } catch (error) {
            console.error('Failed to toggle position status:', error);
        }
    }

    onMount(() => {
        loadPositions();
    });
</script>

<PageSection>
    <div class="text-right mb-2">
        <button class="btn btn-primary btn-sm" on:click={openCreateModal}>
            <Plus class="w-4 h-4 mr-1" /> Add Position
        </button>
    </div>

    {#if loading}
        <div class="flex justify-center p-8">
            <span class="loading loading-spinner loading-lg"></span>
        </div>
    {:else}
        <div class="overflow-x-auto">
            <table class="table">
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Description</th>
                        <th>Status</th>
                        <th class="text-right">Actions</th>
                    </tr>
                </thead>
                <tbody>
                    {#each positions as pos}
                        <tr>
                            <td>{pos.name}</td>
                            <td>{pos.description || '-'}</td>
                            <td>
                                <div class="form-control">
                                    <label
                                        class="label cursor-pointer gap-2 justify-start">
                                        <input
                                            type="checkbox"
                                            class="toggle toggle-success toggle-sm"
                                            checked={pos.is_active}
                                            on:change={() =>
                                                toggleActive(pos)} />
                                        <span class="label-text"
                                            >{pos.is_active
                                                ? 'Active'
                                                : 'Inactive'}</span>
                                    </label>
                                </div>
                            </td>
                            <td class="text-right">
                                <div class="join">
                                    <button
                                        class="btn btn-sm btn-ghost join-item"
                                        title="Permissions"
                                        on:click={() =>
                                            openPermissionModal(pos)}>
                                        <Shield class="w-4 h-4" />
                                    </button>
                                    <button
                                        class="btn btn-sm btn-ghost join-item"
                                        on:click={() => openEditModal(pos)}>
                                        <Edit class="w-4 h-4" />
                                    </button>
                                    <button
                                        class="btn btn-sm btn-ghost join-item text-error"
                                        on:click={() => handleDelete(pos.id)}>
                                        <Trash2 class="w-4 h-4" />
                                    </button>
                                </div>
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    {/if}
</PageSection>

{#if showModal}
    <dialog class="modal modal-open">
        <div class="modal-box">
            <h3 class="font-bold text-lg mb-4">
                {editingPosition ? 'Edit Position' : 'Create Position'}
            </h3>
            <form on:submit|preventDefault={handleSubmit}>
                <div class="form-control">
                    <label class="label" for="name">
                        <span class="label-text">Name *</span>
                    </label>
                    <input
                        id="name"
                        type="text"
                        placeholder="Position name"
                        class="input input-bordered"
                        bind:value={formData.name}
                        required />
                </div>
                <div class="form-control mt-4">
                    <label class="label" for="description">
                        <span class="label-text">Description</span>
                    </label>
                    <textarea
                        id="description"
                        class="textarea textarea-bordered"
                        placeholder="Position description"
                        rows="3"
                        bind:value={formData.description}></textarea>
                </div>
                <div class="modal-action">
                    <button
                        type="button"
                        class="btn"
                        on:click={() => (showModal = false)}>Cancel</button>
                    <button type="submit" class="btn btn-primary">
                        {editingPosition ? 'Update' : 'Create'}
                    </button>
                </div>
            </form>
        </div>
        <form
            method="dialog"
            class="modal-backdrop"
            on:click={() => (showModal = false)}>
            <button>close</button>
        </form>
    </dialog>
{/if}

{#if showPermissionModal && permissionPosition}
    <PermissionModal
        title="Permissions for {permissionPosition.name}"
        positionId={permissionPosition.id}
        on:close={() => (showPermissionModal = false)} />
{/if}
