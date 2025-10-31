<script lang="ts">
    import {
        Home,
        ChevronsLeft,
        ChevronLeft,
        ChevronRight,
        ChevronsRight,
    } from 'lucide-svelte';

    export let selectedDate = new Date().toISOString().slice(0, 10);

    const dayNames = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
    const current = new Date(selectedDate);
    let currentMonth = current.getMonth();
    let currentYear = current.getFullYear();

    function getStartDayOffset(year: number, month: number) {
        return new Date(year, month, 1).getDay();
    }
    function daysInMonth(year: number, month: number) {
        return new Date(year, month + 1, 0).getDate();
    }
    function isToday(year: number, month: number, date: number) {
        const today = new Date();
        return (
            today.getFullYear() === year &&
            today.getMonth() === month &&
            today.getDate() === date
        );
    }
    function selectDay(day: number) {
        const m = String(currentMonth + 1).padStart(2, '0');
        const d = String(day).padStart(2, '0');
        selectedDate = `${currentYear}-${m}-${d}`;
    }
    function prevMonth() {
        if (currentMonth === 0) {
            currentMonth = 11;
            currentYear -= 1;
        } else {
            currentMonth -= 1;
        }
    }
    function nextMonth() {
        if (currentMonth === 11) {
            currentMonth = 0;
            currentYear += 1;
        } else {
            currentMonth += 1;
        }
    }
    function prevYear() {
        currentYear -= 1;
    }
    function nextYear() {
        currentYear += 1;
    }
    function goToToday() {
        const today = new Date();
        currentMonth = today.getMonth();
        currentYear = today.getFullYear();
        selectedDate = today.toISOString().slice(0, 10);
    }
</script>

<h2 class="font-semibold mb-2">Calendar</h2>
<div class="flex items-center mb-2">
    <div>
        <button class="btn btn-xs" on:click={prevYear} title="Previous Year">
            <ChevronsLeft class="w-4 h-4" />
        </button>
        <button class="btn btn-xs" on:click={prevMonth} title="Previous Month">
            <ChevronLeft class="w-4 h-4" />
        </button>
    </div>
    <div class="font-semibold grow text-center">
        {new Date(currentYear, currentMonth).toLocaleString('default', {
            month: 'long',
        })}
        {currentYear}
    </div>
    <div>
        <button class="btn btn-xs" on:click={goToToday} title="Today">
            <Home class="w-4 h-4" />
        </button>
        <button class="btn btn-xs" on:click={nextMonth} title="Next Month">
            <ChevronRight class="w-4 h-4" />
        </button>
        <button class="btn btn-xs" on:click={nextYear} title="Next Year">
            <ChevronsRight class="w-4 h-4" />
        </button>
    </div>
</div>
<div class="grid grid-cols-7 gap-1 text-center font-semibold mb-1">
    {#each dayNames as dayName}
        <div class="text-sm">{dayName}</div>
    {/each}
</div>
<div class="grid grid-cols-7 gap-1 mb-4 text-center">
    {#each Array(getStartDayOffset(currentYear, currentMonth)).fill(0) as _}
        <div></div>
    {/each}

    {#each Array(daysInMonth(currentYear, currentMonth))
        .fill(0)
        .map((_, i) => i + 1) as day (day)}
        {#key day}
            <button
                class={`border border-base-300 p-2 rounded text-sm ${
                    selectedDate.endsWith('-' + String(day).padStart(2, '0'))
                        ? 'bg-primary text-primary-content'
                        : ''
                } ${
                    new Date(currentYear, currentMonth, day).getDay() === 6
                        ? 'text-error'
                        : ''
                } ${
                    isToday(currentYear, currentMonth, day)
                        ? 'text-accent font-semibold'
                        : ''
                }`}
                on:click={() => selectDay(day)}>
                {day}
            </button>
        {/key}
    {/each}
</div>
