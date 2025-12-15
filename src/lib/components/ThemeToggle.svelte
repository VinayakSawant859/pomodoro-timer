<script lang="ts">
    import { theme } from "$lib/state.svelte";
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
        theme.toggle();
        const themeNames: Record<string, string> = {
            light: "Light",
            dark: "Dark",
            academia: "Dark Academia",
            sakura: "Sakura",
            coffee: "Coffee",
            forest: "Forest",
            flame: "Flame",
            anime: "Anime",
        };
        const themeName = themeNames[theme.current] || theme.current;
        toastStore.show(`Theme changed to ${themeName}`, "success");
    }

    function selectTheme(
        selectedTheme:
            | "light"
            | "dark"
            | "academia"
            | "sakura"
            | "coffee"
            | "forest"
            | "flame"
            | "anime",
    ) {
        theme.set(selectedTheme);
        const themeNames: Record<string, string> = {
            light: "Light",
            dark: "Dark",
            academia: "Dark Academia",
            sakura: "Sakura",
            coffee: "Coffee",
            forest: "Forest",
            flame: "Flame",
            anime: "Anime",
        };
        const themeName = themeNames[selectedTheme] || selectedTheme;
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
        {#if theme.current === "light"}
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
        {:else if theme.current === "dark"}
            <svg
                class="icon"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
            >
                <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"
                ></path>
            </svg>
        {:else if theme.current === "academia"}
            <svg
                class="icon"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
            >
                <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"></path>
                <path
                    d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"
                ></path>
                <path d="M12 6v8"></path>
                <path d="M9 9h6"></path>
            </svg>
        {:else if theme.current === "sakura"}
            <svg class="icon" viewBox="0 -960 960 960" fill="currentColor">
                <path
                    d="M216-176q-45-45-70.5-104T120-402q0-63 24-124.5T222-642q35-35 86.5-60t122-39.5Q501-756 591.5-759t202.5 7q8 106 5 195t-16.5 160.5q-13.5 71.5-38 125T684-182q-53 53-112.5 77.5T450-80q-65 0-127-25.5T216-176Zm112-16q29 17 59.5 24.5T450-160q46 0 91-18.5t86-59.5q18-18 36.5-50.5t32-85Q709-426 716-500.5t2-177.5q-49-2-110.5-1.5T485-670q-61 9-116 29t-90 55q-45 45-62 89t-17 85q0 59 22.5 103.5T262-246q42-80 111-153.5T534-520q-72 63-125.5 142.5T328-192Zm0 0Zm0 0Z"
                />
            </svg>
        {:else if theme.current === "coffee"}
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
        {:else if theme.current === "forest"}
            <svg class="icon" viewBox="0 -960 960 960" fill="currentColor">
                <path
                    d="M200-80v-80h240v-160h-80q-83 0-141.5-58.5T160-520q0-60 33-110.5t89-73.5q9-75 65.5-125.5T480-880q76 0 132.5 50.5T678-704q56 23 89 73.5T800-520q0 83-58.5 141.5T600-320h-80v160h240v80H200Zm160-320h240q50 0 85-35t35-85q0-36-20.5-66T646-630l-42-18-6-46q-6-45-39.5-75.5T480-800q-45 0-78.5 30.5T362-694l-6 46-42 18q-33 14-53.5 44T240-520q0 50 35 85t85 35Zm120-200Z"
                />
            </svg>
        {:else if theme.current === "flame"}
            <svg
                class="icon"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
            >
                <path
                    d="M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z"
                ></path>
            </svg>
        {:else if theme.current === "anime"}
            <svg class="icon" viewBox="0 -960 960 960" fill="currentColor">
                <path
                    d="m440-803-83 83H240v117l-83 83 83 83v117h117l83 83 100-100 168 85-86-167 101-101-83-83v-117H523l-83-83Zm0-113 116 116h164v164l116 116-116 116 115 226q7 13 4 25.5T828-132q-8 8-20.5 11t-25.5-4L556-240 440-124 324-240H160v-164L44-520l116-116v-164h164l116-116Zm0 396Z"
                />
            </svg>
        {/if}
    </button>

    {#if showDropdown}
        <div class="theme-dropdown">
            <button
                class="theme-option"
                class:active={theme.current === "light"}
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
                class:active={theme.current === "dark"}
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
                class:active={theme.current === "academia"}
                onclick={() => selectTheme("academia")}
            >
                <svg
                    class="icon"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                >
                    <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"></path>
                    <path
                        d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"
                    ></path>
                    <path d="M12 6v8"></path>
                    <path d="M9 9h6"></path>
                </svg>
                <span>Academia</span>
            </button>
            <button
                class="theme-option"
                class:active={theme.current === "sakura"}
                onclick={() => selectTheme("sakura")}
            >
                <svg class="icon" viewBox="0 -960 960 960" fill="currentColor">
                    <path
                        d="M216-176q-45-45-70.5-104T120-402q0-63 24-124.5T222-642q35-35 86.5-60t122-39.5Q501-756 591.5-759t202.5 7q8 106 5 195t-16.5 160.5q-13.5 71.5-38 125T684-182q-53 53-112.5 77.5T450-80q-65 0-127-25.5T216-176Zm112-16q29 17 59.5 24.5T450-160q46 0 91-18.5t86-59.5q18-18 36.5-50.5t32-85Q709-426 716-500.5t2-177.5q-49-2-110.5-1.5T485-670q-61 9-116 29t-90 55q-45 45-62 89t-17 85q0 59 22.5 103.5T262-246q42-80 111-153.5T534-520q-72 63-125.5 142.5T328-192Zm0 0Zm0 0Z"
                    />
                </svg>
                <span>Sakura</span>
            </button>
            <button
                class="theme-option"
                class:active={theme.current === "coffee"}
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
                <span>Coffee</span>
            </button>
            <button
                class="theme-option"
                class:active={theme.current === "forest"}
                onclick={() => selectTheme("forest")}
            >
                <svg class="icon" viewBox="0 -960 960 960" fill="currentColor">
                    <path
                        d="M200-80v-80h240v-160h-80q-83 0-141.5-58.5T160-520q0-60 33-110.5t89-73.5q9-75 65.5-125.5T480-880q76 0 132.5 50.5T678-704q56 23 89 73.5T800-520q0 83-58.5 141.5T600-320h-80v160h240v80H200Zm160-320h240q50 0 85-35t35-85q0-36-20.5-66T646-630l-42-18-6-46q-6-45-39.5-75.5T480-800q-45 0-78.5 30.5T362-694l-6 46-42 18q-33 14-53.5 44T240-520q0 50 35 85t85 35Zm120-200Z"
                    />
                </svg>
                <span>Forest</span>
            </button>
            <button
                class="theme-option"
                class:active={theme.current === "flame"}
                onclick={() => selectTheme("flame")}
            >
                <svg
                    class="icon"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                >
                    <path
                        d="M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z"
                    ></path>
                </svg>
                <span>Flame</span>
            </button>
            <button
                class="theme-option"
                class:active={theme.current === "anime"}
                onclick={() => selectTheme("anime")}
            >
                <svg class="icon" viewBox="0 -960 960 960" fill="currentColor">
                    <path
                        d="m440-803-83 83H240v117l-83 83 83 83v117h117l83 83 100-100 168 85-86-167 101-101-83-83v-117H523l-83-83Zm0-113 116 116h164v164l116 116-116 116 115 226q7 13 4 25.5T828-132q-8 8-20.5 11t-25.5-4L556-240 440-124 324-240H160v-164L44-520l116-116v-164h164l116-116Zm0 396Z"
                    />
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
        flex-shrink: 0;
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
        flex-shrink: 0;
    }

    .theme-option span {
        flex: 1;
    }
</style>
