<script lang="ts">
    import { themeStore } from "$lib/stores";
</script>

<button class="theme-toggle" on:click={themeStore.toggle} title="Toggle theme">
    {#if $themeStore === "light"}
        <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <circle cx="12" cy="12" r="5"></circle>
            <line x1="12" y1="1" x2="12" y2="3"></line>
            <line x1="12" y1="21" x2="12" y2="23"></line>
            <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
            <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
            <line x1="1" y1="12" x2="3" y2="12"></line>
            <line x1="21" y1="12" x2="23" y2="12"></line>
            <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
            <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
        </svg>
    {:else if $themeStore === "dark"}
        <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
        </svg>
    {:else}
        <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path d="M12 2C12 2 10.5 3.5 10.5 6.5C10.5 9.5 12 11 12 11C12 11 13.5 9.5 13.5 6.5C13.5 3.5 12 2 12 2Z"></path>
            <path d="M12 13C12 13 10.5 14.5 10.5 17.5C10.5 20.5 12 22 12 22C12 22 13.5 20.5 13.5 17.5C13.5 14.5 12 13 12 13Z"></path>
            <circle cx="12" cy="12" r="2"></circle>
        </svg>
    {/if}
</button>

<style>
    .theme-toggle {
        background: var(--surface-color);
        border: 1px solid var(--border-color);
        border-radius: 0.5rem;
        padding: 0.75rem;
        cursor: pointer;
        color: var(--text-color);
        transition: all 0.2s ease;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .theme-toggle:hover {
        background: var(--background-color);
        border-color: var(--primary-color);
        transform: scale(1.05);
    }

    .icon {
        width: 1.25rem;
        height: 1.25rem;
        stroke-width: 2;
    }
</style>
