<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import {
        Home,
        Users,
        UserCheck,
        Plus,
        Search,
        Edit,
        Trash2,
    } from 'lucide-svelte';
    import PageSection from '../../../../components/PageSection.svelte';
    import * as employeeService from '$lib/services/employee';
    import { departmentService } from '$lib/services/department';
    import { positionService } from '$lib/services/position';
    import { personService } from '$lib/services/person';
    import type {
        Employee,
        CreateEmployeeRequest,
        UpdateEmployeeRequest,
    } from '$lib/types/employee';
    import type { Department } from '$lib/services/department';
    import type { Position } from '$lib/services/position';
    import type { Person } from '$lib/services/person';
    import { onMount } from 'svelte';

    pageTitle.set({
        title: 'Employee Management',
        desc: 'Manage employee records',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'HR', icon: Users },
        { label: 'Employee', icon: UserCheck },
    ]);

    let employees: Employee[] = [];
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
    let selectedEmployee: Employee | null = null;

    let formData = {
        employeeId: '',
        personId: '',
        department: '',
        position: '',
        hireDate: '',
        employmentType: 'Full-time',
        salary: 0,
        managerId: '',
    };

    async function loadEmployees() {
        loading = true;
        try {
            const response = await employeeService.listEmployees({
                page,
                pageSize,
                search: searchText || undefined,
                status: statusFilter !== 'all' ? statusFilter : undefined,
            });
            employees = response.employees;
            total = response.total;
        } catch (err) {
            console.error('Failed to load employees:', err);
        } finally {
            loading = false;
        }
    }

    function openCreateModal() {
        modalMode = 'create';
        selectedEmployee = null;
        formData = {
            employeeId: '',
            personId: '',
            department: '',
            position: '',
            hireDate: new Date().toISOString().split('T')[0],
            employmentType: 'Full-time',
            salary: 0,
            managerId: '',
        };
        showModal = true;
    }

    function openEditModal(employee: Employee) {
        modalMode = 'edit';
        selectedEmployee = employee;
        formData = {
            employeeId: employee.employeeId,
            personId: employee.personId,
            department: employee.department || '',
            position: employee.position || '',
            hireDate: employee.hireDate,
            employmentType: employee.employmentType || 'Full-time',
            salary: employee.salary || 0,
            managerId: employee.managerId || '',
        };
        showModal = true;
    }

    async function handleSubmit() {
        try {
            if (modalMode === 'create') {
                const createPayload = {
                    ...formData,
                    managerId: formData.managerId || undefined,
                };
                await employeeService.createEmployee(
                    createPayload as CreateEmployeeRequest,
                );
            } else if (selectedEmployee) {
                const updateData: UpdateEmployeeRequest = {
                    department: formData.department,
                    position: formData.position,
                    employmentType: formData.employmentType,
                    salary: formData.salary,
                    managerId: formData.managerId || undefined,
                };
                await employeeService.updateEmployee(
                    selectedEmployee.id,
                    updateData,
                );
            }
            showModal = false;
            await loadEmployees();
        } catch (err) {
            console.error('Failed to save employee:', err);
            alert('Failed to save employee');
        }
    }

    async function handleDelete(employee: Employee) {
        if (
            confirm(
                `Are you sure you want to delete ${employee.firstName} ${employee.lastName}?`,
            )
        ) {
            try {
                await employeeService.deleteEmployee(employee.id);
                await loadEmployees();
            } catch (err) {
                console.error('Failed to delete employee:', err);
                alert('Failed to delete employee');
            }
        }
    }

    async function loadRelatedData() {
        try {
            const [deptResponse, posResponse, personResponse] =
                await Promise.all([
                    departmentService.getAll(true),
                    positionService.getAll(true),
                    personService.getAll({ pageSize: 100 }), // Load top 100 persons for dropdown
                ]);
            departments = deptResponse;
            positions = posResponse;
            persons = personResponse.persons;
        } catch (err) {
            console.error('Failed to load related data:', err);
        }
    }

    onMount(() => {
        loadEmployees();
        loadRelatedData();
    });

    $: if (searchText !== undefined || statusFilter || page) {
        loadEmployees();
    }

    const totalPages = Math.ceil(total / pageSize);
</script>

<PageSection title="Employee Management">
    <div class="mb-6 flex flex-wrap gap-4 justify-between items-center">
        <div class="flex gap-2 flex-1 max-w-md">
            <input
                type="text"
                placeholder="Search employees..."
                bind:value={searchText}
                class="input input-bordered flex-1" />
            <button class="btn btn-square" on:click={loadEmployees}>
                <Search size={20} />
            </button>
        </div>

        <div class="flex gap-2 items-center">
            <select bind:value={statusFilter} class="select select-bordered">
                <option value="all">All Status</option>
                <option value="active">Active</option>
                <option value="inactive">Inactive</option>
                <option value="terminated">Terminated</option>
            </select>

            <button class="btn btn-primary" on:click={openCreateModal}>
                <Plus size={20} />
                Add Employee
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
                        <th>Employee ID</th>
                        <th>Name</th>
                        <th>Email</th>
                        <th>Department</th>
                        <th>Position</th>
                        <th>Status</th>
                        <th>Actions</th>
                    </tr>
                </thead>
                <tbody>
                    {#each employees as employee}
                        <tr>
                            <td>{employee.employeeId}</td>
                            <td
                                >{employee.firstName}
                                {employee.middleName || ''}
                                {employee.lastName}</td>
                            <td>{employee.email}</td>
                            <td>{employee.department || 'N/A'}</td>
                            <td>{employee.position || 'N/A'}</td>
                            <td>
                                <span
                                    class="badge"
                                    class:badge-success={employee.status ===
                                        'active'}
                                    class:badge-error={employee.status ===
                                        'terminated'}>
                                    {employee.status}
                                </span>
                            </td>
                            <td>
                                <div class="flex gap-2">
                                    <button
                                        class="btn btn-sm btn-ghost"
                                        on:click={() =>
                                            openEditModal(employee)}>
                                        <Edit size={16} />
                                    </button>
                                    <button
                                        class="btn btn-sm btn-ghost text-error"
                                        on:click={() => handleDelete(employee)}>
                                        <Trash2 size={16} />
                                    </button>
                                </div>
                            </td>
                        </tr>
                    {:else}
                        <tr>
                            <td colspan="7" class="text-center py-8"
                                >No employees found</td>
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
                {modalMode === 'create' ? 'Add New Employee' : 'Edit Employee'}
            </h3>

            <form on:submit|preventDefault={handleSubmit} class="space-y-4">
                <div class="grid grid-cols-2 gap-4">
                    <div class="form-control">
                        <label class="label" for="employeeId">
                            <span class="label-text">Employee ID</span>
                        </label>
                        <input
                            id="employeeId"
                            type="text"
                            class="input input-bordered"
                            bind:value={formData.employeeId}
                            disabled={modalMode === 'edit'}
                            placeholder="e.g. EMP-001"
                            required />
                    </div>

                    <div class="form-control">
                        <label class="label" for="personId">
                            <span class="label-text">Select Person</span>
                        </label>
                        <select
                            id="personId"
                            class="select select-bordered"
                            bind:value={formData.personId}
                            disabled={modalMode === 'edit'}
                            required>
                            <option value="" disabled>Select Person</option>
                            {#each persons as person}
                                <option value={person.id}>
                                    {person.first_name}
                                    {person.middle_name
                                        ? person.middle_name + ' '
                                        : ''}{person.last_name}
                                </option>
                            {/each}
                        </select>
                        {#if modalMode === 'create'}
                            <label class="label">
                                <a
                                    href="/admin/hr/person"
                                    class="label-text-alt link link-primary">
                                    + Add new person
                                </a>
                            </label>
                        {/if}
                    </div>

                    <div class="form-control">
                        <label class="label" for="department">
                            <span class="label-text">Department</span>
                        </label>
                        <select
                            id="department"
                            class="select select-bordered"
                            bind:value={formData.department}
                            required>
                            <option value="" disabled>Select Department</option>
                            {#each departments as dept}
                                <option value={dept.id}>{dept.name}</option>
                            {/each}
                        </select>
                    </div>

                    <div class="form-control">
                        <label class="label" for="position">
                            <span class="label-text">Position</span>
                        </label>
                        <select
                            id="position"
                            class="select select-bordered"
                            bind:value={formData.position}
                            required>
                            <option value="" disabled>Select Position</option>
                            {#each positions as pos}
                                <option value={pos.id}>{pos.name}</option>
                            {/each}
                        </select>
                    </div>

                    <div class="form-control">
                        <label class="label" for="hireDate">
                            <span class="label-text">Hire Date</span>
                        </label>
                        <input
                            id="hireDate"
                            type="date"
                            class="input input-bordered"
                            bind:value={formData.hireDate}
                            required />
                    </div>

                    <div class="form-control">
                        <label class="label" for="employmentType">
                            <span class="label-text">Employment Type</span>
                        </label>
                        <select
                            id="employmentType"
                            class="select select-bordered"
                            bind:value={formData.employmentType}>
                            <option value="Full-time">Full-time</option>
                            <option value="Part-time">Part-time</option>
                            <option value="Contract">Contract</option>
                        </select>
                    </div>

                    <div class="form-control">
                        <label class="label" for="salary">
                            <span class="label-text">Salary</span>
                        </label>
                        <input
                            id="salary"
                            type="number"
                            class="input input-bordered"
                            bind:value={formData.salary} />
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
