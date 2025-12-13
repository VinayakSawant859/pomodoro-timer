<script lang="ts">
    import { onMount } from "svelte";
    import {
        timerStore,
        taskStore,
        themeStore,
        fontStore,
        sessionHistoryStore,
    } from "$lib/stores";
    import Timer from "$lib/components/Timer.svelte";
    import TaskManager from "$lib/components/TaskManager.svelte";
    import SessionProgress from "$lib/components/SessionProgress.svelte";
    import ThemeToggle from "$lib/components/ThemeToggle.svelte";
    import FontToggle from "$lib/components/FontToggle.svelte";
    import Statistics from "$lib/components/Statistics.svelte";
    import Toast from "$lib/components/Toast.svelte";

    let showTasks = $state(false);
    let showStatistics = $state(false);

    onMount(async () => {
        // Initialize theme and font from localStorage
        themeStore.init();
        fontStore.init();

        // Load tasks from database
        try {
            await taskStore.load();
        } catch (error) {
            console.error("Failed to load tasks:", error);
        }

        // Load today's session history and initialize timer state
        try {
            const todayHistory = await sessionHistoryStore.loadToday();

            // Update timer store with today's session count
            if (todayHistory) {
                timerStore.update((state) => ({
                    ...state,
                    dailySessionCount:
                        todayHistory.total_work_sessions +
                        todayHistory.total_break_sessions,
                    sessionNumber: todayHistory.total_work_sessions + 1,
                }));
            }
        } catch (error) {
            console.error("Failed to load session history:", error);
        }
    });
</script>

<svelte:head>
    <title>Pomodoro Timer</title>
</svelte:head>

<main class="app" data-theme={$themeStore} data-font={$fontStore}>
    <header class="header">
        <div class="header-left">
            <h1>Pomodoro Timer <span class="author">by vinayak</span></h1>
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
            <button
                class="toggle-btn stats-toggle"
                onclick={() => (showStatistics = true)}
                title="View Statistics"
            >
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                    <line x1="12" y1="20" x2="12" y2="10"></line>
                    <line x1="18" y1="20" x2="18" y2="4"></line>
                    <line x1="6" y1="20" x2="6" y2="16"></line>
                </svg>
            </button>
            <FontToggle />
            <ThemeToggle />
        </div>
    </header>

    <div class="content">
        <div class="main-content">
            <div class="flip-card" class:flipped={showTasks}>
                <div class="flip-card-inner">
                    <div class="flip-card-front">
                        <Timer />
                    </div>
                    <div class="flip-card-back">
                        <TaskManager onStartTask={() => (showTasks = false)} />
                    </div>
                </div>
            </div>
            {#if !showTasks}
                <div class="session-progress-container">
                    <SessionProgress />
                </div>
            {/if}
        </div>
    </div>

    {#if showStatistics}
        <Statistics onClose={() => (showStatistics = false)} />
    {/if}

    <Toast />
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

    :global([data-theme="academia"]) {
        /* Dark Academia Theme */
        --primary-color: #3A4F7A;
        --primary-light: #5C7AA8;
        --primary-dark: #2A3857;
        --success-color: #C9A24D;
        --warning-color: #5C3D2E;
        --error-color: #8B4513;

        --background-color: #1E1B18;
        --surface-color: #2A2621;
        --text-color: #EDE6D8;
        --text-secondary: #B6B0A4;
        --border-color: #5C3D2E;
        --shadow: rgba(0, 0, 0, 0.5);
    }

    :global([data-theme="sakura"]) {
        /* Sakura Aesthetic Theme - Polished */
        /* Semantic Roles:
           - primary: Soft pink for timer, buttons, progress
           - background: Warm peachy white
           - surface: Subtle blush for cards
           - text-primary: Deep wine for readability
           - text-secondary: Muted rose for hierarchy
        */
        --primary-color: #c3829e;
        --primary-light: #d69bb3;
        --primary-dark: #a06781;
        --success-color: #c3829e;
        --warning-color: #d69bb3;
        --error-color: #8b4862;

        --background-color: #fff5f2;
        --surface-color: #ffe9e6;
        --text-color: #4a1f2f;
        --text-secondary: #7a5563;
        --border-color: rgba(195, 130, 158, 0.25);
        --shadow: rgba(74, 31, 47, 0.08);
    }

    :global([data-theme="tobacco"]) {
        /* Tobacco Warm Theme - Polished */
        /* Semantic Roles:
           - primary: Warm taupe for timer, buttons, progress
           - background: Soft cream
           - surface: Gentle sand for cards
           - text-primary: Deep espresso for readability
           - text-secondary: Medium brown for hierarchy
        */
        --primary-color: #9d8b70;
        --primary-light: #b5a38a;
        --primary-dark: #6d5d49;
        --success-color: #9d8b70;
        --warning-color: #b5a38a;
        --error-color: #5c4430;

        --background-color: #faf7f2;
        --surface-color: #f0ebe3;
        --text-color: #2a2218;
        --text-secondary: #594c3d;
        --border-color: rgba(157, 139, 112, 0.25);
        --shadow: rgba(42, 34, 24, 0.08);
    }

    :global([data-theme="forest"]) {
        /* Forest Theme - Natural & Grounded */
        /* Semantic Roles:
           - primary: Forest green for timer, buttons, progress
           - background: Natural off-white
           - surface: Soft sage for cards
           - text-primary: Deep forest for readability
           - text-secondary: Muted sage for hierarchy
           - accent: Sand tone for highlights
        */
        --primary-color: #5F8D4E;
        --primary-light: #7AA55F;
        --primary-dark: #4A6E3D;
        --success-color: #5F8D4E;
        --warning-color: #A4BE7B;
        --error-color: #4A6E3D;

        --background-color: #F3F7F4;
        --surface-color: #E5D9B6;
        --text-color: #2F3E2E;
        --text-secondary: #6B7F6A;
        --border-color: rgba(95, 141, 78, 0.25);
        --shadow: rgba(47, 62, 46, 0.08);
    }

    :global([data-theme="pastel"]) {
        /* Pastel Productivity Theme - Modern & Friendly */
        /* Semantic Roles:
           - primary: Lavender blue for timer, buttons, progress
           - background: Clean white
           - surface: Soft purple for cards
           - text-primary: Dark charcoal for readability
           - text-secondary: Medium gray for hierarchy
           - accent: Pastel pink highlights
        */
        --primary-color: #B8C0FF;
        --primary-light: #C8CEFF;
        --primary-dark: #9BA5E8;
        --success-color: #B8C0FF;
        --warning-color: #FFD6E0;
        --error-color: #CDB4DB;

        --background-color: #FAFAFA;
        --surface-color: rgba(205, 180, 219, 0.15);
        --text-color: #2B2B2B;
        --text-secondary: #7A7A7A;
        --border-color: rgba(184, 192, 255, 0.3);
        --shadow: rgba(43, 43, 43, 0.06);
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
            color 0.2s ease,
            font-family 0.2s ease;
    }

    :global([data-font="josefin"] body),
    :global([data-font="josefin"] *) {
        font-family: "Josefin Sans", sans-serif !important;
        font-optical-sizing: auto;
    }

    :global([data-font="cause"] body),
    :global([data-font="cause"] *) {
        font-family: "Cause", cursive !important;
        font-optical-sizing: auto;
    }

    :global([data-font="cabin"] body),
    :global([data-font="cabin"] *) {
        font-family: "Cabin Sketch", sans-serif !important;
    }

    :global([data-font="inconsolata"] body),
    :global([data-font="inconsolata"] *) {
        font-family: "Inconsolata", monospace !important;
        font-optical-sizing: auto;
    }

    :global([data-font="poppins"] body),
    :global([data-font="poppins"] *) {
        font-family: "Poppins", sans-serif !important;
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

    .content {
        flex: 1;
        display: flex;
        justify-content: center;
        align-items: flex-start;
        padding: 2rem 0;
    }

    .main-content {
        width: 100%;
        max-width: 800px;
        display: flex;
        flex-direction: column;
        gap: 2rem;
    }

    .flip-card {
        background-color: transparent;
        width: 100%;
        height: auto;
        perspective: 1000px;
    }

    .session-progress-container {
        width: 100%;
        margin-top: 2rem;
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
        width: 100%;
        min-height: 500px;
        -webkit-backface-visibility: hidden;
        backface-visibility: hidden;
    }

    .flip-card-front {
        position: relative;
    }

    .flip-card-back {
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        transform: rotateY(180deg);
    }

    /* Prevent interaction with hidden sides */
    .flip-card:not(.flipped) .flip-card-back {
        pointer-events: none;
    }

    .flip-card.flipped .flip-card-front {
        pointer-events: none;
    }

    @media (max-width: 768px) {
        .main-content {
            gap: 1.5rem;
        }

        .flip-card-front,
        .flip-card-back {
            min-height: 450px;
        }
    }

    @media (max-width: 480px) {
        .flip-card-front,
        .flip-card-back {
            min-height: 400px;
        }

        .header {
            flex-direction: column;
            gap: 1rem;
            align-items: flex-start;
        }

        .header-left {
            flex-direction: column;
            align-items: flex-start;
            gap: 0.75rem;
        }

        .header h1 {
            font-size: 1.5rem;
        }
    }
</style>
