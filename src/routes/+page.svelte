<script lang="ts">
    import { onMount } from "svelte";
    import {
        timer,
        tasks,
        theme,
        font,
        sessionHistory,
    } from "$lib/state.svelte";
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
        theme.init();
        font.init();

        // Load tasks from database
        try {
            await tasks.load();
        } catch (error) {
            console.error("Failed to load tasks:", error);
        }

        // Load today's session history and initialize timer state
        try {
            const todayHistory = await sessionHistory.loadToday();

            // Update timer with today's session count
            if (todayHistory) {
                timer.dailySessionCount =
                    todayHistory.total_work_sessions +
                    todayHistory.total_break_sessions;
                timer.sessionNumber = todayHistory.total_work_sessions + 1;
            }
        } catch (error) {
            console.error("Failed to load session history:", error);
        }
    });
</script>

<svelte:head>
    <title>Pomodoro Timer</title>
</svelte:head>

<main class="app" data-theme={theme.current} data-font={font.current}>
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
        --primary-color: #6c7cff;
        --primary-light: #8b98ff;
        --primary-dark: #5565e8;
        --success-color: #10b981;
        --warning-color: #f59e0b;
        --error-color: #ef4444;

        /* Light theme - Modern & Premium */
        --background-color: #f6f7fb;
        --surface-color: #ffffff;
        --text-color: #1e2433;
        --text-secondary: #5f677a;
        --border-color: #e3e6ef;
        --shadow: rgba(30, 36, 51, 0.08);
    }

    :global([data-theme="dark"]) {
        /* Dark theme - Modern & Premium */
        --primary-color: #6c7cff;
        --primary-light: #8b98ff;
        --primary-dark: #5565e8;

        --background-color: #0e1320;
        --surface-color: #1a2133;
        --text-color: #e6e9f2;
        --text-secondary: #a8b0c3;
        --border-color: #2a3347;
        --shadow: rgba(0, 0, 0, 0.4);
    }

    :global([data-theme="academia"]) {
        /* Dark Academia Theme */
        --primary-color: #3a4f7a;
        --primary-light: #5c7aa8;
        --primary-dark: #2a3857;
        --success-color: #c9a24d;
        --warning-color: #5c3d2e;
        --error-color: #8b4513;

        --background-color: #1e1b18;
        --surface-color: rgba(42, 38, 33, 0.7);
        --text-color: #ede6d8;
        --text-secondary: #b6b0a4;
        --border-color: #5c3d2e;
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
        --surface-color: rgba(255, 233, 230, 0.6);
        --text-color: #4a1f2f;
        --text-secondary: #7a5563;
        --border-color: rgba(195, 130, 158, 0.25);
        --shadow: rgba(74, 31, 47, 0.08);
    }

    :global([data-theme="coffee"]) {
        /* Coffee Theme - Cozy & Aesthetic */
        /* Semantic Roles:
           - primary: Coffee brown for timer, buttons, progress
           - background: Warm cream
           - surface: Soft latte for cards
           - text-primary: Deep espresso for readability
           - text-secondary: Warm brown for hierarchy
           - accent: Caramel highlights
        */
        --primary-color: #6d4c41;
        --primary-light: #8d6e63;
        --primary-dark: #4e342e;
        --success-color: #6d4c41;
        --warning-color: #a1887f;
        --error-color: #4e342e;

        --background-color: #f5efe7;
        --surface-color: rgba(161, 136, 127, 0.25);
        --text-color: #3e2723;
        --text-secondary: #7b5e57;
        --border-color: rgba(109, 76, 65, 0.25);
        --shadow: rgba(62, 39, 35, 0.08);
    }

    :global([data-theme="forest"]) {
        /* Forest Theme - Botanical Canopy Aesthetic */
        /* Semantic Roles:
           - primary: Desaturated moss accent (≤8% usage on interactive elements)
           - background: Deep earth/bark base for grounded depth
           - surface: Natural bark tone for primary cards
           - secondary-surface: Warm earth for subtle elevation
           - light-surface: Paper tone for inputs/elevated elements
           - text-primary: Warm off-white for readability
           - text-secondary: Muted stone for hierarchy
           - text-on-light: Deep charcoal for light backgrounds
        */
        --primary-color: #6a7b5e;
        --primary-light: #7f9270;
        --primary-dark: #5a6b50;
        --success-color: #6a7b5e;
        --warning-color: #7a6f5e;
        --error-color: #5a3b2e;

        --background-color: #332c0f;
        --surface-color: #474329;
        --text-color: #f4eeeb;
        --text-secondary: #cfc8bc;
        --border-color: #5a5340;
        --shadow: rgba(26, 22, 8, 0.6);
    }

    :global([data-theme="flame"]) {
        /* Flame Theme - Dark Fire Aesthetic */
        /* Semantic Roles:
           - primary: Orange flame for timer, buttons, progress with glow
           - background: Deep charcoal-black for dramatic depth
           - surface: Dark elevated surface with transparency
           - text-primary: Warm off-white for readability
           - text-secondary: Muted warm gray for hierarchy
           - flame-accent: Bright orange for gradients and glow
           - ember-accent: Deep red-orange for depth (rare use)
        */
        --primary-color: #ff8c1a;
        --primary-light: #ffb347;
        --primary-dark: #c24a00;
        --success-color: #ff8c1a;
        --warning-color: #ffb347;
        --error-color: #7a1e0e;

        --background-color: #0a0705;
        --surface-color: rgba(28, 22, 15, 0.7);
        --text-color: #f5f1eb;
        --text-secondary: #b8afa3;
        --border-color: rgba(255, 140, 26, 0.15);
        --shadow: rgba(255, 140, 26, 0.2);
    }

    :global([data-theme="anime"]) {
        /* Anime Theme - Dark Purple with Neon Accents */
        /* Semantic Roles:
           - primary: Purple for timer, buttons, progress with glow
           - background: Deep purple-black for night aesthetic
           - surface: Dark purple for cards with transparency
           - text-primary: Light lavender for excellent readability
           - text-secondary: Muted purple for hierarchy
           - neon-accent: Bright purple glow for highlights
           - cyan-accent: Subtle cyan for special emphasis (minimal use)
        */
        --primary-color: #7c5cff;
        --primary-light: #9b7fff;
        --primary-dark: #6344e6;
        --success-color: #7c5cff;
        --warning-color: #b46cff;
        --error-color: #ff6b9d;

        --background-color: #0b0a12;
        --surface-color: rgba(22, 20, 39, 0.6);
        --text-color: #eae7ff;
        --text-secondary: #a39fcb;
        --border-color: rgba(124, 92, 255, 0.2);
        --shadow: rgba(124, 92, 255, 0.15);
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
