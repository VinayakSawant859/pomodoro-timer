<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { timer, audio } from "$lib/state.svelte";
    import { updateStatus, showNotification, formatTimerTitle } from "$lib/native";

    let interval: number;

    // Session presets
    const sessionPresets = [
        { name: "Short Session", work: 25, break: 5 },
        { name: "Long Session", work: 45, break: 15 },
        { name: "Ultra Focus", work: 90, break: 20 },
    ];

    let selectedPreset = $state(sessionPresets[0]);
    let customWork = $state(25);
    let customBreak = $state(5);

    // No props needed

    // UI state management using $derived with new state
    const isTimerRunning = $derived(timer.isRunning || timer.isPaused);
    const showSessionControls = $derived(!isTimerRunning);

    const timeDisplay = $derived(formatTime(timer.timeRemaining));
    const sessionTypeDisplay = $derived(
        timer.currentSession.type === "work" ? "Work Time" : "Break Time",
    );
    const progressPercentage = $derived(
        ((timer.currentSession.duration * 60 - timer.timeRemaining) /
            (timer.currentSession.duration * 60)) *
            100,
    );

    function formatTime(seconds: number): string {
        const mins = Math.floor(seconds / 60);
        const secs = seconds % 60;
        return `${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
    }

    function getTodayDate(): string {
        const today = new Date();
        const day = String(today.getDate()).padStart(2, "0");
        const month = String(today.getMonth() + 1).padStart(2, "0");
        return `${day}/${month}`;
    }

    async function startTimer() {
        if (!timer.isRunning) {
            await timer.start();
            // The $effect block will handle starting the interval
        }
    }

    async function pauseTimer() {
        if (timer.isRunning && !timer.isPaused) {
            await timer.stop();
            audio.playStop();
            clearInterval(interval);
        }
    }

    function resetTimer() {
        timer.reset();
        timer.setSession("work", selectedPreset.work);
        clearInterval(interval);
    }

    function applyPreset(preset: (typeof sessionPresets)[0]) {
        selectedPreset = preset;
        timer.setSession("work", preset.work);
    }

    function applyCustomSession() {
        timer.setSession("work", customWork);
    }

    onMount(() => {
        timer.setSession("work", selectedPreset.work);
    });

    onDestroy(() => {
        if (interval) {
            clearInterval(interval);
        }
    });

    // Watch for timer state changes and start interval if timer is running
    $effect(() => {
        if (timer.isRunning && !interval) {
            console.log("Timer is running, starting interval...");
            // Play appropriate start sound based on session type
            if (
                timer.currentSession.type === "break" &&
                timer.currentSession.duration === 5
            ) {
                audio.playBreakStart();
            } else {
                audio.playStart();
            }
            interval = setInterval(async () => {
                timer.tick();

                // Update window title and tray tooltip every second
                const mins = Math.floor(timer.timeRemaining / 60);
                const secs = timer.timeRemaining % 60;
                const sessionType = timer.currentSession.type === "work" ? "Focus" : "Break";
                const titleText = formatTimerTitle(mins, secs, sessionType);
                await updateStatus(titleText);

                // Check if session is complete
                if (timer.timeRemaining <= 0) {
                    clearInterval(interval);
                    interval = undefined as any;

                    // Show native notification
                    const notifTitle = timer.currentSession.type === "work" 
                        ? "🎉 Work Session Complete!" 
                        : "✨ Break Complete!";
                    const notifBody = timer.currentSession.type === "work"
                        ? "Great work! Time for a well-deserved break."
                        : "Break's over. Ready to focus again?";
                    await showNotification(notifTitle, notifBody);

                    // Play appropriate complete sound based on session type
                    if (
                        timer.currentSession.type === "break" &&
                        timer.currentSession.duration === 5
                    ) {
                        audio.playBreakComplete();
                    } else {
                        audio.playComplete();
                    }
                    await timer.completeSession(false); // false = not interrupted

                    // Reset title to default
                    await updateStatus("Pomodoro Timer");
                }
            }, 1000);
        } else if (!timer.isRunning && interval) {
            console.log("Timer stopped, clearing interval...");
            clearInterval(interval);
            interval = undefined as any;
            // Reset title when timer stops
            updateStatus("Pomodoro Timer");
        }
    });
</script>

<div class="timer-container">
    <svg width="0" height="0">
        <defs>
            <linearGradient id="gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                <stop offset="0%" stop-color="var(--primary-color)" />
                <stop offset="100%" stop-color="var(--primary-light)" />
            </linearGradient>
        </defs>
    </svg>
    <div class="timer-display">
        <div class="circular-progress">
            <svg class="progress-ring" width="280" height="280">
                <circle
                    class="progress-ring-background"
                    cx="140"
                    cy="140"
                    r="120"
                    fill="transparent"
                    stroke-width="8"
                />
                <circle
                    class="progress-ring-progress"
                    cx="140"
                    cy="140"
                    r="120"
                    fill="transparent"
                    stroke-width="8"
                    stroke-dasharray={2 * Math.PI * 120}
                    stroke-dashoffset={2 *
                        Math.PI *
                        120 *
                        (1 - progressPercentage / 100)}
                    transform="rotate(-90 140 140)"
                />
            </svg>
            <div class="timer-content">
                <div class="session-type">{sessionTypeDisplay}</div>
                <div class="time">{timeDisplay}</div>
                <div class="session-info">
                    Session {timer.sessionNumber} • {timer.currentSession
                        .duration}min {timer.currentSession.type === "work"
                        ? "work"
                        : "break"}
                </div>
                <div class="session-date">({getTodayDate()})</div>
            </div>
        </div>
    </div>

    <div class="timer-controls">
        {#if !timer.isRunning}
            <button class="btn btn-primary" onclick={startTimer}>
                <svg
                    class="icon"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                >
                    <polygon points="5,3 19,12 5,21 5,3"></polygon>
                </svg>
                Start
            </button>
        {:else}
            <button class="btn btn-primary" onclick={pauseTimer}>
                <svg
                    class="icon"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                >
                    <rect x="3" y="3" width="18" height="18" rx="2" ry="2"
                    ></rect>
                </svg>
                Stop
            </button>
        {/if}

        <button class="btn btn-outline" onclick={resetTimer}>
            <svg
                class="icon"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
            >
                <polyline points="23,4 23,10 17,10"></polyline>
                <path d="M20.49,15a9,9,0,1,1-2.12-9.36L23,10"></path>
            </svg>
            Reset
        </button>
    </div>

    {#if !isTimerRunning}
        <div class="session-presets">
            <h3>Session Presets</h3>
            <div class="presets-grid">
                {#each sessionPresets as preset}
                    <button
                        class="preset-btn"
                        class:active={selectedPreset === preset}
                        onclick={() => applyPreset(preset)}
                    >
                        <div class="preset-name">{preset.name}</div>
                        <div class="preset-times">
                            {preset.work}m work / {preset.break}m break
                        </div>
                    </button>
                {/each}
            </div>

            <div class="custom-session">
                <h4>✨ Custom Session</h4>
                <div class="custom-card">
                    <div class="custom-inputs">
                        <div class="input-group">
                            <label for="custom-work">🏢 Work</label>
                            <div class="input-wrapper">
                                <input
                                    id="custom-work"
                                    type="number"
                                    bind:value={customWork}
                                    min="1"
                                    max="120"
                                    class="custom-input"
                                />
                                <span class="input-unit">min</span>
                            </div>
                        </div>
                        <div class="input-separator">➜</div>
                        <div class="input-group">
                            <label for="custom-break">☕ Break</label>
                            <div class="input-wrapper">
                                <input
                                    id="custom-break"
                                    type="number"
                                    bind:value={customBreak}
                                    min="1"
                                    max="60"
                                    class="custom-input"
                                />
                                <span class="input-unit">min</span>
                            </div>
                        </div>
                    </div>
                    <button class="btn-apply" onclick={applyCustomSession}>
                        <svg
                            class="icon"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                        >
                            <polyline points="20,6 9,17 4,12"></polyline>
                        </svg>
                        Apply Custom Session
                    </button>
                </div>
            </div>
        </div>
    {/if}
</div>

<style>
    .timer-container {
        background: var(--surface-color);
        border-radius: 1rem;
        padding: 2rem;
        box-shadow: 0 4px 6px -1px var(--shadow);
        border: 1px solid var(--border-color);
    }

    .timer-display {
        display: flex;
        justify-content: center;
        margin-bottom: 2rem;
    }

    .circular-progress {
        position: relative;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .progress-ring {
        transform: rotate(-90deg);
    }

    .progress-ring-background {
        stroke: var(--border-color);
        opacity: 0.3;
    }

    .progress-ring-progress {
        stroke: url(#gradient);
        stroke-linecap: round;
        transition: stroke-dashoffset 0.3s ease;
    }

    .timer-content {
        position: absolute;
        text-align: center;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
    }

    .session-type {
        font-size: 1.1rem;
        font-weight: 500;
        color: var(--text-secondary);
        margin-bottom: 0.5rem;
    }

    .time {
        font-size: 3.5rem;
        font-weight: 700;
        color: var(--primary-color);
        margin-bottom: 0.5rem;
        font-variant-numeric: tabular-nums;
        line-height: 1;
        font-family: "Courier New", monospace;
    }

    .session-info {
        font-size: 0.85rem;
        color: var(--text-secondary);
        font-weight: 500;
    }

    .session-date {
        font-size: 0.8rem;
        color: var(--text-secondary);
        font-weight: 400;
        margin-top: 0.25rem;
    }

    .timer-controls {
        display: flex;
        gap: 1rem;
        justify-content: center;
        margin-bottom: 2rem;
    }

    .session-presets h3 {
        margin-bottom: 1rem;
        color: var(--text-color);
    }

    .presets-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
        gap: 1rem;
        margin-bottom: 1.5rem;
    }

    .preset-btn {
        background: var(--surface-color);
        border: 2px solid var(--border-color);
        border-radius: 0.75rem;
        padding: 1.25rem;
        cursor: pointer;
        transition: all 0.2s ease;
        color: var(--text-color);
        text-align: left;
        width: 100%;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }

    .preset-btn:hover {
        border-color: var(--primary-color);
        transform: translateY(-2px);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
        background: var(--background-color);
    }

    .preset-btn.active {
        border-color: var(--primary-color);
        background: linear-gradient(
            135deg,
            var(--primary-color),
            var(--primary-light)
        );
        color: white;
        box-shadow: 0 4px 16px rgba(99, 102, 241, 0.3);
    }

    .preset-btn.active .preset-name,
    .preset-btn.active .preset-times {
        color: white;
    }

    .preset-btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
        transform: none;
    }

    .preset-name {
        font-weight: 600;
        margin-bottom: 0.5rem;
        color: inherit;
        font-size: 1rem;
    }

    .preset-times {
        font-size: 0.875rem;
        opacity: 0.8;
        color: inherit;
        font-weight: 500;
    }

    .custom-session h4 {
        margin-bottom: 1rem;
        color: var(--text-color);
        font-weight: 600;
        font-size: 1.1rem;
    }

    .custom-card {
        background: var(--surface-color);
        border: 2px solid var(--border-color);
        border-radius: 0.75rem;
        padding: 1.5rem;
        position: relative;
        overflow: hidden;
    }

    .custom-inputs {
        display: flex;
        gap: 1rem;
        align-items: center;
        margin-bottom: 1.5rem;
        flex-wrap: wrap;
    }

    .input-separator {
        font-size: 1.5rem;
        color: var(--primary-color);
        font-weight: bold;
    }

    .input-group {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        flex: 1;
        min-width: 120px;
    }

    .input-group label {
        font-size: 0.9rem;
        font-weight: 600;
        color: var(--text-color);
    }

    .input-wrapper {
        position: relative;
        display: flex;
        align-items: center;
    }

    .custom-input {
        width: 100%;
        padding: 0.75rem 3rem 0.75rem 1rem;
        border: 2px solid var(--border-color);
        border-radius: 0.75rem;
        background: var(--surface-color);
        color: var(--text-color);
        font-size: 1.1rem;
        font-weight: 600;
        text-align: center;
        transition: all 0.2s ease;
    }

    .custom-input:focus {
        outline: none;
        border-color: var(--primary-color);
        box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
    }

    .input-unit {
        position: absolute;
        right: 1rem;
        font-size: 0.85rem;
        color: var(--text-secondary);
        font-weight: 500;
        pointer-events: none;
    }

    .btn-apply {
        background: var(--primary-color);
        color: white;
        border: none;
        padding: 0.75rem 1.5rem;
        border-radius: 0.5rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.2s ease;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        font-size: 0.95rem;
    }

    .btn-apply:hover:not(:disabled) {
        background: var(--primary-dark);
        transform: translateY(-2px);
    }

    .btn-apply:disabled {
        opacity: 0.6;
        cursor: not-allowed;
        transform: none;
    }

    .btn-apply .icon {
        width: 1.2rem;
        height: 1.2rem;
    }

    .btn {
        display: inline-flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.75rem 1.5rem;
        border: none;
        border-radius: 0.5rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s ease;
        text-decoration: none;
    }

    .btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .btn-primary {
        background: var(--primary-color);
        color: white;
    }

    .btn-primary:hover:not(:disabled) {
        background: var(--primary-dark);
    }

    .btn-outline {
        background: transparent;
        color: var(--text-color);
        border: 1px solid var(--border-color);
    }

    .btn-outline:hover:not(:disabled) {
        background: var(--surface-color);
    }

    .icon {
        width: 1rem;
        height: 1rem;
    }

    @media (max-width: 768px) {
        .timer-container {
            padding: 1.5rem;
        }

        .time {
            font-size: 3rem;
        }

        .timer-controls {
            flex-wrap: wrap;
            gap: 0.75rem;
        }

        .custom-inputs {
            flex-direction: column;
            align-items: stretch;
        }

        .input-group input {
            width: 100%;
        }
    }
</style>
