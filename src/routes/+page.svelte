<script lang="ts">
    import { onMount } from "svelte";
    import { timerStore, taskStore, themeStore } from "$lib/stores";
    import Timer from "$lib/components/Timer.svelte";
    import TaskManager from "$lib/components/TaskManager.svelte";
    import ThemeToggle from "$lib/components/ThemeToggle.svelte";

    let showTasks = $state(false);

    onMount(async () => {
        // Initialize theme from localStorage
        themeStore.init();

        // Load tasks from database
        try {
            await taskStore.load();
        } catch (error) {
            console.error("Failed to load tasks:", error);
        }
    });
</script>

<svelte:head>
    <title>Pomodoro Timer</title>
</svelte:head>

<main class="app" data-theme={$themeStore}>
    <header class="header">
        <div class="header-left">
            <h1>Pomodoro Timer <span class="author">by vinayak</span></h1>
            <a
                href="https://github.com/VinayakSawant859"
                target="_blank"
                rel="noopener"
                class="github-link"
            >
                <svg
                    class="github-icon"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                >
                    <path
                        d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"
                    />
                </svg>
                GitHub
            </a>
        </div>
        <div class="header-controls">
            <button
                class="toggle-btn task-toggle"
                onclick={() => (showTasks = !showTasks)}
                title={showTasks ? "Show Timer" : "Show Tasks"}
            >
                {#if showTasks}
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                        <circle cx="12" cy="12" r="10"></circle>
                        <polyline points="12,6 12,12 16,14"></polyline>
                    </svg>
                {:else}
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                        <rect x="3" y="4" width="18" height="18" rx="2" ry="2"
                        ></rect>
                        <line x1="16" y1="2" x2="16" y2="6"></line>
                        <line x1="8" y1="2" x2="8" y2="6"></line>
                        <line x1="3" y1="10" x2="21" y2="10"></line>
                    </svg>
                {/if}
            </button>
            <ThemeToggle />
        </div>
    </header>

    <div class="content">
        <div class="flip-card" class:flipped={showTasks}>
            <div class="flip-card-inner">
                <div class="flip-card-front">
                    <Timer />
                </div>
                <div class="flip-card-back">
                    <TaskManager />
                </div>
            </div>
        </div>
    </div>
</main>

<style>
    :global(:root) {
        --primary-color: #6366f1;
        --primary-light: #818cf8;
        --primary-dark: #4f46e5;
        --success-color: #10b981;
        --warning-color: #f59e0b;
        --error-color: #ef4444;

        /* Light theme */
        --background-color: #ffffff;
        --surface-color: #f8fafc;
        --text-color: #1f2937;
        --text-secondary: #6b7280;
        --border-color: #e5e7eb;
        --shadow: rgba(0, 0, 0, 0.1);
    }

    :global([data-theme="dark"]) {
        --background-color: #1f2937;
        --surface-color: #374151;
        --text-color: #f9fafb;
        --text-secondary: #d1d5db;
        --border-color: #4b5563;
        --shadow: rgba(0, 0, 0, 0.3);
    }

    :global(*) {
        margin: 0;
        padding: 0;
        box-sizing: border-box;
    }

    :global(body) {
        font-family:
            -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        background-color: var(--background-color);
        color: var(--text-color);
        transition:
            background-color 0.2s ease,
            color 0.2s ease;
    }

    .app {
        min-height: 100vh;
        display: flex;
        flex-direction: column;
        max-width: 1200px;
        margin: 0 auto;
        padding: 0 1rem;
    }

    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 2rem 0;
        border-bottom: 1px solid var(--border-color);
    }

    .header-controls {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .toggle-btn {
        width: 2.5rem;
        height: 2.5rem;
        border-radius: 0.5rem;
        border: 1px solid var(--border-color);
        background: var(--surface-color);
        color: var(--text-color);
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: all 0.2s ease;
    }

    .toggle-btn:hover {
        background: var(--primary-color);
        color: white;
        border-color: var(--primary-color);
    }

    .toggle-btn svg {
        width: 1.25rem;
        height: 1.25rem;
    }

    .header-left {
        display: flex;
        align-items: center;
        gap: 1.5rem;
    }

    .header h1 {
        font-size: 2rem;
        font-weight: 600;
        color: var(--primary-color);
        display: flex;
        align-items: baseline;
        gap: 0.5rem;
    }

    .author {
        font-size: 0.9rem;
        font-weight: 400;
        color: var(--text-secondary);
        font-style: italic;
    }

    .github-link {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.5rem 1rem;
        background: var(--surface-color);
        border: 1px solid var(--border-color);
        border-radius: 0.5rem;
        color: var(--text-color);
        text-decoration: none;
        transition: all 0.2s ease;
        font-size: 0.9rem;
    }

    .github-link:hover {
        background: var(--primary-color);
        color: white;
        border-color: var(--primary-color);
    }

    .github-icon {
        width: 1.2rem;
        height: 1.2rem;
    }

    .content {
        flex: 1;
        display: flex;
        justify-content: center;
        align-items: flex-start;
        padding: 2rem 0;
    }

    .flip-card {
        background-color: transparent;
        width: 100%;
        max-width: 800px;
        height: 600px;
        perspective: 1000px;
    }

    .flip-card-inner {
        position: relative;
        width: 100%;
        height: 100%;
        text-align: center;
        transition: transform 0.6s;
        transform-style: preserve-3d;
    }

    .flip-card.flipped .flip-card-inner {
        transform: rotateY(180deg);
    }

    .flip-card-front,
    .flip-card-back {
        position: absolute;
        width: 100%;
        height: 100%;
        -webkit-backface-visibility: hidden;
        backface-visibility: hidden;
    }

    .flip-card-back {
        transform: rotateY(180deg);
    }

    @media (max-width: 768px) {
        .content {
            flex-direction: column;
            gap: 1.5rem;
        }

        .content > :global(*:last-child) {
            width: 100%;
        }
    }
</style>
