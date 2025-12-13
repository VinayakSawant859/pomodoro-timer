<script lang="ts">
    import { themeStore } from "$lib/stores";
    import { toastStore } from "$lib/stores/toastStore";
    import { dropdownStore } from "$lib/stores/dropdownStore";

    let showDropdown = $state(false);
    let hoverTimeout: number | null = null;

    $effect(() => {
        const unsubscribe = dropdownStore.subscribe((activeDropdown) => {
            if (activeDropdown !== "theme") {
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
            dropdownStore.open("theme");
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
        let currentTheme = "";
        const unsubscribe = themeStore.subscribe((theme) => {
            currentTheme = theme;
        });
        unsubscribe();
        const themeNames: Record<string, string> = {
            light: "Light",
            dark: "Dark",
            academia: "Dark Academia",
            sakura: "Sakura",
            tobacco: "Tobacco",
            forest: "Forest",
            pastel: "Pastel",
        };
        const themeName = themeNames[currentTheme] || currentTheme;
        toastStore.show(`Theme changed to ${themeName}`, "success");
    }

    function selectTheme(
        theme: "light" | "dark" | "academia" | "sakura" | "coffee" | "forest" | "minimal" | "anime",
    ) {
        themeStore.set(theme);
        const themeNames: Record<string, string> = {
            light: "Light",
            dark: "Dark",
            academia: "Dark Academia",
            sakura: "Sakura",
            tobacco: "Tobacco",
            forest: "Forest",
            pastel: "Pastel",
        };
        const themeName = themeNames[theme] || theme;
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
    <button class="theme-toggle" onclick={handleToggle} title="Toggle theme">
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
        {:else if $themeStore === "academia"}
            <svg
                class="icon"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
            >
                <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"></path>
                <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"></path>
                <path d="M12 6v8"></path>
                <path d="M9 9h6"></path>
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
        {:else if $themeStore === "coffee"}
            <svg
                class="icon"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
            >
                <path d="M17 8h1a4 4 0 0 1 0 8h-1"></path>
                <path d="M3 8h14v9a4 4 0 0 1-4 4H7a4 4 0 0 1-4-4Z"></path>
                <line x1="6" y1="2" x2="6" y2="4"></line>
                <line x1="10" y1="2" x2="10" y2="4"></line>
                <line x1="14" y1="2" x2="14" y2="4"></line>
            </svg>
        {:else if $themeStore === "forest"}
            <svg
                class="icon"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
            >
                <path d="M12 2L2 7l10 5 10-5-10-5z"></path>
                <path d="M2 17l10 5 10-5"></path>
                <path d="M2 12l10 5 10-5"></path>
            </svg>
        {:else if $themeStore === "minimal"}
            <svg
                class="icon"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
            >
                <rect x="3" y="3" width="18" height="18" rx="2"></rect>
                <line x1="3" y1="9" x2="21" y2="9"></line>
                <line x1="9" y1="21" x2="9" y2="9"></line>
            </svg>
        {:else}
            <svg
                class="icon"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
            >
                <polygon points="12,2 15.09,8.26 22,9.27 17,14.14 18.18,21.02 12,17.77 5.82,21.02 7,14.14 2,9.27 8.91,8.26"></polygon>
                <circle cx="12" cy="12" r="3" fill="currentColor"></circle>
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
                class:active={$themeStore === "academia"}
                onclick={() => selectTheme("academia")}
            >
                <svg
                    class="icon"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                >
                    <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"></path>
                    <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"></path>
                    <path d="M12 6v8"></path>
                    <path d="M9 9h6"></path>
                </svg>
                <span>Academia</span>
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
                class:active={$themeStore === "coffee"}
                onclick={() => selectTheme("coffee")}
            >
                <svg
                    class="icon"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                >
                    <path d="M17 8h1a4 4 0 0 1 0 8h-1"></path>
                    <path d="M3 8h14v9a4 4 0 0 1-4 4H7a4 4 0 0 1-4-4Z"></path>
                    <line x1="6" y1="2" x2="6" y2="4"></line>
                    <line x1="10" y1="2" x2="10" y2="4"></line>
                    <line x1="14" y1="2" x2="14" y2="4"></line>
                </svg>
                <span>Warm Coffee</span>
            </button>
            <button
                class="theme-option"
                class:active={$themeStore === "forest"}
                onclick={() => selectTheme("forest")}
            >
                <svg
                    class="icon"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                >
                    <path d="M12 2L2 7l10 5 10-5-10-5z"></path>
                    <path d="M2 17l10 5 10-5"></path>
                    <path d="M2 12l10 5 10-5"></path>
                </svg>
                <span>Forest</span>
            </button>
            <button
                class="theme-option"
                class:active={$themeStore === "minimal"}
                onclick={() => selectTheme("minimal")}
            >
                <svg
                    class="icon"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                >
                    <rect x="3" y="3" width="18" height="18" rx="2"></rect>
                    <line x1="3" y1="9" x2="21" y2="9"></line>
                    <line x1="9" y1="21" x2="9" y2="9"></line>
                </svg>
                <span>Soft Minimal</span>
            </button>
            <button
                class="theme-option"
                class:active={$themeStore === "anime"}
                onclick={() => selectTheme("anime")}
            >
                <svg
                    class="icon"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                >
                    <polygon points="12,2 15.09,8.26 22,9.27 17,14.14 18.18,21.02 12,17.77 5.82,21.02 7,14.14 2,9.27 8.91,8.26"></polygon>
                    <circle cx="12" cy="12" r="3" fill="currentColor"></circle>
                </svg>
                <span>Anime</span>
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
