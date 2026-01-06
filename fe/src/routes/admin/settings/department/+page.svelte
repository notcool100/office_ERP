<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import {
        Home,
        Settings,
        Building2,
        Plus,
        Edit,
        Trash2,
        Shield,
    } from 'lucide-svelte';
    import { onMount } from 'svelte';
    import PageSection from '../../../../components/PageSection.svelte';
    import PermissionModal from '../../../../components/PermissionModal.svelte';
    import {
        departmentService,
        type Department,
    } from '$lib/services/department';

    pageTitle.set({
        title: 'Departments',
        desc: 'Manage organizational departments',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Settings', icon: Settings },
        { label: 'Departments', icon: Building2 },
    ]);

    let departments: Department[] = [];
    let loading = true;
    let showModal = false;
    let showPermissionModal = false;
    let editingDepartment: Department | null = null;
    let permissionDepartment: Department | null = null;
    let formData = {
        name: '',
        description: '',
    };

    async function loadDepartments() {
        loading = true;
        try {
            departments = await departmentService.getAll();
        } catch (error) {
            console.error('Failed to load departments:', error);
        } finally {
            loading = false;
        }
    }

    function openCreateModal() {
        editingDepartment = null;
        formData = { name: '', description: '' };
        showModal = true;
    }

    function openEditModal(dept: Department) {
        editingDepartment = dept;
        formData = {
            name: dept.name,
            description: dept.description || '',
        };
        showModal = true;
    }

    function openPermissionModal(dept: Department) {
        permissionDepartment = dept;
        showPermissionModal = true;
    }

    async function handleSubmit() {
        try {
            if (editingDepartment) {
                await departmentService.update(editingDepartment.id, formData);
            } else {
                await departmentService.create(formData);
            }
            showModal = false;
            await loadDepartments();
        } catch (error) {
            console.error('Failed to save department:', error);
        }
    }

    async function handleDelete(id: string) {
        if (confirm('Are you sure you want to delete this department?')) {
            try {
                await departmentService.delete(id);
                await loadDepartments();
            } catch (error) {
                console.error('Failed to delete department:', error);
            }
        }
    }

    async function toggleActive(dept: Department) {
        try {
            await departmentService.update(dept.id, {
                is_active: !dept.is_active,
            });
            await loadDepartments();
        } catch (error) {
            console.error('Failed to toggle department status:', error);
        }
    }

    onMount(() => {
        loadDepartments();
    });
</script>

<PageSection>
    <div class="text-right mb-2">
        <button class="btn btn-primary btn-sm" on:click={openCreateModal}>
            <Plus class="w-4 h-4 mr-1" /> Add Department
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
                    {#each departments as dept}
                        <tr>
                            <td>{dept.name}</td>
                            <td>{dept.description || '-'}</td>
                            <td>
                                <div class="form-control">
                                    <label
                                        class="label cursor-pointer gap-2 justify-start">
                                        <input
                                            type="checkbox"
                                            class="toggle toggle-success toggle-sm"
                                            checked={dept.is_active}
                                            on:change={() =>
                                                toggleActive(dept)} />
                                        <span class="label-text"
                                            >{dept.is_active
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
                                            openPermissionModal(dept)}>
                                        <Shield class="w-4 h-4" />
                                    </button>
                                    <button
                                        class="btn btn-sm btn-ghost join-item"
                                        on:click={() => openEditModal(dept)}>
                                        <Edit class="w-4 h-4" />
                                    </button>
                                    <button
                                        class="btn btn-sm btn-ghost join-item text-error"
                                        on:click={() => handleDelete(dept.id)}>
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
                {editingDepartment ? 'Edit Department' : 'Create Department'}
            </h3>
            <form on:submit|preventDefault={handleSubmit}>
                <div class="form-control">
                    <label class="label" for="name">
                        <span class="label-text">Name *</span>
                    </label>
                    <input
                        id="name"
                        type="text"
                        placeholder="Department name"
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
                        placeholder="Department description"
                        rows="3"
                        bind:value={formData.description}></textarea>
                </div>
                <div class="modal-action">
                    <button
                        type="button"
                        class="btn"
                        on:click={() => (showModal = false)}>Cancel</button>
                    <button type="submit" class="btn btn-primary">
                        {editingDepartment ? 'Update' : 'Create'}
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

{#if showPermissionModal && permissionDepartment}
    <PermissionModal
        title="Permissions for {permissionDepartment.name}"
        departmentId={permissionDepartment.id}
        on:close={() => (showPermissionModal = false)} />
{/if}
