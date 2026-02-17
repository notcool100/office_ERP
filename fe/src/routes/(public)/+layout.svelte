<script>
    import '../../app.css';
    let open = $state(false);
    const { children } = $props();
</script>

<div class="drawer drawer-mobile h-screen overflow-hidden">
    <input
        id="drawer-toggle"
        type="checkbox"
        class="drawer-toggle"
        bind:checked={open} />
    <div class="drawer-content flex flex-col overflow-y-auto">
        <!-- Header -->
        <div class="navbar bg-base-100/80 backdrop-blur shadow px-4">
            <div class="flex-none lg:hidden">
                <label for="drawer-toggle" class="btn btn-square btn-ghost">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="h-6 w-6"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M4 6h16M4 12h16M4 18h16" />
                    </svg>
                </label>
            </div>
            <div class="flex-1">
                <button type="button" class="btn btn-ghost text-xl">
                    Team 1 | Free
                </button>
            </div>
            <div class="flex-none gap-4">
                <div class="form-control hidden md:block">
                    <input
                        type="text"
                        placeholder="Search"
                        class="input input-bordered w-24 md:w-auto" />
                </div>
                <div class="dropdown dropdown-end">
                    <button
                        type="button"
                        tabIndex={0}
                        class="btn btn-ghost btn-circle avatar">
                        <div class="w-10 rounded-full">
                            <img
                                src="https://i.pravatar.cc/300"
                                alt="User avatar" />
                        </div>
                    </button>
                </div>
            </div>
        </div>

        <!-- Main content -->
        <div class="p-6">
            {@render children()}
        </div>
    </div>

    <div class="drawer-side">
        <label for="drawer-toggle" class="drawer-overlay"></label>
        <aside
            class="menu p-4 w-80 bg-gradient-to-b from-base-200 to-base-100 text-base-content overflow-y-auto shadow-lg">
            <p class="text-lg font-bold mb-4">Overview</p>
            <ul class="menu">
                <li><button type="button" class="active">App</button></li>
                <li><button type="button">Ecommerce</button></li>
                <li><button type="button">Analytics</button></li>
                <li><button type="button">Banking</button></li>
                <li><button type="button">Booking</button></li>
                <li><button type="button">File</button></li>
                <li><button type="button">Course</button></li>
            </ul>

            <p class="text-lg font-bold mt-6 mb-4">Management</p>
            <ul class="menu">
                <li><button type="button">User</button></li>
                <li><button type="button">Cards</button></li>
                <li><button type="button">List</button></li>
                <li><button type="button">Create</button></li>
                <li><button type="button">Edit</button></li>
                <li><button type="button">Account</button></li>
                <li><button type="button">Product</button></li>
                <li><button type="button">Order</button></li>
                <li><button type="button">Invoice</button></li>
            </ul>
        </aside>
    </div>
</div>
