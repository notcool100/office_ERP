<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import {
        Home,
        Users,
        SquareUser,
        Plus,
        Search,
        Edit,
        Trash2,
    } from 'lucide-svelte';
    import { departmentService } from '$lib/services/department';
    import { positionService } from '$lib/services/position';
    import { personService } from '$lib/services/person';
    import PageSection from '../../../../components/PageSection.svelte';
    import * as internService from '$lib/services/intern';
    import type {
        Intern,
        CreateInternRequest,
        UpdateInternRequest,
    } from '$lib/types/intern';
    import type { Department } from '$lib/types/department';
    import type { Position } from '$lib/types/position';
    import type { Person } from '$lib/types/person';
    import { onMount } from 'svelte';

    pageTitle.set({
        title: 'Intern Management',
        desc: 'Manage intern records',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'HR', icon: Users },
        { label: 'Intern', icon: SquareUser },
    ]);

    let interns: Intern[] = [];
    let departments: Department[] = [];
    let positions: Position[] = [];
    let persons: Person[] = [];
    let total = 0;
    let page = 1;
    let pageSize = 10;
    let searchText = '';
    let statusFilter = 'all';
    let loading = false;
    let showModal = false;
    let modalMode: 'create' | 'edit' = 'create';
    let selectedIntern: Intern | null = null;

    let formData = {
        internId: '',
        personId: '',
        department: '',
        supervisorId: '',
        startDate: '',
        endDate: '',
        stipend: 0,
        university: '',
    };

    async function loadInterns() {
        loading = true;
        try {
            const response = await internService.listInterns({
                page,
                pageSize,
                search: searchText || undefined,
                status: statusFilter !== 'all' ? statusFilter : undefined,
            });
            interns = response.interns;
            total = response.total;
        } catch (err) {
            console.error('Failed to load interns:', err);
        } finally {
            loading = false;
        }
    }

    async function loadRelatedData() {
        try {
            const [deptResponse, posResponse, personResponse] =
                await Promise.all([
                    departmentService.getAll(true),
                    positionService.getAll(true),
                    personService.getAll({ pageSize: 100 }),
                ]);
            departments = deptResponse;
            positions = posResponse;
            persons = personResponse.persons;
        } catch (err) {
            console.error('Failed to load related data:', err);
        }
    }

    function openCreateModal() {
        modalMode = 'create';
        selectedIntern = null;
        formData = {
            internId: '',
            personId: '',
            department: '',
            supervisorId: '',
            startDate: new Date().toISOString().split('T')[0],
            endDate: '',
            stipend: 0,
            university: '',
        };
        showModal = true;
    }

    function openEditModal(intern: Intern) {
        modalMode = 'edit';
        selectedIntern = intern;
        formData = {
            internId: intern.internId,
            personId: intern.personId,
            department: intern.department || '',
            supervisorId: intern.supervisorId || '',
            startDate: intern.startDate,
            endDate: intern.endDate || '',
            stipend: intern.stipend || 0,
            university: intern.university || '',
        };
        showModal = true;
    }

    async function handleSubmit() {
        try {
            if (modalMode === 'create') {
                const createData: CreateInternRequest = {
                    internId: formData.internId,
                    personId: formData.personId,
                    department: formData.department || undefined,
                    supervisorId: formData.supervisorId || undefined,
                    startDate: formData.startDate,
                    endDate: formData.endDate || undefined,
                    stipend: formData.stipend,
                    university: formData.university || undefined,
                };
                await internService.createIntern(createData);
            } else if (selectedIntern) {
                const updateData: UpdateInternRequest = {
                    department: formData.department || undefined,
                    supervisorId: formData.supervisorId || undefined,
                    endDate: formData.endDate || undefined,
                    stipend: formData.stipend,
                    university: formData.university || undefined,
                };
                await internService.updateIntern(selectedIntern.id, updateData);
            }
            showModal = false;
            await loadInterns();
        } catch (err) {
            console.error('Failed to save intern:', err);
            alert('Failed to save intern');
        }
    }

    async function handleDelete(intern: Intern) {
        if (
            confirm(
                `Are you sure you want to delete intern ${intern.firstName} ${intern.lastName}?`,
            )
        ) {
            try {
                await internService.deleteIntern(intern.id);
                await loadInterns();
            } catch (err) {
                console.error('Failed to delete intern:', err);
                alert('Failed to delete intern');
            }
        }
    }

    onMount(() => {
        loadInterns();
        loadRelatedData();
    });

    $: if (searchText !== undefined || statusFilter || page) {
        loadInterns();
    }

    const totalPages = Math.ceil(total / pageSize);
</script>

<PageSection title="Intern Management">
    <div class="mb-6 flex flex-wrap gap-4 justify-between items-center">
        <div class="flex gap-2 flex-1 max-w-md">
            <input
                type="text"
                placeholder="Search interns..."
                bind:value={searchText}
                class="input input-bordered flex-1" />
            <button class="btn btn-square" on:click={loadInterns}>
                <Search size={20} />
            </button>
        </div>

        <div class="flex gap-2 items-center">
            <select bind:value={statusFilter} class="select select-bordered">
                <option value="all">All Status</option>
                <option value="active">Active</option>
                <option value="completed">Completed</option>
                <option value="terminated">Terminated</option>
            </select>

            <button class="btn btn-primary" on:click={openCreateModal}>
                <Plus size={20} />
                Add Intern
            </button>
        </div>
    </div>

    {#if loading}
        <div class="flex justify-center py-8">
            <span class="loading loading-spinner loading-lg"></span>
        </div>
    {:else}
        <div class="overflow-x-auto">
            <table class="table table-zebra">
                <thead>
                    <tr>
                        <th>Intern ID</th>
                        <th>Name</th>
                        <th>Email</th>
                        <th>Department</th>
                        <th>University</th>
                        <th>Start Date</th>
                        <th>Status</th>
                        <th>Actions</th>
                    </tr>
                </thead>
                <tbody>
                    {#each interns as intern}
                        <tr>
                            <td>{intern.internId}</td>
                            <td
                                >{intern.firstName}
                                {intern.middleName || ''}
                                {intern.lastName}</td>
                            <td>{intern.email}</td>
                            <td>{intern.department || 'N/A'}</td>
                            <td>{intern.university || 'N/A'}</td>
                            <td>{intern.startDate}</td>
                            <td>
                                <span
                                    class="badge"
                                    class:badge-success={intern.status ===
                                        'active'}
                                    class:badge-info={intern.status ===
                                        'completed'}
                                    class:badge-error={intern.status ===
                                        'terminated'}>
                                    {intern.status}
                                </span>
                            </td>
                            <td>
                                <div class="flex gap-2">
                                    <button
                                        class="btn btn-sm btn-ghost"
                                        on:click={() => openEditModal(intern)}>
                                        <Edit size={16} />
                                    </button>
                                    <button
                                        class="btn btn-sm btn-ghost text-error"
                                        on:click={() => handleDelete(intern)}>
                                        <Trash2 size={16} />
                                    </button>
                                </div>
                            </td>
                        </tr>
                    {:else}
                        <tr>
                            <td colspan="8" class="text-center py-8"
                                >No interns found</td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>

        <div class="flex justify-center mt-4 gap-2">
            <button
                class="btn btn-sm"
                disabled={page <= 1}
                on:click={() => (page = 1)}>First</button>
            <button
                class="btn btn-sm"
                disabled={page <= 1}
                on:click={() => page--}>Previous</button>
            <span class="flex items-center px-4"
                >Page {page} of {totalPages || 1}</span>
            <button
                class="btn btn-sm"
                disabled={page >= totalPages}
                on:click={() => page++}>Next</button>
            <button
                class="btn btn-sm"
                disabled={page >= totalPages}
                on:click={() => (page = totalPages)}>Last</button>
        </div>
    {/if}
</PageSection>

{#if showModal}
    <div class="modal modal-open">
        <div class="modal-box max-w-2xl">
            <h3 class="font-bold text-lg mb-4">
                {modalMode === 'create' ? 'Add New Intern' : 'Edit Intern'}
            </h3>

            <form on:submit|preventDefault={handleSubmit} class="space-y-4">
                <div class="grid grid-cols-2 gap-4">
                    <div class="form-control">
                        <label class="label" for="internId">
                            <span class="label-text">Intern ID</span>
                        </label>
                        <input
                            id="internId"
                            type="text"
                            class="input input-bordered"
                            bind:value={formData.internId}
                            disabled={modalMode === 'edit'}
                            required />
                    </div>

                    <div class="form-control">
                        <label class="label" for="department">
                            <span class="label-text">Department</span>
                        </label>
                        <input
                            id="department"
                            type="text"
                            class="input input-bordered"
                            bind:value={formData.department} />
                    </div>

                    <div class="form-control">
                        <label class="label" for="university">
                            <span class="label-text">University</span>
                        </label>
                        <input
                            id="university"
                            type="text"
                            class="input input-bordered"
                            bind:value={formData.university} />
                    </div>

                    <div class="form-control">
                        <label class="label" for="stipend">
                            <span class="label-text">Stipend</span>
                        </label>
                        <input
                            id="stipend"
                            type="number"
                            class="input input-bordered"
                            bind:value={formData.stipend} />
                    </div>

                    <div class="form-control">
                        <label class="label" for="startDate">
                            <span class="label-text">Start Date</span>
                        </label>
                        <input
                            id="startDate"
                            type="date"
                            class="input input-bordered"
                            bind:value={formData.startDate}
                            required />
                    </div>

                    <div class="form-control">
                        <label class="label" for="endDate">
                            <span class="label-text">End Date (Optional)</span>
                        </label>
                        <input
                            id="endDate"
                            type="date"
                            class="input input-bordered"
                            bind:value={formData.endDate} />
                    </div>
                </div>

                <div class="modal-action">
                    <button
                        type="button"
                        class="btn"
                        on:click={() => (showModal = false)}>Cancel</button>
                    <button type="submit" class="btn btn-primary">
                        {modalMode === 'create' ? 'Create' : 'Update'}
                    </button>
                </div>
            </form>
        </div>
        <div class="modal-backdrop" on:click={() => (showModal = false)}></div>
    </div>
{/if}
