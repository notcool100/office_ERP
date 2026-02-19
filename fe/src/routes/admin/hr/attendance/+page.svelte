<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import {
        Home,
        Users,
        Clock,
        LogIn,
        LogOut,
        BarChart3,
    } from 'lucide-svelte';
    import PageSection from '../../../../components/PageSection.svelte';
    import * as attendanceService from '$lib/services/attendance';
    import * as employeeService from '$lib/services/employee';
    import type {
        AttendanceRecord,
        AttendanceSummary,
    } from '$lib/types/attendance';
    import type { Employee } from '$lib/types/employee';
    import {
        navigationStore,
        canCreate,
        canRead,
        navigationLoading,
    } from '$lib/stores/navigation';
    import { userStore } from '$lib/stores/user';
    import { onMount } from 'svelte';

    pageTitle.set({
        title: 'Attendance System',
        desc: 'Track employee attendance',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'HR', icon: Users },
        { label: 'Attendance', icon: Clock },
    ]);

    let records: AttendanceRecord[] = [];
    let summary: AttendanceSummary | null = null;
    let total = 0;
    let page = 1;
    let pageSize = 10;
    let loading = false;
    let checkInEmployeeId = '';
    let selectedEmployeeId = '';
    let selectedEmployeeUuid = '';
    let summaryStartDate = '';
    let summaryEndDate = '';
    let employees: Employee[] = [];
    let employeeOptions: Employee[] = [];
    let currentEmployee: Employee | null = null;
    let currentEmployeeLoading = false;
    let allEmployeesLoading = false;
    let employeesLoading = false;
    let employeesError = '';
    let allEmployeesLoaded = false;
    let canSelectAllEmployees = false;
    let lockedToSelf = false;
    const navPath = '/admin/hr/attendance';

    async function loadRecords() {
        loading = true;
        try {
            const response = await attendanceService.listAttendance({
                page,
                pageSize,
                employeeId: selectedEmployeeUuid || undefined,
            });
            records = response.records;
            total = response.total;
        } catch (err) {
            console.error('Failed to load attendance records:', err);
        } finally {
            loading = false;
        }
    }

    async function handleCheckIn() {
        if (!checkInEmployeeId) {
            alert('Please select an employee');
            return;
        }

        try {
            await attendanceService.checkIn({
                employeeId: checkInEmployeeId,
            });
            if (canSelectAllEmployees) {
                checkInEmployeeId = '';
            }
            await loadRecords();
            alert('Check-in successful!');
        } catch (err) {
            console.error('Failed to check in:', err);
            alert('Failed to check in. You may have already checked in today.');
        }
    }

    async function handleCheckOut() {
        if (!checkInEmployeeId) {
            alert('Please select an employee');
            return;
        }

        try {
            await attendanceService.checkOut(checkInEmployeeId);
            if (canSelectAllEmployees) {
                checkInEmployeeId = '';
            }
            await loadRecords();
            alert('Check-out successful!');
        } catch (err) {
            console.error('Failed to check out:', err);
            alert('Failed to check out. Make sure you have checked in today.');
        }
    }

    async function loadSummary() {
        if (!selectedEmployeeId || !summaryStartDate || !summaryEndDate) {
            alert('Please select an employee and date range');
            return;
        }

        try {
            summary = await attendanceService.getAttendanceSummary(
                selectedEmployeeId,
                summaryStartDate,
                summaryEndDate,
            );
        } catch (err) {
            console.error('Failed to load summary:', err);
            alert('Failed to load attendance summary');
        }
    }

    function employeeLabel(employee: Employee): string {
        const nameParts = [
            employee.firstName,
            employee.middleName,
            employee.lastName,
        ].filter((part): part is string => Boolean(part));
        return `${nameParts.join(' ')} (${employee.employeeId})`;
    }

    async function loadCurrentEmployee() {
        currentEmployeeLoading = true;
        employeesError = '';
        try {
            const employee = await employeeService.getMyEmployee();
            currentEmployee = employee;
        } catch (err) {
            console.error('Failed to load current employee:', err);
            employeesError = 'Failed to load your employee record.';
        } finally {
            currentEmployeeLoading = false;
        }
    }

    async function loadAllEmployees() {
        allEmployeesLoading = true;
        employeesError = '';
        try {
            const response = await employeeService.listEmployees({
                pageSize: 1000,
                status: 'active',
            });
            employees = response.employees;
        } catch (err) {
            console.error('Failed to load employees:', err);
            employeesError = 'Failed to load employees list.';
        } finally {
            allEmployeesLoading = false;
            allEmployeesLoaded = true;
        }
    }

    onMount(() => {
        loadCurrentEmployee();
        // Set default summary dates to current month
        const now = new Date();
        summaryEndDate = now.toISOString().split('T')[0];
        const firstDay = new Date(now.getFullYear(), now.getMonth(), 1);
        summaryStartDate = firstDay.toISOString().split('T')[0];
    });

    $: canCreateHere = canCreate(navPath, $navigationStore);
    $: canReadHere = canRead(navPath, $navigationStore);
    $: canSelectAllEmployees =
        ($userStore.user?.isAdmin ?? false) ||
        canRead('/admin/hr/employee', $navigationStore);
    $: employeesLoading = currentEmployeeLoading || allEmployeesLoading;
    $: employeeOptions = [...employees].sort((a, b) =>
        employeeLabel(a).localeCompare(employeeLabel(b)),
    );
    $: if (currentEmployee && !$navigationLoading && !canSelectAllEmployees) {
        lockedToSelf = true;
        employees = [currentEmployee];
        if (!checkInEmployeeId) checkInEmployeeId = currentEmployee.employeeId;
        if (!selectedEmployeeId) selectedEmployeeId = currentEmployee.employeeId;
    }
    $: if (lockedToSelf && canSelectAllEmployees) {
        lockedToSelf = false;
        if (currentEmployee) {
            if (checkInEmployeeId === currentEmployee.employeeId) {
                checkInEmployeeId = '';
            }
            if (selectedEmployeeId === currentEmployee.employeeId) {
                selectedEmployeeId = '';
            }
        }
    }
    $: selectedEmployeeUuid =
        employees.find((e) => e.employeeId === selectedEmployeeId)?.id ?? '';
    $: if (canSelectAllEmployees && !allEmployeesLoaded && !allEmployeesLoading) {
        loadAllEmployees();
    }

    $: if (page && (canSelectAllEmployees || selectedEmployeeUuid)) {
        loadRecords();
    }

    const totalPages = Math.ceil(total / pageSize);

    function formatDateTime(datetime: string | undefined): string {
        if (!datetime) return 'N/A';
        return new Date(datetime).toLocaleString();
    }
</script>

<PageSection title="Attendance System">
    <div class="grid md:grid-cols-2 gap-6 mb-6">
        <!-- Check-in/Check-out Card -->
        <div class="card bg-base-200">
            <div class="card-body">
                <h3 class="card-title text-lg">Check In/Out</h3>

                <div class="form-control">
                    <label class="label" for="checkInEmployeeId">
                        <span class="label-text">Employee Name</span>
                    </label>
                    {#if canSelectAllEmployees}
                        <select
                            id="checkInEmployeeId"
                            class="select select-bordered"
                            bind:value={checkInEmployeeId}
                            disabled={employeesLoading || employeeOptions.length === 0}>
                            <option value="">Select employee</option>
                            {#each employeeOptions as employee}
                                <option value={employee.employeeId}>
                                    {employeeLabel(employee)}
                                </option>
                            {/each}
                        </select>
                    {:else}
                        <input
                            id="checkInEmployeeId"
                            type="text"
                            class="input input-bordered"
                            value={currentEmployee
                                ? employeeLabel(currentEmployee)
                                : employeesLoading
                                  ? 'Loading employee...'
                                  : ''}
                            readonly />
                    {/if}
                    {#if employeesError}
                        <p class="text-xs text-error mt-1">
                            {employeesError}
                        </p>
                    {/if}
                </div>

                <div class="flex gap-2 mt-4">
                    <button
                        class="btn btn-success flex-1"
                        on:click={handleCheckIn}
                        disabled={!canCreateHere}
                        title={!canCreateHere ? 'No permission to create' : ''}>
                        <LogIn size={20} />
                        Check In
                    </button>
                    <button
                        class="btn btn-error flex-1"
                        on:click={handleCheckOut}
                        disabled={!canCreateHere}
                        title={!canCreateHere ? 'No permission to create' : ''}>
                        <LogOut size={20} />
                        Check Out
                    </button>
                </div>
            </div>
        </div>

        <!-- Summary Card -->
        <div class="card bg-base-200">
            <div class="card-body">
                <h3 class="card-title text-lg flex items-center gap-2">
                    <BarChart3 size={20} />
                    Attendance Summary
                </h3>

                <div class="space-y-2">
                    <div class="form-control">
                        <label class="label" for="summaryEmployeeId">
                            <span class="label-text">Employee Name</span>
                        </label>
                        {#if canSelectAllEmployees}
                            <select
                                id="summaryEmployeeId"
                                class="select select-bordered select-sm"
                                bind:value={selectedEmployeeId}
                                disabled={employeesLoading || employeeOptions.length === 0}>
                                <option value="">Select employee</option>
                                {#each employeeOptions as employee}
                                    <option value={employee.employeeId}>
                                        {employeeLabel(employee)}
                                    </option>
                                {/each}
                            </select>
                        {:else}
                            <input
                                id="summaryEmployeeId"
                                type="text"
                                class="input input-bordered input-sm"
                                value={currentEmployee
                                    ? employeeLabel(currentEmployee)
                                    : employeesLoading
                                      ? 'Loading employee...'
                                      : ''}
                                readonly />
                        {/if}
                    </div>

                    <div class="grid grid-cols-2 gap-2">
                        <div class="form-control">
                            <label class="label" for="summaryStartDate">
                                <span class="label-text text-xs"
                                    >Start Date</span>
                            </label>
                            <input
                                id="summaryStartDate"
                                type="date"
                                class="input input-bordered input-sm"
                                bind:value={summaryStartDate} />
                        </div>
                        <div class="form-control">
                            <label class="label" for="summaryEndDate">
                                <span class="label-text text-xs">End Date</span>
                            </label>
                            <input
                                id="summaryEndDate"
                                type="date"
                                class="input input-bordered input-sm"
                                bind:value={summaryEndDate} />
                        </div>
                    </div>

                    <button
                        class="btn btn-primary btn-sm w-full"
                        on:click={loadSummary}
                        disabled={!canReadHere}
                        title={!canReadHere ? 'No permission to read' : ''}>
                        Generate Summary
                    </button>
                </div>

                {#if summary}
                    <div class="stats stats-vertical shadow mt-4">
                        <div class="stat">
                            <div class="stat-title">Total Days</div>
                            <div class="stat-value text-2xl">
                                {summary.totalDays}
                            </div>
                        </div>
                        <div class="stat">
                            <div class="stat-title">Present Days</div>
                            <div class="stat-value text-2xl text-success">
                                {summary.presentDays}
                            </div>
                        </div>
                        <div class="stat">
                            <div class="stat-title">Total Hours</div>
                            <div class="stat-value text-2xl">
                                {summary.totalHours.toFixed(1)}h
                            </div>
                        </div>
                    </div>
                {/if}
            </div>
        </div>
    </div>

    <!-- Attendance Records Table -->
    <div class="mb-4">
        <h3 class="text-lg font-semibold mb-2">Attendance Records</h3>
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
                        <th>Employee</th>
                        <th>Date</th>
                        <th>Check In</th>
                        <th>Check Out</th>
                        <th>Total Hours</th>
                        <th>Status</th>
                    </tr>
                </thead>
                <tbody>
                    {#each records as record}
                        <tr>
                            <td>{record.employeeName}</td>
                            <td>{record.date}</td>
                            <td>{formatDateTime(record.checkIn)}</td>
                            <td>{formatDateTime(record.checkOut)}</td>
                            <td>
                                {record.totalHours
                                    ? `${record.totalHours.toFixed(2)}h`
                                    : 'N/A'}
                            </td>
                            <td>
                                <span
                                    class="badge"
                                    class:badge-success={record.status ===
                                        'present'}
                                    class:badge-warning={record.status ===
                                        'late'}
                                    class:badge-error={record.status ===
                                        'absent'}>
                                    {record.status}
                                </span>
                            </td>
                        </tr>
                    {:else}
                        <tr>
                            <td colspan="6" class="text-center py-8"
                                >No attendance records found</td>
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
