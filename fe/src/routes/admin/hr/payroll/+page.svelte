<script lang="ts">
    import { onMount } from 'svelte';
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { api } from '$lib/services/api';
    import { Home, Users, Banknote, Play, CheckCircle, Trash2, Plus, Pencil, X, ChevronLeft, DollarSign } from 'lucide-svelte';
    import { canCreate, canUpdate, canDelete, navigationStore } from '$lib/stores/navigation';

    pageTitle.set({ title: 'Payroll', desc: 'Manage salary structures and monthly payroll runs' });
    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'HR', icon: Users },
        { label: 'Payroll', icon: Banknote },
    ]);

    type Tab = 'runs' | 'structures' | 'myslips';
    let activeTab: Tab = 'runs';

    // ── Payroll Runs ──────────────────────────────────────────────────────────
    let runs: any[] = [];
    let runsLoading = true;
    let selectedRun: any = null;
    let runDetail: any = null;
    let runDetailLoading = false;

    // New run modal
    let showNewRun = false;
    let newRunMonth = new Date().getMonth() + 1;
    let newRunYear = new Date().getFullYear();
    let newRunWorkingDays = 22;
    let newRunNotes = '';
    let runSaving = false;

    // ── Salary Structures ─────────────────────────────────────────────────────
    let structures: any[] = [];
    let structuresLoading = true;
    let employees: any[] = [];
    let showStructureForm = false;
    let editingStructure: any = null;

    let ssEmployeeId = '';
    let ssEffectiveDate = new Date().toISOString().slice(0, 10);
    let ssBasic = '';
    let ssHousing = '';
    let ssTransport = '';
    let ssMeal = '';
    let ssOtherAllow = '';
    let ssTaxPct = '';
    let ssSsPct = '';
    let ssOtherDed = '';
    let ssNotes = '';
    let ssSaving = false;

    // ── Adjustments ───────────────────────────────────────────────────────────
    let editingEntry: any = null;
    let adjBonus = '';
    let adjOvertime = '';
    let adjOtherDed = '';
    let adjDaysPresent = '';
    let adjNotes = '';
    let adjSaving = false;

    // ── My Payslips ───────────────────────────────────────────────────────────
    let myPayslips: any[] = [];
    let mySlipsLoading = true;

    // ── Toast ─────────────────────────────────────────────────────────────────
    let toast = { show: false, msg: '', ok: true };
    function showToast(msg: string, ok = true) {
        toast = { show: true, msg, ok };
        setTimeout(() => (toast = { ...toast, show: false }), 3500);
    }

    onMount(async () => {
        await Promise.all([loadRuns(), loadStructures(), loadMySlips(), loadEmployees()]);
    });

    async function loadRuns() {
        runsLoading = true;
        const res = await api.get('/payroll/runs');
        if (res.ok) runs = await res.json();
        runsLoading = false;
    }

    async function loadStructures() {
        structuresLoading = true;
        const res = await api.get('/payroll/salary-structures');
        if (res.ok) structures = await res.json();
        structuresLoading = false;
    }

    async function loadMySlips() {
        mySlipsLoading = true;
        const res = await api.get('/payroll/my-payslips');
        if (res.ok) myPayslips = await res.json();
        mySlipsLoading = false;
    }

    async function loadEmployees() {
        const res = await api.get('/employees');
        if (res.ok) {
            const data = await res.json();
            employees = data?.employees ?? data ?? [];
        }
    }

    async function openRun(run: any) {
        selectedRun = run;
        runDetailLoading = true;
        const res = await api.get(`/payroll/runs/${run.id}`);
        if (res.ok) runDetail = await res.json();
        runDetailLoading = false;
    }

    function backToRuns() {
        selectedRun = null;
        runDetail = null;
    }

    async function createRun() {
        if (newRunMonth < 1 || newRunMonth > 12) return;
        runSaving = true;
        const res = await api.post('/payroll/runs', {
            month: newRunMonth,
            year: newRunYear,
            workingDays: newRunWorkingDays,
            notes: newRunNotes || null,
        });
        runSaving = false;
        if (res.ok) {
            showNewRun = false;
            newRunNotes = '';
            await loadRuns();
            showToast('Payroll run created');
        } else {
            const e = await res.json();
            showToast(e.error ?? 'Failed to create run', false);
        }
    }

    async function processRun(run: any) {
        if (!confirm(`Process payroll for ${monthName(run.month)} ${run.year}? This will calculate all employee salaries.`)) return;
        const res = await api.post(`/payroll/runs/${run.id}/process`, {});
        if (res.ok) {
            await loadRuns();
            const data = await res.json();
            runDetail = data;
            selectedRun = data.run;
            showToast('Payroll processed successfully');
        } else {
            const e = await res.json();
            showToast(e.error ?? 'Failed to process', false);
        }
    }

    async function markPaid(run: any) {
        if (!confirm(`Mark ${monthName(run.month)} ${run.year} payroll as PAID?`)) return;
        const res = await api.post(`/payroll/runs/${run.id}/pay`, {});
        if (res.ok) {
            await loadRuns();
            if (selectedRun?.id === run.id) {
                const dr = await api.get(`/payroll/runs/${run.id}`);
                if (dr.ok) runDetail = await dr.json();
                selectedRun = runDetail.run;
            }
            showToast('Marked as paid');
        } else {
            const e = await res.json();
            showToast(e.error ?? 'Failed', false);
        }
    }

    async function deleteRun(run: any) {
        if (!confirm(`Delete draft run for ${monthName(run.month)} ${run.year}?`)) return;
        const res = await api.delete(`/payroll/runs/${run.id}`);
        if (res.ok) {
            await loadRuns();
            if (selectedRun?.id === run.id) backToRuns();
            showToast('Run deleted');
        } else {
            const e = await res.json();
            showToast(e.error ?? 'Failed', false);
        }
    }

    // ── Salary Structures ─────────────────────────────────────────────────────

    function openNewStructure() {
        editingStructure = null;
        ssEmployeeId = '';
        ssEffectiveDate = new Date().toISOString().slice(0, 10);
        ssBasic = ssHousing = ssTransport = ssMeal = ssOtherAllow = '';
        ssTaxPct = ssSsPct = ssOtherDed = ssNotes = '';
        showStructureForm = true;
    }

    function openEditStructure(s: any) {
        editingStructure = s;
        ssEmployeeId = s.employeeId;
        ssEffectiveDate = s.effectiveDate;
        ssBasic = s.basicSalary;
        ssHousing = s.housingAllowance;
        ssTransport = s.transportAllowance;
        ssMeal = s.mealAllowance;
        ssOtherAllow = s.otherAllowance;
        ssTaxPct = s.incomeTaxPct;
        ssSsPct = s.socialSecurityPct;
        ssOtherDed = s.otherDeductionFixed;
        ssNotes = s.notes ?? '';
        showStructureForm = true;
    }

    async function saveStructure() {
        if (!ssEmployeeId || !ssBasic) return;
        ssSaving = true;
        const body = {
            employeeId: ssEmployeeId,
            effectiveDate: ssEffectiveDate,
            basicSalary: parseFloat(ssBasic) || 0,
            housingAllowance: parseFloat(ssHousing) || 0,
            transportAllowance: parseFloat(ssTransport) || 0,
            mealAllowance: parseFloat(ssMeal) || 0,
            otherAllowance: parseFloat(ssOtherAllow) || 0,
            incomeTaxPct: parseFloat(ssTaxPct) || 0,
            socialSecurityPct: parseFloat(ssSsPct) || 0,
            otherDeductionFixed: parseFloat(ssOtherDed) || 0,
            notes: ssNotes || null,
        };
        const res = await api.post('/payroll/salary-structures', body);
        ssSaving = false;
        if (res.ok) {
            showStructureForm = false;
            await loadStructures();
            showToast('Salary structure saved');
        } else {
            const e = await res.json();
            showToast(e.error ?? 'Failed', false);
        }
    }

    async function deleteStructure(id: string) {
        if (!confirm('Delete this salary structure?')) return;
        const res = await api.delete(`/payroll/salary-structures/${id}`);
        if (res.ok) {
            await loadStructures();
            showToast('Deleted');
        } else {
            showToast('Failed to delete', false);
        }
    }

    // ── Entry Adjustment ──────────────────────────────────────────────────────

    function openAdjust(entry: any) {
        editingEntry = entry;
        adjBonus = entry.bonus;
        adjOvertime = entry.overtimePay;
        adjOtherDed = entry.otherDeduction;
        adjDaysPresent = entry.daysPresent;
        adjNotes = entry.notes ?? '';
    }

    async function saveAdjustment() {
        if (!editingEntry) return;
        adjSaving = true;
        const res = await api.put(`/payroll/entries/${editingEntry.id}`, {
            bonus: parseFloat(adjBonus) || 0,
            overtimePay: parseFloat(adjOvertime) || 0,
            otherDeduction: parseFloat(adjOtherDed) || 0,
            daysPresent: parseFloat(adjDaysPresent) || 0,
            notes: adjNotes || null,
        });
        adjSaving = false;
        if (res.ok) {
            const updated = await res.json();
            if (runDetail) {
                runDetail = {
                    ...runDetail,
                    entries: runDetail.entries.map((e: any) => e.id === editingEntry.id ? updated : e),
                };
            }
            editingEntry = null;
            showToast('Adjustment saved');
        } else {
            const e = await res.json();
            showToast(e.error ?? 'Failed', false);
        }
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    const NAV_PATH = '/admin/hr/payroll';
    $: canC = canCreate(NAV_PATH, $navigationStore);
    $: canU = canUpdate(NAV_PATH, $navigationStore);
    $: canD = canDelete(NAV_PATH, $navigationStore);

    const MONTHS = ['Jan','Feb','Mar','Apr','May','Jun','Jul','Aug','Sep','Oct','Nov','Dec'];
    function monthName(m: number) { return MONTHS[m - 1] ?? m; }

    function fmt(val: any) {
        const n = parseFloat(val ?? 0);
        return n.toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 });
    }

    function statusBadge(s: string) {
        if (s === 'paid') return 'badge-success';
        if (s === 'processed') return 'badge-warning';
        return 'badge-ghost';
    }

    function employeeName(id: string) {
        const e = employees.find(e => e.id === id);
        if (!e) return id;
        return `${e.firstName ?? ''} ${e.lastName ?? ''}`.trim();
    }
</script>

<!-- Toast -->
{#if toast.show}
    <div class="toast toast-top toast-end z-50">
        <div class="alert {toast.ok ? 'alert-success' : 'alert-error'} text-sm py-2">
            {toast.msg}
        </div>
    </div>
{/if}

<div class="p-6 space-y-5 max-w-7xl mx-auto">

    <!-- Header -->
    <div class="flex items-center justify-between">
        <div>
            <h1 class="text-xl font-bold">Payroll</h1>
            <p class="text-sm text-base-content/50">Manage salary structures and monthly payroll runs</p>
        </div>
    </div>

    <!-- Tabs -->
    <div class="tabs tabs-border">
        <button class="tab {activeTab === 'runs' ? 'tab-active' : ''}" onclick={() => activeTab = 'runs'}>
            Payroll Runs
        </button>
        <button class="tab {activeTab === 'structures' ? 'tab-active' : ''}" onclick={() => activeTab = 'structures'}>
            Salary Structures
        </button>
        <button class="tab {activeTab === 'myslips' ? 'tab-active' : ''}" onclick={() => activeTab = 'myslips'}>
            My Payslips
        </button>
    </div>

    <!-- ── Payroll Runs tab ── -->
    {#if activeTab === 'runs'}
        {#if selectedRun}
            <!-- Run Detail -->
            <div class="space-y-4">
                <div class="flex items-center gap-3">
                    <button class="btn btn-ghost btn-sm gap-1" onclick={backToRuns}>
                        <ChevronLeft size={16} /> Back
                    </button>
                    <h2 class="font-semibold text-lg">
                        {monthName(selectedRun.month)} {selectedRun.year}
                        <span class="badge {statusBadge(selectedRun.status)} ml-2">{selectedRun.status}</span>
                    </h2>
                    <div class="ml-auto flex gap-2">
                        {#if selectedRun.status === 'draft' && canC}
                            <button class="btn btn-primary btn-sm gap-1" onclick={() => processRun(selectedRun)}>
                                <Play size={14} /> Process
                            </button>
                        {/if}
                        {#if selectedRun.status === 'processed' && canU}
                            <button class="btn btn-success btn-sm gap-1" onclick={() => markPaid(selectedRun)}>
                                <CheckCircle size={14} /> Mark Paid
                            </button>
                        {/if}
                    </div>
                </div>

                {#if runDetailLoading}
                    <div class="flex justify-center py-12">
                        <span class="loading loading-spinner loading-md"></span>
                    </div>
                {:else if runDetail}
                    <!-- Summary bar -->
                    <div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
                        {#each [
                            { label: 'Employees', value: runDetail.entries.length },
                            { label: 'Working Days', value: runDetail.run.workingDays },
                            { label: 'Total Gross', value: fmt(runDetail.entries.reduce((s: number, e: any) => s + parseFloat(e.grossPay ?? 0), 0)) },
                            { label: 'Total Net Pay', value: fmt(runDetail.entries.reduce((s: number, e: any) => s + parseFloat(e.netPay ?? 0), 0)) },
                        ] as kpi}
                            <div class="stat bg-base-100 border border-base-300 rounded-box p-4">
                                <div class="stat-title text-xs">{kpi.label}</div>
                                <div class="stat-value text-lg">{kpi.value}</div>
                            </div>
                        {/each}
                    </div>

                    <!-- Entries table -->
                    <div class="bg-base-100 border border-base-300 rounded-box overflow-x-auto">
                        <table class="table table-sm">
                            <thead>
                                <tr class="text-xs text-base-content/50">
                                    <th>Employee</th>
                                    <th>Department</th>
                                    <th class="text-right">Days</th>
                                    <th class="text-right">Basic</th>
                                    <th class="text-right">Gross</th>
                                    <th class="text-right">Deductions</th>
                                    <th class="text-right">Net Pay</th>
                                    {#if selectedRun.status !== 'paid' && canU}
                                        <th></th>
                                    {/if}
                                </tr>
                            </thead>
                            <tbody>
                                {#each runDetail.entries as entry}
                                    <tr class="hover">
                                        <td>
                                            <div class="font-medium text-sm">{entry.employeeName}</div>
                                            <div class="text-xs text-base-content/40">{entry.employeeCode}</div>
                                        </td>
                                        <td class="text-xs text-base-content/60">{entry.department ?? '—'}</td>
                                        <td class="text-right text-sm">{entry.daysPresent}/{entry.workingDays}</td>
                                        <td class="text-right text-sm">{fmt(entry.basicSalary)}</td>
                                        <td class="text-right text-sm font-medium">{fmt(entry.grossPay)}</td>
                                        <td class="text-right text-sm text-error">({fmt(entry.totalDeductions)})</td>
                                        <td class="text-right font-bold text-success">{fmt(entry.netPay)}</td>
                                        {#if selectedRun.status !== 'paid' && canU}
                                            <td>
                                                <button class="btn btn-ghost btn-xs" onclick={() => openAdjust(entry)}>
                                                    <Pencil size={12} />
                                                </button>
                                            </td>
                                        {/if}
                                    </tr>
                                {/each}
                            </tbody>
                        </table>
                        {#if runDetail.entries.length === 0}
                            <div class="text-center py-10 text-base-content/40 text-sm">
                                No entries — click Process to calculate payroll
                            </div>
                        {/if}
                    </div>
                {/if}
            </div>

        {:else}
            <!-- Runs list -->
            <div class="space-y-3">
                {#if canC}
                    <div class="flex justify-end">
                        <button class="btn btn-primary btn-sm gap-1" onclick={() => showNewRun = true}>
                            <Plus size={15} /> New Payroll Run
                        </button>
                    </div>
                {/if}

                {#if runsLoading}
                    <div class="flex justify-center py-12"><span class="loading loading-spinner loading-md"></span></div>
                {:else if runs.length === 0}
                    <div class="text-center py-16 text-base-content/40">
                        <Banknote size={40} class="mx-auto mb-3 opacity-30" />
                        <p>No payroll runs yet. Create one to get started.</p>
                    </div>
                {:else}
                    <div class="bg-base-100 border border-base-300 rounded-box overflow-x-auto">
                        <table class="table">
                            <thead>
                                <tr class="text-xs text-base-content/50">
                                    <th>Period</th>
                                    <th>Status</th>
                                    <th class="text-right">Employees</th>
                                    <th class="text-right">Total Net Pay</th>
                                    <th>Actions</th>
                                </tr>
                            </thead>
                            <tbody>
                                {#each runs as run}
                                    <tr class="hover cursor-pointer" onclick={() => openRun(run)}>
                                        <td class="font-medium">{monthName(run.month)} {run.year}</td>
                                        <td><span class="badge {statusBadge(run.status)} badge-sm">{run.status}</span></td>
                                        <td class="text-right">{run.entryCount}</td>
                                        <td class="text-right font-semibold">{fmt(run.totalNetPay)}</td>
                                        <td onclick={(e) => e.stopPropagation()}>
                                            <div class="flex gap-1">
                                                {#if run.status === 'draft' && canC}
                                                    <button class="btn btn-primary btn-xs gap-1" onclick={() => processRun(run)}>
                                                        <Play size={11} /> Process
                                                    </button>
                                                {/if}
                                                {#if run.status === 'processed' && canU}
                                                    <button class="btn btn-success btn-xs gap-1" onclick={() => markPaid(run)}>
                                                        <CheckCircle size={11} /> Pay
                                                    </button>
                                                {/if}
                                                {#if run.status === 'draft' && canD}
                                                    <button class="btn btn-ghost btn-xs text-error" onclick={() => deleteRun(run)}>
                                                        <Trash2 size={11} />
                                                    </button>
                                                {/if}
                                            </div>
                                        </td>
                                    </tr>
                                {/each}
                            </tbody>
                        </table>
                    </div>
                {/if}
            </div>
        {/if}
    {/if}

    <!-- ── Salary Structures tab ── -->
    {#if activeTab === 'structures'}
        <div class="space-y-3">
            {#if canC}
                <div class="flex justify-end">
                    <button class="btn btn-primary btn-sm gap-1" onclick={openNewStructure}>
                        <Plus size={15} /> Add Structure
                    </button>
                </div>
            {/if}

            {#if structuresLoading}
                <div class="flex justify-center py-12"><span class="loading loading-spinner loading-md"></span></div>
            {:else if structures.length === 0}
                <div class="text-center py-16 text-base-content/40">
                    <DollarSign size={40} class="mx-auto mb-3 opacity-30" />
                    <p>No salary structures defined. Add one per employee.</p>
                </div>
            {:else}
                <div class="bg-base-100 border border-base-300 rounded-box overflow-x-auto">
                    <table class="table table-sm">
                        <thead>
                            <tr class="text-xs text-base-content/50">
                                <th>Employee</th>
                                <th>Effective</th>
                                <th class="text-right">Basic</th>
                                <th class="text-right">Allowances</th>
                                <th class="text-right">Tax %</th>
                                <th class="text-right">SS %</th>
                                <th class="text-right">Total Package</th>
                                <th></th>
                            </tr>
                        </thead>
                        <tbody>
                            {#each structures as s}
                                {@const totalAllow = parseFloat(s.housingAllowance ?? 0) + parseFloat(s.transportAllowance ?? 0) + parseFloat(s.mealAllowance ?? 0) + parseFloat(s.otherAllowance ?? 0)}
                                {@const total = parseFloat(s.basicSalary ?? 0) + totalAllow}
                                <tr class="hover">
                                    <td>
                                        <div class="font-medium text-sm">{s.employeeName}</div>
                                        <div class="text-xs text-base-content/40">{s.employeeCode}</div>
                                    </td>
                                    <td class="text-sm">{s.effectiveDate}</td>
                                    <td class="text-right text-sm">{fmt(s.basicSalary)}</td>
                                    <td class="text-right text-sm">{fmt(totalAllow)}</td>
                                    <td class="text-right text-sm">{s.incomeTaxPct}%</td>
                                    <td class="text-right text-sm">{s.socialSecurityPct}%</td>
                                    <td class="text-right font-semibold text-sm">{fmt(total)}</td>
                                    <td>
                                        <div class="flex gap-1">
                                            {#if canU}
                                                <button class="btn btn-ghost btn-xs" onclick={() => openEditStructure(s)}>
                                                    <Pencil size={12} />
                                                </button>
                                            {/if}
                                            {#if canD}
                                                <button class="btn btn-ghost btn-xs text-error" onclick={() => deleteStructure(s.id)}>
                                                    <Trash2 size={12} />
                                                </button>
                                            {/if}
                                        </div>
                                    </td>
                                </tr>
                            {/each}
                        </tbody>
                    </table>
                </div>
            {/if}
        </div>
    {/if}

    <!-- ── My Payslips tab ── -->
    {#if activeTab === 'myslips'}
        {#if mySlipsLoading}
            <div class="flex justify-center py-12"><span class="loading loading-spinner loading-md"></span></div>
        {:else if myPayslips.length === 0}
            <div class="text-center py-16 text-base-content/40">
                <Banknote size={40} class="mx-auto mb-3 opacity-30" />
                <p>No payslips available yet.</p>
            </div>
        {:else}
            <div class="grid gap-4">
                {#each myPayslips as slip}
                    <div class="bg-base-100 border border-base-300 rounded-box p-5">
                        <div class="flex items-center justify-between mb-4">
                            <div>
                                <h3 class="font-bold text-base">{monthName(slip.month)} {slip.year}</h3>
                                <span class="badge {statusBadge(slip.status)} badge-sm">{slip.status}</span>
                            </div>
                            <div class="text-right">
                                <div class="text-xs text-base-content/40">Net Pay</div>
                                <div class="text-2xl font-bold text-success">{fmt(slip.netPay)}</div>
                            </div>
                        </div>
                        <div class="grid grid-cols-2 sm:grid-cols-4 gap-3 text-sm">
                            <div>
                                <div class="text-xs text-base-content/40">Basic</div>
                                <div class="font-medium">{fmt(slip.basicSalary)}</div>
                            </div>
                            <div>
                                <div class="text-xs text-base-content/40">Allowances</div>
                                <div class="font-medium">{fmt(parseFloat(slip.housingAllowance) + parseFloat(slip.transportAllowance) + parseFloat(slip.mealAllowance) + parseFloat(slip.otherAllowance))}</div>
                            </div>
                            <div>
                                <div class="text-xs text-base-content/40">Bonus / Overtime</div>
                                <div class="font-medium">{fmt(parseFloat(slip.bonus) + parseFloat(slip.overtimePay))}</div>
                            </div>
                            <div>
                                <div class="text-xs text-base-content/40">Days Present</div>
                                <div class="font-medium">{slip.daysPresent} / {slip.workingDays}</div>
                            </div>
                            <div>
                                <div class="text-xs text-base-content/40">Gross Pay</div>
                                <div class="font-medium">{fmt(slip.grossPay)}</div>
                            </div>
                            <div>
                                <div class="text-xs text-base-content/40">Income Tax</div>
                                <div class="font-medium text-error">({fmt(slip.incomeTax)})</div>
                            </div>
                            <div>
                                <div class="text-xs text-base-content/40">Social Security</div>
                                <div class="font-medium text-error">({fmt(slip.socialSecurity)})</div>
                            </div>
                            <div>
                                <div class="text-xs text-base-content/40">Other Deductions</div>
                                <div class="font-medium text-error">({fmt(slip.otherDeduction)})</div>
                            </div>
                        </div>
                    </div>
                {/each}
            </div>
        {/if}
    {/if}
</div>

<!-- ── New Run Modal ── -->
{#if showNewRun}
    <div class="modal modal-open">
        <div class="modal-box max-w-sm">
            <h3 class="font-bold text-base mb-4">New Payroll Run</h3>
            <div class="space-y-3">
                <div class="grid grid-cols-2 gap-3">
                    <div class="form-control">
                        <label class="label pb-1" for="run-month"><span class="label-text text-xs">Month</span></label>
                        <select id="run-month" class="select select-bordered select-sm" bind:value={newRunMonth}>
                            {#each MONTHS as m, i}
                                <option value={i + 1}>{m}</option>
                            {/each}
                        </select>
                    </div>
                    <div class="form-control">
                        <label class="label pb-1" for="run-year"><span class="label-text text-xs">Year</span></label>
                        <input id="run-year" type="number" class="input input-bordered input-sm" bind:value={newRunYear} min="2020" max="2099" />
                    </div>
                </div>
                <div class="form-control">
                    <label class="label pb-1" for="run-wd"><span class="label-text text-xs">Working Days</span></label>
                    <input id="run-wd" type="number" class="input input-bordered input-sm" bind:value={newRunWorkingDays} min="1" max="31" />
                </div>
                <div class="form-control">
                    <label class="label pb-1" for="run-notes"><span class="label-text text-xs">Notes (optional)</span></label>
                    <input id="run-notes" type="text" class="input input-bordered input-sm" bind:value={newRunNotes} />
                </div>
            </div>
            <div class="modal-action">
                <button class="btn btn-ghost btn-sm" onclick={() => showNewRun = false}>Cancel</button>
                <button class="btn btn-primary btn-sm" onclick={createRun} disabled={runSaving}>
                    {#if runSaving}<span class="loading loading-spinner loading-xs"></span>{/if}
                    Create
                </button>
            </div>
        </div>
        <button class="modal-backdrop" onclick={() => showNewRun = false}></button>
    </div>
{/if}

<!-- ── Salary Structure Form ── -->
{#if showStructureForm}
    <div class="modal modal-open">
        <div class="modal-box max-w-2xl">
            <h3 class="font-bold text-base mb-4">{editingStructure ? 'Edit' : 'Add'} Salary Structure</h3>
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                <div class="form-control sm:col-span-2">
                    <label class="label pb-1" for="ss-emp"><span class="label-text text-xs">Employee</span></label>
                    <select id="ss-emp" class="select select-bordered select-sm" bind:value={ssEmployeeId} disabled={!!editingStructure}>
                        <option value="">Select employee...</option>
                        {#each employees as emp}
                            <option value={emp.id}>{emp.firstName} {emp.lastName} ({emp.employeeId})</option>
                        {/each}
                    </select>
                </div>
                <div class="form-control">
                    <label class="label pb-1" for="ss-date"><span class="label-text text-xs">Effective Date</span></label>
                    <input id="ss-date" type="date" class="input input-bordered input-sm" bind:value={ssEffectiveDate} />
                </div>
                <div class="form-control">
                    <label class="label pb-1" for="ss-basic"><span class="label-text text-xs">Basic Salary</span></label>
                    <input id="ss-basic" type="number" step="0.01" class="input input-bordered input-sm" bind:value={ssBasic} placeholder="0.00" />
                </div>
                <div class="form-control">
                    <label class="label pb-1" for="ss-housing"><span class="label-text text-xs">Housing Allowance</span></label>
                    <input id="ss-housing" type="number" step="0.01" class="input input-bordered input-sm" bind:value={ssHousing} placeholder="0.00" />
                </div>
                <div class="form-control">
                    <label class="label pb-1" for="ss-transport"><span class="label-text text-xs">Transport Allowance</span></label>
                    <input id="ss-transport" type="number" step="0.01" class="input input-bordered input-sm" bind:value={ssTransport} placeholder="0.00" />
                </div>
                <div class="form-control">
                    <label class="label pb-1" for="ss-meal"><span class="label-text text-xs">Meal Allowance</span></label>
                    <input id="ss-meal" type="number" step="0.01" class="input input-bordered input-sm" bind:value={ssMeal} placeholder="0.00" />
                </div>
                <div class="form-control">
                    <label class="label pb-1" for="ss-other-allow"><span class="label-text text-xs">Other Allowance</span></label>
                    <input id="ss-other-allow" type="number" step="0.01" class="input input-bordered input-sm" bind:value={ssOtherAllow} placeholder="0.00" />
                </div>
                <div class="form-control">
                    <label class="label pb-1" for="ss-tax"><span class="label-text text-xs">Income Tax %</span></label>
                    <input id="ss-tax" type="number" step="0.01" min="0" max="100" class="input input-bordered input-sm" bind:value={ssTaxPct} placeholder="0.00" />
                </div>
                <div class="form-control">
                    <label class="label pb-1" for="ss-ss"><span class="label-text text-xs">Social Security %</span></label>
                    <input id="ss-ss" type="number" step="0.01" min="0" max="100" class="input input-bordered input-sm" bind:value={ssSsPct} placeholder="0.00" />
                </div>
                <div class="form-control">
                    <label class="label pb-1" for="ss-ded"><span class="label-text text-xs">Fixed Other Deduction</span></label>
                    <input id="ss-ded" type="number" step="0.01" class="input input-bordered input-sm" bind:value={ssOtherDed} placeholder="0.00" />
                </div>
                <div class="form-control sm:col-span-2">
                    <label class="label pb-1" for="ss-notes"><span class="label-text text-xs">Notes</span></label>
                    <input id="ss-notes" type="text" class="input input-bordered input-sm" bind:value={ssNotes} />
                </div>
            </div>
            <div class="modal-action">
                <button class="btn btn-ghost btn-sm" onclick={() => showStructureForm = false}>Cancel</button>
                <button class="btn btn-primary btn-sm" onclick={saveStructure} disabled={ssSaving || !ssEmployeeId || !ssBasic}>
                    {#if ssSaving}<span class="loading loading-spinner loading-xs"></span>{/if}
                    Save
                </button>
            </div>
        </div>
        <button class="modal-backdrop" onclick={() => showStructureForm = false}></button>
    </div>
{/if}

<!-- ── Adjustment Modal ── -->
{#if editingEntry}
    <div class="modal modal-open">
        <div class="modal-box max-w-sm">
            <div class="flex items-center justify-between mb-4">
                <h3 class="font-bold text-base">Adjust — {editingEntry.employeeName}</h3>
                <button class="btn btn-ghost btn-xs btn-circle" onclick={() => editingEntry = null}><X size={14} /></button>
            </div>
            <div class="space-y-3">
                <div class="form-control">
                    <label class="label pb-1" for="adj-days"><span class="label-text text-xs">Days Present</span></label>
                    <input id="adj-days" type="number" step="0.5" min="0" class="input input-bordered input-sm" bind:value={adjDaysPresent} />
                </div>
                <div class="form-control">
                    <label class="label pb-1" for="adj-bonus"><span class="label-text text-xs">Bonus</span></label>
                    <input id="adj-bonus" type="number" step="0.01" class="input input-bordered input-sm" bind:value={adjBonus} />
                </div>
                <div class="form-control">
                    <label class="label pb-1" for="adj-ot"><span class="label-text text-xs">Overtime Pay</span></label>
                    <input id="adj-ot" type="number" step="0.01" class="input input-bordered input-sm" bind:value={adjOvertime} />
                </div>
                <div class="form-control">
                    <label class="label pb-1" for="adj-ded"><span class="label-text text-xs">Other Deduction</span></label>
                    <input id="adj-ded" type="number" step="0.01" class="input input-bordered input-sm" bind:value={adjOtherDed} />
                </div>
                <div class="form-control">
                    <label class="label pb-1" for="adj-notes"><span class="label-text text-xs">Notes</span></label>
                    <input id="adj-notes" type="text" class="input input-bordered input-sm" bind:value={adjNotes} />
                </div>
            </div>
            <div class="modal-action">
                <button class="btn btn-ghost btn-sm" onclick={() => editingEntry = null}>Cancel</button>
                <button class="btn btn-primary btn-sm" onclick={saveAdjustment} disabled={adjSaving}>
                    {#if adjSaving}<span class="loading loading-spinner loading-xs"></span>{/if}
                    Save
                </button>
            </div>
        </div>
        <button class="modal-backdrop" onclick={() => editingEntry = null}></button>
    </div>
{/if}
