<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, ClipboardList, Plus, Edit, Eye } from 'lucide-svelte';
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import PageSection from '../../../components/PageSection.svelte';
    import { projectService } from '$lib/services/project';
    import type { Project } from '$lib/types/project';
    import {
        navigationStore,
        canCreate,
        canUpdate,
    } from '$lib/stores/navigation';

    pageTitle.set({
        title: 'Projects',
        desc: 'Manage project boards and assignments',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Projects', icon: ClipboardList },
    ]);

    const navPath = '/admin/projects';
    let projects: Project[] = [];
    let loading = true;
    let showModal = false;
    let editingProject: Project | null = null;
    let errorMessage = '';

    let formData = {
        project_key: '',
        name: '',
        description: '',
        status: 'active',
    };

    async function loadProjects() {
        loading = true;
        errorMessage = '';
        try {
            projects = await projectService.list();
        } catch (error) {
            console.error('Failed to load projects:', error);
            errorMessage = 'Failed to load projects';
        } finally {
            loading = false;
        }
    }

    function openCreateModal() {
        editingProject = null;
        formData = {
            project_key: '',
            name: '',
            description: '',
            status: 'active',
        };
        showModal = true;
    }

    function openEditModal(project: Project) {
        editingProject = project;
        formData = {
            project_key: project.project_key,
            name: project.name,
            description: project.description || '',
            status: project.status,
        };
        showModal = true;
    }

    async function handleSubmit() {
        errorMessage = '';
        try {
            if (editingProject) {
                await projectService.update(editingProject.id, {
                    name: formData.name,
                    description: formData.description,
                    status: formData.status,
                });
            } else {
                await projectService.create({
                    project_key: formData.project_key,
                    name: formData.name,
                    description: formData.description || undefined,
                });
            }
            showModal = false;
            await loadProjects();
        } catch (error: any) {
            console.error('Failed to save project:', error);
            errorMessage = error?.message || 'Failed to save project';
        }
    }

    function openProject(project: Project) {
        goto(`/admin/projects/${project.id}`);
    }

    onMount(() => {
        loadProjects();
    });

    $: canCreateHere = canCreate(navPath, $navigationStore);
    $: canUpdateHere = canUpdate(navPath, $navigationStore);
</script>

<PageSection>
    <div class="flex items-center justify-between mb-2">
        <div class="text-sm text-error">{errorMessage}</div>
        <button
            class="btn btn-primary btn-sm"
            on:click={openCreateModal}
            disabled={!canCreateHere}
            title={!canCreateHere ? 'No permission to create' : ''}>
            <Plus class="w-4 h-4 mr-1" /> Add Project
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
                        <th>Key</th>
                        <th>Name</th>
                        <th>Status</th>
                        <th>Role</th>
                        <th class="text-right">Actions</th>
                    </tr>
                </thead>
                <tbody>
                    {#if projects.length === 0}
                        <tr>
                            <td colspan="5" class="text-center text-sm opacity-70">
                                No projects found.
                            </td>
                        </tr>
                    {:else}
                        {#each projects as project}
                            <tr>
                                <td class="font-mono">{project.project_key}</td>
                                <td>{project.name}</td>
                                <td class="capitalize">{project.status}</td>
                                <td class="capitalize">
                                    {project.member_role || 'member'}
                                </td>
                                <td class="text-right">
                                    <div class="join">
                                        <button
                                            class="btn btn-sm btn-ghost join-item"
                                            title="Open board"
                                            on:click={() =>
                                                openProject(project)}>
                                            <Eye class="w-4 h-4" />
                                        </button>
                                        <button
                                            class="btn btn-sm btn-ghost join-item"
                                            disabled={!canUpdateHere}
                                            title={!canUpdateHere
                                                ? 'No permission to update'
                                                : 'Edit project'}
                                            on:click={() =>
                                                openEditModal(project)}>
                                            <Edit class="w-4 h-4" />
                                        </button>
                                    </div>
                                </td>
                            </tr>
                        {/each}
                    {/if}
                </tbody>
            </table>
        </div>
    {/if}
</PageSection>

{#if showModal}
    <dialog class="modal modal-open">
        <div class="modal-box">
            <h3 class="font-bold text-lg mb-4">
                {editingProject ? 'Edit Project' : 'Create Project'}
            </h3>
            <form on:submit|preventDefault={handleSubmit}>
                <div class="form-control">
                    <label class="label" for="project_key">
                        <span class="label-text">Project Key *</span>
                    </label>
                    <input
                        id="project_key"
                        type="text"
                        placeholder="PRJ"
                        class="input input-bordered uppercase"
                        bind:value={formData.project_key}
                        disabled={!!editingProject}
                        required />
                </div>
                <div class="form-control mt-4">
                    <label class="label" for="name">
                        <span class="label-text">Name *</span>
                    </label>
                    <input
                        id="name"
                        type="text"
                        placeholder="Project name"
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
                        placeholder="Project description"
                        rows="3"
                        bind:value={formData.description}></textarea>
                </div>
                {#if editingProject}
                    <div class="form-control mt-4">
                        <label class="label" for="status">
                            <span class="label-text">Status</span>
                        </label>
                        <select
                            id="status"
                            class="select select-bordered"
                            bind:value={formData.status}>
                            <option value="active">Active</option>
                            <option value="archived">Archived</option>
                        </select>
                    </div>
                {/if}
                <div class="modal-action">
                    <button
                        type="button"
                        class="btn"
                        on:click={() => (showModal = false)}>Cancel</button>
                    <button
                        type="submit"
                        class="btn btn-primary"
                        disabled={
                            editingProject ? !canUpdateHere : !canCreateHere
                        }>
                        {editingProject ? 'Update' : 'Create'}
                    </button>
                </div>
            </form>
        </div>
        <form method="dialog" class="modal-backdrop">
            <button type="button" on:click={() => (showModal = false)}>
                close
            </button>
        </form>
    </dialog>
{/if}
