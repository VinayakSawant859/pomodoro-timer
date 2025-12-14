<script lang="ts">
    import { onMount } from "svelte";
    import { sessionHistory, timer } from "$lib/state.svelte";
    import type { DailySessionHistory } from "$lib/state.svelte";

    let expanded = $state(false);

    // Use $derived to reactively get daily history
    const dailyHistory = $derived(sessionHistory.history);

    // Watch timer changes to reload sessions
    $effect(() => {
        // Access timer properties to create reactive dependency
        timer.isRunning;
        timer.sessionsCompleted;

        // Reload today's sessions when timer state changes
        if (dailyHistory) {
            sessionHistory.loadToday();
        }
    });

    onMount(() => {
        sessionHistory.loadToday();
    });

    function toggleExpanded() {
        expanded = !expanded;
    }

    function formatTime(minutes: number): string {
        if (minutes < 60) {
            return `${minutes}m`;
        }
        const hours = Math.floor(minutes / 60);
        const mins = minutes % 60;
        return `${hours}h ${mins}m`;
    }

    function formatSessionTime(startTime: string, endTime?: string): string {
        const start = new Date(startTime);
        const timeStr = start.toLocaleTimeString([], {
            hour: "2-digit",
            minute: "2-digit",
        });

        if (endTime) {
            const end = new Date(endTime);
            const endTimeStr = end.toLocaleTimeString([], {
                hour: "2-digit",
                minute: "2-digit",
            });
            return `${timeStr} - ${endTimeStr}`;
        }
        return timeStr;
    }

    function formatDate(dateString: string): string {
        try {
            const date = new Date(dateString);
            const day = String(date.getDate()).padStart(2, "0");
            const month = String(date.getMonth() + 1).padStart(2, "0");
            return `${day}/${month}`;
        } catch {
            return "";
        }
    }

    function getSessionIcon(type: string): string {
        switch (type) {
            case "work":
                return "🍅";
            case "short_break":
                return "☕";
            case "long_break":
                return "🧘";
            default:
                return "⏱️";
        }
    }

    function getSessionLabel(type: string): string {
        switch (type) {
            case "work":
                return "Work";
            case "short_break":
                return "Short Break";
            case "long_break":
                return "Long Break";
            default:
                return "Session";
        }
    }
</script>

<div class="session-progress">
    <div
        class="progress-header"
        onclick={toggleExpanded}
        role="button"
        tabindex="0"
        onkeydown={(e) => e.key === "Enter" && toggleExpanded()}
    >
        <div class="progress-summary">
            <h3>Today's Sessions</h3>
            {#if dailyHistory}
                <div class="stats-summary">
                    <span class="stat-item">
                        <span class="stat-icon">🍅</span>
                        <span class="stat-value"
                            >{dailyHistory.total_work_sessions}</span
                        >
                        <span class="stat-label">work</span>
                    </span>
                    <span class="stat-item">
                        <span class="stat-icon">⏱️</span>
                        <span class="stat-value"
                            >{formatTime(dailyHistory.total_work_time)}</span
                        >
                        <span class="stat-label">focused</span>
                    </span>
                    {#if dailyHistory.sessions.length > 0}
                        <span class="stat-item">
                            <span class="stat-icon">📈</span>
                            <span class="stat-value"
                                >{Math.round(
                                    dailyHistory.completion_rate,
                                )}%</span
                            >
                            <span class="stat-label">completed</span>
                        </span>
                    {/if}
                </div>
            {:else}
                <div class="stats-summary">
                    <span class="stat-item">
                        <span class="stat-icon">🍅</span>
                        <span class="stat-value">0</span>
                        <span class="stat-label">sessions today</span>
                    </span>
                </div>
            {/if}
        </div>
        <button
            class="expand-btn"
            class:expanded
            aria-label={expanded ? "Collapse sessions" : "Expand sessions"}
        >
            <svg
                class="icon"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
            >
                <polyline points="6,9 12,15 18,9"></polyline>
            </svg>
        </button>
    </div>

    {#if expanded && dailyHistory && dailyHistory.sessions.length > 0}
        <div class="session-timeline">
            <div class="timeline-header">
                <span>Session Timeline</span>
                <span class="session-count"
                    >{dailyHistory.sessions.length} sessions</span
                >
            </div>
            <div class="timeline">
                {#each dailyHistory.sessions as session, index}
                    <div
                        class="session-item"
                        class:completed={session.completed}
                        class:interrupted={!session.completed}
                    >
                        <div class="session-number">{index + 1}</div>
                        <div class="session-details">
                            <div class="session-type">
                                <span class="session-icon"
                                    >{getSessionIcon(session.type)}</span
                                >
                                <span class="session-name"
                                    >{getSessionLabel(session.type)}</span
                                >
                                <span class="session-duration"
                                    >({session.duration}min)</span
                                >
                            </div>
                            <div class="session-time">
                                {formatSessionTime(
                                    session.started_at,
                                    session.completed_at,
                                )}
                                <span class="session-date"
                                    >({formatDate(session.started_at)})</span
                                >
                            </div>
                        </div>
                        <div class="session-status">
                            {#if session.completed}
                                <span class="status-icon completed">✅</span>
                            {:else}
                                <span class="status-icon interrupted">❌</span>
                            {/if}
                        </div>
                    </div>
                {/each}
            </div>
        </div>
    {:else if expanded && dailyHistory && dailyHistory.sessions.length === 0}
        <div class="no-sessions">
            <div class="no-sessions-icon">🎯</div>
            <p>No sessions completed yet today.</p>
            <p>Start your first Pomodoro session!</p>
        </div>
    {/if}
</div>

<style>
    .session-progress {
        background: var(--surface-color);
        border-radius: 1rem;
        border: 1px solid var(--border-color);
        overflow: hidden;
        transition: all 0.3s ease;
    }

    .progress-header {
        padding: 1.5rem;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: space-between;
        transition: background-color 0.2s ease;
    }

    .progress-header:hover {
        background-color: rgba(99, 102, 241, 0.05);
    }

    .progress-summary h3 {
        margin: 0 0 0.75rem 0;
        color: var(--text-color);
        font-size: 1.1rem;
        font-weight: 600;
    }

    .stats-summary {
        display: flex;
        gap: 1.5rem;
        flex-wrap: wrap;
    }

    .stat-item {
        display: flex;
        align-items: center;
        gap: 0.25rem;
        font-size: 0.9rem;
    }

    .stat-icon {
        font-size: 1rem;
    }

    .stat-value {
        font-weight: 600;
        color: var(--primary-color);
    }

    .stat-label {
        color: var(--text-secondary);
    }

    .expand-btn {
        background: none;
        border: none;
        padding: 0.5rem;
        cursor: pointer;
        border-radius: 0.5rem;
        transition: all 0.2s ease;
        color: var(--text-secondary);
    }

    .expand-btn:hover {
        background-color: var(--primary-color);
        color: white;
    }

    .expand-btn.expanded {
        transform: rotate(180deg);
    }

    .icon {
        width: 1.25rem;
        height: 1.25rem;
        stroke-width: 2;
    }

    .session-timeline {
        border-top: 1px solid var(--border-color);
        padding: 1.5rem;
        background: rgba(99, 102, 241, 0.02);
    }

    .timeline-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 1rem;
        font-weight: 600;
        color: var(--text-color);
    }

    .session-count {
        font-size: 0.9rem;
        color: var(--text-secondary);
        font-weight: 400;
    }

    .timeline {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .session-item {
        display: flex;
        align-items: center;
        gap: 1rem;
        padding: 0.75rem;
        background: var(--background-color);
        border-radius: 0.75rem;
        border: 1px solid var(--border-color);
        transition: all 0.2s ease;
    }

    .session-item.completed {
        border-color: var(--success-color);
        background: rgba(16, 185, 129, 0.05);
    }

    .session-item.interrupted {
        border-color: var(--error-color);
        background: rgba(239, 68, 68, 0.05);
    }

    .session-number {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 2rem;
        height: 2rem;
        background: var(--primary-color);
        color: white;
        border-radius: 50%;
        font-size: 0.8rem;
        font-weight: 600;
        flex-shrink: 0;
    }

    .session-details {
        flex: 1;
        min-width: 0;
    }

    .session-type {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        margin-bottom: 0.25rem;
    }

    .session-icon {
        font-size: 1rem;
    }

    .session-name {
        font-weight: 500;
        color: var(--text-color);
    }

    .session-duration {
        color: var(--text-secondary);
        font-size: 0.85rem;
    }

    .session-time {
        color: var(--text-secondary);
        font-size: 0.8rem;
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .session-date {
        color: var(--text-secondary);
        font-size: 0.75rem;
        font-weight: 500;
    }

    .session-status {
        flex-shrink: 0;
    }

    .status-icon {
        font-size: 1rem;
    }

    .no-sessions {
        padding: 2rem;
        text-align: center;
        color: var(--text-secondary);
        border-top: 1px solid var(--border-color);
    }

    .no-sessions-icon {
        font-size: 2rem;
        margin-bottom: 0.5rem;
    }

    .no-sessions p {
        margin: 0.25rem 0;
    }

    @media (max-width: 640px) {
        .stats-summary {
            gap: 1rem;
        }

        .session-item {
            gap: 0.75rem;
        }

        .session-number {
            width: 1.5rem;
            height: 1.5rem;
            font-size: 0.7rem;
        }

        .session-timeline {
            padding: 1rem;
        }
    }
</style>
