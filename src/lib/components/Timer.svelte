<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { timer, audio, tasks, dailySummary } from "$lib/state.svelte";
    import {
        updateStatus,
        showNotification,
        formatTimerTitle,
    } from "$lib/native";
    import { invoke } from "@tauri-apps/api/core";
    import CompletionDialog from "./CompletionDialog.svelte";

    let interval: number;
    let ambientNoiseEnabled = $state(false);

    const sessionPresets = [
        { name: "Short Session", work: 25, break: 5 },
        { name: "Long Session", work: 45, break: 15 },
        { name: "Ultra Focus", work: 90, break: 20 },
    ];

    let selectedPreset = $state(sessionPresets[0]);
    let customWork = $state(25);
    let customBreak = $state(5);

    // Computed video URL for videostream protocol
    const zenVideoUrl = $derived(() => {
        if (!timer.zenVideoPath) return null;
        // URL encode the path for the custom protocol
        return `videostream://${encodeURIComponent(timer.zenVideoPath)}`;
    });

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

    // Active task integration
    const activeTask = $derived(
        timer.currentTaskId
            ? tasks.tasks.find((t) => t.id === timer.currentTaskId)
            : null,
    );

    // Animation states
    let isStarting = $state(false);
    let isCompleting = $state(false);

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
            isStarting = true;
            await timer.start();
            // The $effect block will handle starting the interval
            setTimeout(() => {
                isStarting = false;
            }, 600);
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

    async function toggleAmbientNoise() {
        ambientNoiseEnabled = !ambientNoiseEnabled;
        localStorage.setItem(
            "ambientNoiseEnabled",
            String(ambientNoiseEnabled),
        );

        if (ambientNoiseEnabled) {
            await startAmbientNoise();
        } else {
            await stopAmbientNoise();
        }
    }

    async function startAmbientNoise() {
        try {
            await invoke("set_white_noise", {
                soundName: "boiler-ambient-noise.mp3",
            });
        } catch (error) {
            console.error("Failed to start ambient noise:", error);
            ambientNoiseEnabled = false;
        }
    }

    async function stopAmbientNoise() {
        try {
            await invoke("set_white_noise", { soundName: null });
        } catch (error) {
            console.error("Failed to stop ambient noise:", error);
        }
    }

    onMount(() => {
        const savedAmbient = localStorage.getItem("ambientNoiseEnabled");
        if (savedAmbient === "true") {
            ambientNoiseEnabled = true;
            startAmbientNoise();
        }
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
                const sessionType =
                    timer.currentSession.type === "work" ? "Focus" : "Break";
                const titleText = formatTimerTitle(mins, secs, sessionType);
                await updateStatus(titleText);

                // Check if session is complete
                if (timer.timeRemaining <= 0) {
                    clearInterval(interval);
                    interval = undefined as any;

                    // Show native notification
                    const notifTitle =
                        timer.currentSession.type === "work"
                            ? "🎉 Work Session Complete!"
                            : "✨ Break Complete!";
                    const notifBody =
                        timer.currentSession.type === "work"
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

                    // Complete the session (this will trigger the dialog if needed)
                    await timer.completeSession(false);

                    // Check if we should show daily summary (natural end-of-day check)
                    // Only check after completing a work session
                    if (timer.currentSession.type === "break") {
                        await dailySummary.checkAndShow();
                    }

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

<!-- Zen Mode Background Video -->
{#if timer.monkMode && zenVideoUrl()}
    <video
        class="zen-background-video"
        autoplay
        loop
        muted
        playsinline
        src={zenVideoUrl()}
    ></video>
{/if}

<div class="timer-container" class:zen-mode={timer.monkMode}>
    <svg width="0" height="0">
        <defs>
            <linearGradient id="gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                <stop offset="0%" stop-color="var(--primary-color)" />
                <stop offset="100%" stop-color="var(--primary-light)" />
            </linearGradient>
        </defs>
    </svg>
    <div
        class="timer-display"
        class:starting={isStarting}
        class:completing={isCompleting}
    >
        <div
            class="circular-progress"
            data-session-type={timer.currentSession.type}
        >
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
                <div class="session-type" data-type={timer.currentSession.type}>
                    {#if timer.currentSession.type === "work"}
                        <svg
                            class="session-icon"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                        >
                            <circle cx="12" cy="12" r="10"></circle>
                            <polyline points="12,6 12,12 16,14"></polyline>
                        </svg>
                    {:else}
                        <svg
                            class="session-icon"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                        >
                            <path
                                d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"
                            ></path>
                        </svg>
                    {/if}
                    {sessionTypeDisplay}
                </div>
                <div class="time">{timeDisplay}</div>
                {#if activeTask}
                    <div class="active-task-indicator">
                        <svg
                            class="task-icon"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                        >
                            <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
                            <polyline points="22,4 12,14.01 9,11.01"></polyline>
                        </svg>
                        <span class="task-name">{activeTask.text}</span>
                    </div>
                {/if}
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

        <button
            class="btn btn-outline ambient-toggle"
            class:active={ambientNoiseEnabled}
            onclick={toggleAmbientNoise}
            title={ambientNoiseEnabled
                ? "Stop Ambient Noise - Turn off background sound"
                : "Start Ambient Noise - Play calming background sound to help focus"}
        >
            <svg
                class="icon"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
            >
                {#if ambientNoiseEnabled}
                    <circle cx="12" cy="12" r="10"></circle>
                    <path d="M8 14s1.5 2 4 2 4-2 4-2"></path>
                    <line x1="9" y1="9" x2="9.01" y2="9"></line>
                    <line x1="15" y1="9" x2="15.01" y2="9"></line>
                {:else}
                    <circle cx="12" cy="12" r="10"></circle>
                    <line x1="8" y1="15" x2="16" y2="15"></line>
                    <line x1="9" y1="9" x2="9.01" y2="9"></line>
                    <line x1="15" y1="9" x2="15.01" y2="9"></line>
                {/if}
            </svg>
        </button>

        <button
            class="btn btn-outline"
            onclick={resetTimer}
            title="Reset Timer - Start over with current session settings"
        >
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

    {#if timer.showCompletionDialog}
        <CompletionDialog
            taskId={timer.currentTaskId}
            taskName={tasks.tasks.find((t) => t.id === timer.currentTaskId)
                ?.text || "your task"}
            onClose={() => timer.closeCompletionDialog()}
        />
    {/if}

    {#if !isTimerRunning && !timer.monkMode}
        <div class="session-presets">
            <h3>Session Presets</h3>
            <div class="presets-grid">
                {#each sessionPresets as preset}
                    <button
                        class="preset-btn"
                        class:active={selectedPreset === preset}
                        onclick={() => applyPreset(preset)}
                        title={preset.name === "Short Session"
                            ? "Perfect for quick tasks and maintaining momentum"
                            : preset.name === "Long Session"
                              ? "Ideal for deep work requiring extended focus"
                              : "Maximum concentration for complex challenges"}
                    >
                        <div class="preset-name">
                            {#if selectedPreset === preset}
                                <svg
                                    class="check-icon"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="3"
                                >
                                    <polyline points="20,6 9,17 4,12"
                                    ></polyline>
                                </svg>
                            {/if}
                            {preset.name}
                        </div>
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

            <!-- Zen Mode Video Selection -->
            <div class="zen-video-section">
                <h4>🎬 Zen Mode Background</h4>
                <div class="zen-video-controls">
                    {#if timer.zenVideoPath}
                        <div class="video-info">
                            <svg
                                class="icon"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                            >
                                <path
                                    d="M23 7l-7 5 7 5V7z M1 5h14v14H1z"
                                ></path>
                            </svg>
                            <span class="video-name"
                                >{timer.zenVideoPath.split(/[/\\]/).pop()}</span
                            >
                        </div>
                        <div class="video-actions">
                            <button
                                class="btn-secondary btn-sm"
                                onclick={() => timer.selectZenVideo()}
                                title="Change zen mode background video"
                            >
                                Change
                            </button>
                            <button
                                class="btn-danger btn-sm"
                                onclick={() => timer.clearZenVideo()}
                                title="Remove zen mode background video"
                            >
                                Remove
                            </button>
                        </div>
                    {:else}
                        <button
                            class="btn-zen-video"
                            onclick={() => timer.selectZenVideo()}
                            title="Select a video to play in the background during Zen Mode"
                        >
                            <svg
                                class="icon"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                            >
                                <path
                                    d="M23 7l-7 5 7 5V7z M1 5h14v14H1z"
                                ></path>
                            </svg>
                            Select Zen Background Video
                        </button>
                    {/if}
                </div>
            </div>
        </div>
    {/if}
</div>

<style>
    .timer-container {
        background: var(--surface-color);
        border-radius: 1.25rem;
        padding: 2.5rem;
        box-shadow:
            0 1px 3px rgba(0, 0, 0, 0.08),
            0 4px 12px rgba(0, 0, 0, 0.05),
            0 0 0 1px var(--border-color);
        border: none;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .timer-display {
        display: flex;
        justify-content: center;
        margin-bottom: 2.5rem;
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
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        font-size: 1.1rem;
        font-weight: 500;
        letter-spacing: 0.02em;
        margin-bottom: 1rem;
        opacity: 0.85;
        transition: all 0.3s ease;
    }

    .session-type[data-type="work"] {
        color: var(--primary-color);
    }

    .session-type[data-type="break"] {
        color: #10b981;
    }

    .session-icon {
        width: 1.25rem;
        height: 1.25rem;
        stroke-width: 2.5;
    }

    .time {
        font-size: 3.5rem;
        font-weight: 700;
        letter-spacing: 0.05em;
        color: var(--primary-color);
        margin-bottom: 0.5rem;
        font-variant-numeric: tabular-nums;
        line-height: 1;
        font-family: "Courier New", monospace;
    }

    .active-task-indicator {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        margin: 0.75rem 0;
        padding: 0.5rem 1rem;
        background: var(--hover-bg);
        border-radius: 0.5rem;
        border: 1px solid var(--border-color);
        max-width: 250px;
        margin-left: auto;
        margin-right: auto;
    }

    .task-icon {
        width: 1rem;
        height: 1rem;
        color: var(--primary-color);
        flex-shrink: 0;
    }

    .task-name {
        font-size: 0.9rem;
        font-weight: 500;
        letter-spacing: 0.01em;
        color: var(--text-color);
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
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

    /* Animation for timer start */
    .timer-display.starting .circular-progress {
        animation: pulse 0.6s ease-out;
    }

    @keyframes pulse {
        0% {
            transform: scale(1);
        }
        50% {
            transform: scale(1.05);
        }
        100% {
            transform: scale(1);
        }
    }

    /* Session type color coding for progress ring */
    .circular-progress[data-session-type="work"] .progress-ring-progress {
        stroke: url(#gradient);
        filter: drop-shadow(0 0 4px var(--primary-color));
    }

    .circular-progress[data-session-type="break"] .progress-ring-progress {
        stroke: #10b981;
        filter: drop-shadow(0 0 4px #10b981);
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
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
        color: var(--text-color);
        text-align: left;
        width: 100%;
        box-shadow:
            0 1px 3px rgba(0, 0, 0, 0.06),
            0 2px 4px rgba(0, 0, 0, 0.04);
    }

    .preset-btn:hover {
        border-color: var(--primary-color);
        transform: translateY(-2px);
        box-shadow:
            0 4px 12px rgba(0, 0, 0, 0.1),
            0 2px 6px rgba(0, 0, 0, 0.06);
        background: var(--background-color);
    }

    .preset-btn:active:not(:disabled) {
        transform: translateY(0);
        box-shadow:
            0 1px 3px rgba(0, 0, 0, 0.06),
            0 2px 4px rgba(0, 0, 0, 0.04);
        transition: all 0.1s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .preset-btn.active {
        border-color: var(--primary-color);
        border-width: 3px;
        background: linear-gradient(
            135deg,
            var(--primary-color),
            var(--primary-light)
        );
        box-shadow:
            0 4px 12px rgba(99, 102, 241, 0.25),
            0 2px 4px rgba(99, 102, 241, 0.15),
            inset 0 1px 0 rgba(255, 255, 255, 0.1);
        color: white;
        box-shadow: 0 6px 20px rgba(99, 102, 241, 0.4);
        transform: scale(1.02);
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
        display: flex;
        align-items: center;
        gap: 0.5rem;
        font-weight: 600;
        letter-spacing: 0.01em;
        margin-bottom: 0.5rem;
        color: inherit;
        font-size: 1rem;
    }

    .check-icon {
        width: 1.1rem;
        height: 1.1rem;
        animation: checkmark 0.3s ease-out;
    }

    @keyframes checkmark {
        0% {
            transform: scale(0);
        }
        50% {
            transform: scale(1.2);
        }
        100% {
            transform: scale(1);
        }
    }

    .preset-times {
        font-size: 0.875rem;
        opacity: 0.9;
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
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
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
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
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
        transform: translateY(-1px);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    }

    .btn-primary:active:not(:disabled) {
        transform: translateY(0);
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        transition: all 0.1s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .btn-outline {
        background: transparent;
        color: var(--text-color);
        border: 1px solid var(--border-color);
    }

    .btn-outline:hover:not(:disabled) {
        background: var(--surface-color);
        transform: translateY(-1px);
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
    }

    .ambient-toggle.active {
        background: var(--primary-color);
        color: white;
        border-color: var(--primary-color);
    }

    .ambient-toggle.active:hover:not(:disabled) {
        background: var(--primary-dark);
        border-color: var(--primary-dark);
    }

    .icon {
        width: 1rem;
        height: 1rem;
        stroke-width: 2.5;
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

    /* Zen Mode Background Video */
    .zen-background-video {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        object-fit: cover;
        z-index: -1;
        opacity: 0.6;
        pointer-events: none;
    }

    .timer-container.zen-mode {
        background: rgba(0, 0, 0, 0.3);
        backdrop-filter: blur(2px);
        position: relative;
        z-index: 1;
    }

    /* Zen Video Section */
    .zen-video-section {
        margin-top: 2rem;
        padding: 2rem;
        background: linear-gradient(
            135deg,
            rgba(139, 92, 246, 0.05),
            rgba(99, 102, 241, 0.05)
        );
        border: 2px solid rgba(139, 92, 246, 0.2);
        border-radius: 1rem;
    }

    .zen-video-section h4 {
        margin: 0 0 1rem 0;
        font-size: 1rem;
        font-weight: 600;
        color: var(--text-color);
    }

    .zen-video-controls {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .video-info {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        padding: 0.75rem 1rem;
        background: var(--background-color);
        border-radius: 0.5rem;
        border: 1px solid var(--border-color);
    }

    .video-info .icon {
        width: 1.25rem;
        height: 1.25rem;
        color: var(--primary-color);
        flex-shrink: 0;
    }

    .video-name {
        flex: 1;
        font-size: 0.875rem;
        color: var(--text-color);
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .video-actions {
        display: flex;
        gap: 0.5rem;
    }

    .btn-zen-video {
        width: 100%;
        padding: 1rem 1.5rem;
        background: linear-gradient(
            135deg,
            rgba(139, 92, 246, 0.08),
            rgba(99, 102, 241, 0.08)
        );
        border: 2px dashed rgba(139, 92, 246, 0.3);
        border-radius: 0.75rem;
        color: var(--text-color);
        font-weight: 600;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.75rem;
        transition: all 0.3s ease;
    }

    .btn-zen-video:hover {
        background: linear-gradient(
            135deg,
            rgba(139, 92, 246, 0.12),
            rgba(99, 102, 241, 0.12)
        );
        border-color: rgba(139, 92, 246, 0.5);
        transform: translateY(-2px);
    }

    .btn-zen-video .icon {
        width: 1.25rem;
        height: 1.25rem;
    }

    .btn-secondary {
        background: var(--surface-color);
        border: 2px solid var(--border-color);
        color: var(--text-color);
    }

    .btn-secondary:hover {
        background: var(--hover-bg);
        border-color: var(--primary-color);
    }

    .btn-danger {
        background: rgba(239, 68, 68, 0.1);
        border: 2px solid rgba(239, 68, 68, 0.3);
        color: rgb(239, 68, 68);
    }

    .btn-danger:hover {
        background: rgba(239, 68, 68, 0.2);
        border-color: rgba(239, 68, 68, 0.5);
    }

    .btn-sm {
        padding: 0.5rem 1rem;
        font-size: 0.875rem;
    }
</style>
