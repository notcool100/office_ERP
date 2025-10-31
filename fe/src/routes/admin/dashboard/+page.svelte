<script lang="ts">
    import { breadcrumb } from '$lib/stores/breadcrumb';
    import { pageTitle } from '$lib/stores/page-title';
    import { Home, LayoutDashboard } from 'lucide-svelte';
    import SalesChart from '../../../components/charts/SalesChart.svelte';

    let stats = {
        sales: 24000,
        income: 18500,
        clients: 1254,
    };

    pageTitle.set({
        title: 'Dashboard',
        desc: 'Admin overview and activity',
    });

    breadcrumb.set([
        { label: 'Home', icon: Home },
        { label: 'Dashboard', icon: LayoutDashboard },
    ]);
</script>

<div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
    <div class="bg-base-200 p-4 rounded shadow">
        <h2 class="text-lg font-semibold">📈 Sales</h2>
        <p class="text-2xl mt-2">
            ${stats.sales.toLocaleString()}
        </p>
    </div>
    <div class="bg-base-200 p-4 rounded shadow">
        <h2 class="text-lg font-semibold">💵 Income</h2>
        <p class="text-2xl mt-2">
            ${stats.income.toLocaleString()}
        </p>
    </div>
    <div class="bg-base-200 p-4 rounded shadow">
        <h2 class="text-lg font-semibold">👥 Clients</h2>
        <p class="text-2xl mt-2">{stats.clients}</p>
    </div>
</div>

<!-- <div class="bg-base-200 p-6 rounded shadow">
    <h2 class="text-lg font-semibold mb-4">📊 Sales Overview</h2>
    <div>
        <SalesChart />
    </div>
</div> -->

<div class="bg-base-200 p-6 shadow">
    <div class="card-body">
        <h2 class="card-title">Latest Invoices</h2>
        <div class="overflow-x-auto">
            <table class="table">
                <thead>
                    <tr>
                        <th>Invoice ID</th>
                        <th>Client</th>
                        <th>Amount</th>
                        <th>Status</th>
                    </tr>
                </thead>
                <tbody>
                    <tr
                        ><td>INV-2031</td><td>XYZ Ltd</td><td>$1,200.00</td><td
                            ><span class="badge badge-success">Paid</span></td
                        ></tr>
                    <tr
                        ><td>INV-2032</td><td>Alpha Inc</td><td>$3,450.50</td
                        ><td
                            ><span class="badge badge-warning">Pending</span
                            ></td
                        ></tr>
                    <tr
                        ><td>INV-2033</td><td>Delta Co</td><td>$847.00</td><td
                            ><span class="badge badge-error">Overdue</span></td
                        ></tr>
                    <tr
                        ><td>INV-2034</td><td>Beta Group</td><td>$2,150.75</td
                        ><td><span class="badge badge-success">Paid</span></td
                        ></tr>
                </tbody>
            </table>
        </div>
    </div>
</div>
