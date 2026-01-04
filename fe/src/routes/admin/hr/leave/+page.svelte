<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import {
        Home,
        Users,
        CalendarMinus,
        Plus,
        CheckCircle,
        XCircle,
    } from 'lucide-svelte';
    import PageSection from '../../../../components/PageSection.svelte';
    import * as leaveService from '$lib/services/leave';
    import type {
        LeaveRequest,
        LeaveType,
        CreateLeaveRequestRequest,
        LeaveBalance,
    } from '$lib/types/leave';
    import { onMount } from 'svelte';

    pageTitle.set({
        title: 'Leave Management',
        desc: 'Manage leave requests and balances',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'HR', icon: Users },
        { label: 'Leave', icon: CalendarMinus },
    ]);

    let leaveRequests: LeaveRequest[] = [];
    let leaveTypes: LeaveType[] = [];
    let leaveBalances: LeaveBalance[] = [];
    let total = 0;
    let page = 1;
    let pageSize = 10;
    let statusFilter = 'all';
    let loading = false;
    let showRequestModal = false;
    let showBalanceModal = false;
    let selectedEmployeeId = '';

    let formData = {
        employeeId: '',
        leaveTypeId: '',
        startDate: '',
        endDate: '',
        reason: '',
    };

    async function loadLeaveRequests() {
        loading = true;
        try {
            const response = await leaveService.listLeaveRequests({
                page,
                pageSize,
                status: statusFilter !== 'all' ? statusFilter : undefined,
            });
            leaveRequests = response.requests;
            total = response.total;
        } catch (err) {
            console.error('Failed to load leave requests:', err);
        } finally {
            loading = false;
        }
    }

    async function loadLeaveTypes() {
        try {
            leaveTypes = await leaveService.getLeaveTypes();
        } catch (err) {
            console.error('Failed to load leave types:', err);
        }
    }

    async function loadLeaveBalance(employeeId: string) {
        try {
            leaveBalances = await leaveService.getLeaveBalance(employeeId);
            showBalanceModal = true;
        } catch (err) {
            console.error('Failed to load leave balance:', err);
            alert('Failed to load leave balance');
        }
    }

    function openRequestModal() {
        formData = {
            employeeId: '',
            leaveTypeId: '',
            startDate: new Date().toISOString().split('T')[0],
            endDate: '',
            reason: '',
        };
        showRequestModal = true;
    }

    async function handleSubmit() {
        try {
            const data: CreateLeaveRequestRequest = {
                employeeId: formData.employeeId,
                leaveTypeId: formData.leaveTypeId,
                startDate: formData.startDate,
                endDate: formData.endDate,
                reason: formData.reason || undefined,
            };
            await leaveService.createLeaveRequest(data);
            showRequestModal = false;
            await loadLeaveRequests();
        } catch (err) {
            console.error('Failed to create leave request:', err);
            alert('Failed to create leave request');
        }
    }

    async function approveRequest(id: string) {
        if (confirm('Are you sure you want to approve this leave request?')) {
            try {
                await leaveService.approveLeave(id);
                await loadLeaveRequests();
            } catch (err) {
                console.error('Failed to approve leave:', err);
                alert('Failed to approve leave');
            }
        }
    }

    async function rejectRequest(id: string) {
        if (confirm('Are you sure you want to reject this leave request?')) {
            try {
                await leaveService.rejectLeave(id);
                await loadLeaveRequests();
            } catch (err) {
                console.error('Failed to reject leave:', err);
                alert('Failed to reject leave');
            }
        }
    }

    onMount(() => {
        loadLeaveRequests();
        loadLeaveTypes();
    });

    $: if (statusFilter || page) {
        loadLeaveRequests();
    }

    const totalPages = Math.ceil(total / pageSize);
</script>

<PageSection title="Leave Management">
    <div class="mb-6 flex flex-wrap gap-4 justify-between items-center">
        <div class="flex gap-2 items-center">
            <select bind:value={statusFilter} class="select select-bordered">
                <option value="all">All Status</option>
                <option value="pending">Pending</option>
                <option value="approved">Approved</option>
                <option value="rejected">Rejected</option>
            </select>

            <input
                type="text"
                placeholder="Employee ID for balance"
                bind:value={selectedEmployeeId}
                class="input input-bordered" />
            <button
                class="btn btn-secondary"
                on:click={() => loadLeaveBalance(selectedEmployeeId)}>
                View Balance
            </button>
        </div>

        <button class="btn btn-primary" on:click={openRequestModal}>
            <Plus size={20} />
            New Leave Request
        </button>
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
                        <th>Leave Type</th>
                        <th>Start Date</th>
                        <th>End Date</th>
                        <th>Days</th>
                        <th>Status</th>
                        <th>Actions</th>
                    </tr>
                </thead>
                <tbody>
                    {#each leaveRequests as request}
                        <tr>
                            <td>{request.employeeName}</td>
                            <td>{request.leaveTypeName}</td>
                            <td>{request.startDate}</td>
                            <td>{request.endDate}</td>
                            <td>{request.totalDays}</td>
                            <td>
                                <span
                                    class="badge"
                                    class:badge-warning={request.status ===
                                        'pending'}
                                    class:badge-success={request.status ===
                                        'approved'}
                                    class:badge-error={request.status ===
                                        'rejected'}>
                                    {request.status}
                                </span>
                            </td>
                            <td>
                                {#if request.status === 'pending'}
                                    <div class="flex gap-2">
                                        <button
                                            class="btn btn-sm btn-success"
                                            on:click={() =>
                                                approveRequest(request.id)}>
                                            <CheckCircle size={16} />
                                            Approve
                                        </button>
                                        <button
                                            class="btn btn-sm btn-error"
                                            on:click={() =>
                                                rejectRequest(request.id)}>
                                            <XCircle size={16} />
                                            Reject
                                        </button>
                                    </div>
                                {:else}
                                    <span class="text-sm text-gray-500">
                                        {request.status === 'approved'
                                            ? 'Approved'
                                            : 'Rejected'}
                                        {#if request.approverName}
                                            by {request.approverName}
                                        {/if}
                                    </span>
                                {/if}
                            </td>
                        </tr>
                    {:else}
                        <tr>
                            <td colspan="7" class="text-center py-8"
                                >No leave requests found</td>
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

{#if showRequestModal}
    <div class="modal modal-open">
        <div class="modal-box">
            <h3 class="font-bold text-lg mb-4">New Leave Request</h3>

            <form on:submit|preventDefault={handleSubmit} class="space-y-4">
                <div class="form-control">
                    <label class="label" for="employeeId">
                        <span class="label-text">Employee ID</span>
                    </label>
                    <input
                        id="employeeId"
                        type="text"
                        class="input input-bordered"
                        bind:value={formData.employeeId}
                        required />
                </div>

                <div class="form-control">
                    <label class="label" for="leaveTypeId">
                        <span class="label-text">Leave Type</span>
                    </label>
                    <select
                        id="leaveTypeId"
                        class="select select-bordered"
                        bind:value={formData.leaveTypeId}
                        required>
                        <option value="">Select leave type</option>
                        {#each leaveTypes as type}
                            <option value={type.id}>{type.name}</option>
                        {/each}
                    </select>
                </div>

                <div class="grid grid-cols-2 gap-4">
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
                            <span class="label-text">End Date</span>
                        </label>
                        <input
                            id="endDate"
                            type="date"
                            class="input input-bordered"
                            bind:value={formData.endDate}
                            required />
                    </div>
                </div>

                <div class="form-control">
                    <label class="label" for="reason">
                        <span class="label-text">Reason</span>
                    </label>
                    <textarea
                        id="reason"
                        class="textarea textarea-bordered"
                        bind:value={formData.reason}></textarea>
                </div>

                <div class="modal-action">
                    <button
                        type="button"
                        class="btn"
                        on:click={() => (showRequestModal = false)}
                        >Cancel</button>
                    <button type="submit" class="btn btn-primary"
                        >Submit</button>
                </div>
            </form>
        </div>
        <div class="modal-backdrop" on:click={() => (showRequestModal = false)}>
        </div>
    </div>
{/if}

{#if showBalanceModal}
    <div class="modal modal-open">
        <div class="modal-box">
            <h3 class="font-bold text-lg mb-4">Leave Balance</h3>

            {#if leaveBalances.length > 0}
                <div class="space-y-4">
                    {#each leaveBalances as balance}
                        <div class="card bg-base-200">
                            <div class="card-body p-4">
                                <h4 class="font-semibold">
                                    {balance.leaveTypeName}
                                </h4>
                                <div class="grid grid-cols-3 gap-2 text-sm">
                                    <div>
                                        <p class="text-gray-500">
                                            Total Allowed
                                        </p>
                                        <p class="font-bold">
                                            {balance.totalAllowed} days
                                        </p>
                                    </div>
                                    <div>
                                        <p class="text-gray-500">Used</p>
                                        <p class="font-bold text-error">
                                            {balance.used} days
                                        </p>
                                    </div>
                                    <div>
                                        <p class="text-gray-500">Remaining</p>
                                        <p class="font-bold text-success">
                                            {balance.remaining} days
                                        </p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    {/each}
                </div>
            {:else}
                <p class="text-center py-8">No leave balance found</p>
            {/if}

            <div class="modal-action">
                <button class="btn" on:click={() => (showBalanceModal = false)}
                    >Close</button>
            </div>
        </div>
        <div class="modal-backdrop" on:click={() => (showBalanceModal = false)}>
        </div>
    </div>
{/if}
