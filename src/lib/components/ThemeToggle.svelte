<script lang="ts">
    import { themeStore } from "$lib/stores";
    import { toastStore } from "$lib/stores/toastStore";
    import { dropdownStore } from "$lib/stores/dropdownStore";

    let showDropdown = $state(false);
    let hoverTimeout: number | null = null;

    $effect(() => {
        const unsubscribe = dropdownStore.subscribe((activeDropdown) => {
            if (activeDropdown !== 'theme') {
                showDropdown = false;
                if (hoverTimeout) {
                    clearTimeout(hoverTimeout);
                    hoverTimeout = null;
                }
            }
        });
        return unsubscribe;
    });

    function handleMouseEnter() {
        hoverTimeout = window.setTimeout(() => {
            showDropdown = true;
            dropdownStore.open('theme');
        }, 2000);
    }

    function handleMouseLeave() {
        if (hoverTimeout) {
            clearTimeout(hoverTimeout);
            hoverTimeout = null;
        }
        showDropdown = false;
        dropdownStore.close();
    }

    function handleToggle() {
        themeStore.toggle();
        // Get the current theme value to show in toast
        let currentTheme = '';
        const unsubscribe = themeStore.subscribe(theme => {
            currentTheme = theme;
        });
        unsubscribe();
        const themeName = currentTheme.charAt(0).toUpperCase() + currentTheme.slice(1);
        toastStore.show(`Theme changed to ${themeName}`, "success");
    }

    function selectTheme(
        theme: "light" | "dark" | "sakura" | "tobacco" | "matcha",
    ) {
        themeStore.set(theme);
        const themeName = theme.charAt(0).toUpperCase() + theme.slice(1);
        toastStore.show(`Theme changed to ${themeName}`, "success");
        showDropdown = false;
        dropdownStore.close();
        if (hoverTimeout) {
            clearTimeout(hoverTimeout);
            hoverTimeout = null;
        }
    }
</script>

<div
    class="theme-toggle-container"
    role="group"
    aria-label="Theme selector"
    onmouseenter={handleMouseEnter}
    onmouseleave={handleMouseLeave}
>
    <button
        class="theme-toggle"
        onclick={handleToggle}
        title="Toggle theme"
    >
        {#if $themeStore === "light"}
            <svg
                class="icon"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
            >
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
            <svg
                class="icon"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
            >
                <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"
                ></path>
            </svg>
        {:else if $themeStore === "sakura"}
            <svg
                class="icon"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
            >
                <path
                    d="M12 2C12 2 10.5 3.5 10.5 6.5C10.5 9.5 12 11 12 11C12 11 13.5 9.5 13.5 6.5C13.5 3.5 12 2 12 2Z"
                ></path>
                <path
                    d="M12 13C12 13 10.5 14.5 10.5 17.5C10.5 20.5 12 22 12 22C12 22 13.5 20.5 13.5 17.5C13.5 14.5 12 13 12 13Z"
                ></path>
                <circle cx="12" cy="12" r="2"></circle>
            </svg>
        {:else if $themeStore === "tobacco"}
            <svg
                class="icon"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
            >
                <rect x="6" y="6" width="12" height="12" rx="2"></rect>
                <path d="M9 6V4C9 3.4 9.4 3 10 3H14C14.6 3 15 3.4 15 4V6"
                ></path>
                <path
                    d="M9 18V20C9 20.6 9.4 21 10 21H14C14.6 21 15 20.6 15 20V18"
                ></path>
            </svg>
        {:else}
            <svg
                class="icon"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
            >
                <circle cx="12" cy="12" r="3"></circle>
                <path d="M12 1v6m0 6v10M1 12h6m6 0h10"></path>
                <circle cx="12" cy="12" r="8" stroke-dasharray="2 4"></circle>
            </svg>
        {/if}
    </button>

    {#if showDropdown}
        <div class="theme-dropdown">
            <button
                class="theme-option"
                class:active={$themeStore === "light"}
                onclick={() => selectTheme("light")}
            >
                <svg
                    class="icon"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                >
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
                <span>Light</span>
            </button>
            <button
                class="theme-option"
                class:active={$themeStore === "dark"}
                onclick={() => selectTheme("dark")}
            >
                <svg
                    class="icon"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                >
                    <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"
                    ></path>
                </svg>
                <span>Dark</span>
            </button>
            <button
                class="theme-option"
                class:active={$themeStore === "sakura"}
                onclick={() => selectTheme("sakura")}
            >
                <svg
                    class="icon"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                >
                    <path
                        d="M12 2C12 2 10.5 3.5 10.5 6.5C10.5 9.5 12 11 12 11C12 11 13.5 9.5 13.5 6.5C13.5 3.5 12 2 12 2Z"
                    ></path>
                    <path
                        d="M12 13C12 13 10.5 14.5 10.5 17.5C10.5 20.5 12 22 12 22C12 22 13.5 20.5 13.5 17.5C13.5 14.5 12 13 12 13Z"
                    ></path>
                    <circle cx="12" cy="12" r="2"></circle>
                </svg>
                <span>Sakura</span>
            </button>
            <button
                class="theme-option"
                class:active={$themeStore === "tobacco"}
                onclick={() => selectTheme("tobacco")}
            >
                <svg
                    class="icon"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                >
                    <rect x="6" y="6" width="12" height="12" rx="2"></rect>
                    <path d="M9 6V4C9 3.4 9.4 3 10 3H14C14.6 3 15 3.4 15 4V6"
                    ></path>
                    <path
                        d="M9 18V20C9 20.6 9.4 21 10 21H14C14.6 21 15 20.6 15 20V18"
                    ></path>
                </svg>
                <span>Tobacco</span>
            </button>
            <button
                class="theme-option"
                class:active={$themeStore === "matcha"}
                onclick={() => selectTheme("matcha")}
            >
                <svg
                    class="icon"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                >
                    <circle cx="12" cy="12" r="3"></circle>
                    <path d="M12 1v6m0 6v10M1 12h6m6 0h10"></path>
                    <circle cx="12" cy="12" r="8" stroke-dasharray="2 4"
                    ></circle>
                </svg>
                <span>Matcha</span>
            </button>
        </div>
    {/if}
</div>

<style>
    .theme-toggle-container {
        position: relative;
    }

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

    .theme-dropdown {
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
        min-width: 150px;
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

    .theme-option {
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

    .theme-option:hover {
        background: var(--background-color);
    }

    .theme-option.active {
        background: var(--primary-color);
        color: white;
    }

    .theme-option .icon {
        width: 1rem;
        height: 1rem;
    }

    .theme-option span {
        flex: 1;
    }
</style>
