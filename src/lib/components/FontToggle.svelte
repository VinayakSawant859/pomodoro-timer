<script lang="ts">
    import { fontStore } from "$lib/stores";

    let showDropdown = $state(false);
    let hoverTimeout: number | null = null;

    function handleMouseEnter() {
        hoverTimeout = window.setTimeout(() => {
            showDropdown = true;
        }, 2000);
    }

    function handleMouseLeave() {
        if (hoverTimeout) {
            clearTimeout(hoverTimeout);
            hoverTimeout = null;
        }
        showDropdown = false;
    }

    function selectFont(font: "default" | "josefin" | "cause") {
        fontStore.set(font);
        showDropdown = false;
        if (hoverTimeout) {
            clearTimeout(hoverTimeout);
            hoverTimeout = null;
        }
    }
</script>

<div
    class="font-toggle-container"
    role="group"
    aria-label="Font selector"
    onmouseenter={handleMouseEnter}
    onmouseleave={handleMouseLeave}
>
    <button
        class="font-toggle"
        onclick={fontStore.toggle}
        title="Toggle font"
    >
        <svg
            class="icon"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
        >
            <path d="M4 7V4h16v3"></path>
            <path d="M9 20h6"></path>
            <path d="M12 4v16"></path>
        </svg>
    </button>

    {#if showDropdown}
        <div class="font-dropdown">
            <button
                class="font-option"
                class:active={$fontStore === "default"}
                onclick={() => selectFont("default")}
            >
                <span style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;">Default</span>
            </button>
            <button
                class="font-option"
                class:active={$fontStore === "josefin"}
                onclick={() => selectFont("josefin")}
            >
                <span style="font-family: 'Josefin Sans', sans-serif;">Josefin Sans</span>
            </button>
            <button
                class="font-option"
                class:active={$fontStore === "cause"}
                onclick={() => selectFont("cause")}
            >
                <span style="font-family: 'Cause', cursive;">Cause</span>
            </button>
        </div>
    {/if}
</div>

<style>
    .font-toggle-container {
        position: relative;
    }

    .font-toggle {
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

    .font-toggle:hover {
        background: var(--background-color);
        border-color: var(--primary-color);
        transform: scale(1.05);
    }

    .icon {
        width: 1.25rem;
        height: 1.25rem;
        stroke-width: 2;
    }

    .font-dropdown {
        position: absolute;
        top: calc(100% + 0.5rem);
        right: 0;
        background: var(--surface-color);
        border: 1px solid var(--border-color);
        border-radius: 0.5rem;
        box-shadow: 0 4px 12px var(--shadow);
        padding: 0.5rem;
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        min-width: 180px;
        z-index: 1000;
        animation: slideDown 0.2s ease;
    }

    @keyframes slideDown {
        from {
            opacity: 0;
            transform: translateY(-10px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }

    .font-option {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        padding: 0.75rem 1rem;
        background: transparent;
        border: none;
        border-radius: 0.375rem;
        color: var(--text-color);
        cursor: pointer;
        transition: all 0.2s ease;
        font-size: 0.9rem;
        text-align: left;
    }

    .font-option:hover {
        background: var(--background-color);
    }

    .font-option.active {
        background: var(--primary-color);
        color: white;
    }

    .font-option span {
        flex: 1;
        font-size: 1rem;
    }
</style>
