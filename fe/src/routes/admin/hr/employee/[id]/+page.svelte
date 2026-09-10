<script lang="ts">
    import { onMount } from 'svelte';
    import { page } from '$app/stores';
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, Users, UserCheck, FileText, IdCard, ArrowLeft } from 'lucide-svelte';
    import PageSection from '../../../../../components/PageSection.svelte';
    import DocumentBrowser from '$lib/components/documents/DocumentBrowser.svelte';
    import * as employeeService from '$lib/services/employee';
    import { departmentService } from '$lib/services/department';
    import { positionService } from '$lib/services/position';
    import type { Employee } from '$lib/types/employee';
    import type { Department } from '$lib/services/department';
    import type { Position } from '$lib/services/position';

    const employeeId = $page.params.id;

    let employee: Employee | null = null;
    let departments: Department[] = [];
    let positions: Position[] = [];
    let loading = true;
    let errorMessage = '';
    let activeTab: 'details' | 'documents' = 'details';

    async function load() {
        loading = true;
        errorMessage = '';
        try {
            const [emp, deptList, posList] = await Promise.all([
                employeeService.getEmployee(employeeId),
                departmentService.getAll(true),
                positionService.getAll(true),
            ]);
            employee = emp;
            departments = deptList;
            positions = posList;

            pageTitle.set({
                title: `${emp.firstName} ${emp.lastName}`,
                desc: `Employee ${emp.employeeId}`,
            });
            breadcrumb.set([
                { label: 'Home', icon: Home },
                { label: 'HR', icon: Users },
                { label: 'Employee', icon: UserCheck },
                { label: `${emp.firstName} ${emp.lastName}`, icon: IdCard },
            ]);
        } catch (err) {
            console.error('Failed to load employee:', err);
            errorMessage = 'Failed to load employee';
        } finally {
            loading = false;
        }
    }

    function departmentName(id?: string) {
        if (!id) return 'N/A';
        return departments.find((d) => d.id === id)?.name || 'N/A';
    }

    function positionName(id?: string) {
        if (!id) return 'N/A';
        return positions.find((p) => p.id === id)?.name || 'N/A';
    }

    onMount(load);
</script>

<PageSection>
    <div class="flex items-center justify-between mb-4">
        <a href="/admin/hr/employee" class="btn btn-ghost btn-sm gap-1">
            <ArrowLeft size={16} />
            Back to Employees
        </a>
    </div>

    {#if loading}
        <div class="flex justify-center py-8">
            <span class="loading loading-spinner loading-lg"></span>
        </div>
    {:else if errorMessage}
        <div class="alert alert-error">{errorMessage}</div>
    {:else if employee}
        <div class="flex items-center gap-3 mb-6">
            <div class="avatar placeholder">
                <div class="bg-primary text-primary-content rounded-full w-14">
                    <span class="text-xl"
                        >{employee.firstName[0]}{employee.lastName[0]}</span>
                </div>
            </div>
            <div>
                <h2 class="text-xl font-bold">
                    {employee.firstName} {employee.middleName || ''} {employee.lastName}
                </h2>
                <p class="text-sm opacity-70">
                    {employee.employeeId} · {positionName(employee.position)} · {departmentName(
                        employee.department,
                    )}
                </p>
            </div>
            <span
                class="badge ml-auto"
                class:badge-success={employee.status === 'active'}
                class:badge-error={employee.status === 'terminated'}>
                {employee.status}
            </span>
        </div>

        <div role="tablist" class="tabs tabs-boxed mb-6 w-fit">
            <button
                role="tab"
                class="tab gap-1"
                class:tab-active={activeTab === 'details'}
                on:click={() => (activeTab = 'details')}>
                <IdCard size={16} />
                Details
            </button>
            <button
                role="tab"
                class="tab gap-1"
                class:tab-active={activeTab === 'documents'}
                on:click={() => (activeTab = 'documents')}>
                <FileText size={16} />
                Documents
            </button>
        </div>

        {#if activeTab === 'details'}
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 max-w-2xl">
                <div>
                    <div class="text-xs uppercase opacity-60">Email</div>
                    <div>{employee.email || 'N/A'}</div>
                </div>
                <div>
                    <div class="text-xs uppercase opacity-60">Phone</div>
                    <div>{employee.phone || 'N/A'}</div>
                </div>
                <div>
                    <div class="text-xs uppercase opacity-60">Department</div>
                    <div>{departmentName(employee.department)}</div>
                </div>
                <div>
                    <div class="text-xs uppercase opacity-60">Position</div>
                    <div>{positionName(employee.position)}</div>
                </div>
                <div>
                    <div class="text-xs uppercase opacity-60">Hire Date</div>
                    <div>{employee.hireDate}</div>
                </div>
                <div>
                    <div class="text-xs uppercase opacity-60">Employment Type</div>
                    <div>{employee.employmentType || 'N/A'}</div>
                </div>
                <div>
                    <div class="text-xs uppercase opacity-60">Salary</div>
                    <div>{employee.salary ?? 'N/A'}</div>
                </div>
                <div>
                    <div class="text-xs uppercase opacity-60">Status</div>
                    <div>{employee.status}</div>
                </div>
            </div>
        {:else}
            <DocumentBrowser
                category="employee"
                ownerId={employee.id}
                navPath="/admin/hr/employee"
                rootLabel="Employee Documents" />
        {/if}
    {/if}
</PageSection>
