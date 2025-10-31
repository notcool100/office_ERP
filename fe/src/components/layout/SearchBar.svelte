<script lang="ts">
    import { Search } from 'lucide-svelte';
</script>

<div class="relative mb-3 px-1">
    <input
        type="text"
        placeholder="Search..."
        class="w-full pl-7 pb-1 bg-transparent text-sm placeholder-base-content/50 border-b border-base-content/10 focus:border-base-content/30 focus:outline-none transition-colors" />

    <Search
        class="w-4 h-4 absolute left-2 top-1/2 -translate-y-1/2 text-base-content/50 pointer-events-none" />
</div>
