<script lang="ts">
    import { stats } from "$lib/state.svelte";
    import type { DailyStats } from "$lib/state.svelte";

    interface Props {
        dailyStats: DailyStats;
        onClose: () => void;
    }

    let { dailyStats, onClose }: Props = $props();

    // Format time display
    function formatTime(minutes: number): string {
        const hours = Math.floor(minutes / 60);
        const mins = minutes % 60;

        if (hours > 0) {
            return mins > 0 ? `${hours}h ${mins}m` : `${hours}h`;
        }
        return `${mins}m`;
    }

    // Get encouraging message based on effort
    function getMessage(): string {
        const pomodoros = dailyStats.pomodoros_completed;

        if (pomodoros === 0) {
            return "Rest is progress too";
        } else if (pomodoros <= 2) {
            return "Every step counts";
        } else if (pomodoros <= 4) {
            return "Consistent effort builds momentum";
        } else if (pomodoros <= 6) {
            return "Your focus made a difference today";
        } else {
            return "Remarkable dedication today";
        }
    }
</script>

<div class="summary-overlay" onclick={onClose} role="presentation">
    <div
        class="summary-modal"
        onclick={(e) => e.stopPropagation()}
        role="dialog"
        aria-modal="true"
        aria-labelledby="summary-title"
    >
        <div class="summary-content">
            <div class="summary-icon">🌙</div>

            <h2 id="summary-title" class="summary-title">Day complete</h2>

            <div class="summary-main">
                <p class="summary-focus">
                    You focused for <strong
                        >{formatTime(dailyStats.total_work_time)}</strong
                    > today
                </p>
            </div>

            <div class="summary-details">
                <div class="detail-item">
                    <span class="detail-value"
                        >{dailyStats.pomodoros_completed}</span
                    >
                    <span class="detail-label"
                        >session{dailyStats.pomodoros_completed !== 1
                            ? "s"
                            : ""}</span
                    >
                </div>
                <div class="detail-separator">·</div>
                <div class="detail-item">
                    <span class="detail-value"
                        >{dailyStats.tasks_completed}</span
                    >
                    <span class="detail-label"
                        >task{dailyStats.tasks_completed !== 1 ? "s" : ""}</span
                    >
                </div>
            </div>

            <p class="summary-message">{getMessage()}</p>

            <button
                class="summary-dismiss"
                onclick={onClose}
                aria-label="Close summary"
            >
                Acknowledged
            </button>
        </div>
    </div>
</div>

<style>
    .summary-overlay {
        position: fixed;
        inset: 0;
        background: rgba(0, 0, 0, 0.6);
        backdrop-filter: blur(8px);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 2000;
        animation: fadeIn 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    }

    @keyframes fadeIn {
        from {
            opacity: 0;
        }
        to {
            opacity: 1;
        }
    }

    .summary-modal {
        background: var(--surface-color);
        border-radius: 1.25rem;
        max-width: 400px;
        width: 90%;
        box-shadow:
            0 20px 60px rgba(0, 0, 0, 0.3),
            0 8px 24px rgba(0, 0, 0, 0.2);
        animation: slideScale 0.4s cubic-bezier(0.4, 0, 0.2, 1);
    }

    @keyframes slideScale {
        from {
            opacity: 0;
            transform: translateY(20px) scale(0.95);
        }
        to {
            opacity: 1;
            transform: translateY(0) scale(1);
        }
    }

    .summary-content {
        padding: 2.5rem 2rem;
        text-align: center;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1.25rem;
    }

    .summary-icon {
        font-size: 2.5rem;
        opacity: 0.9;
        margin-bottom: 0.5rem;
    }

    .summary-title {
        font-size: 1.5rem;
        font-weight: 600;
        letter-spacing: -0.01em;
        color: var(--text-color);
        margin: 0;
    }

    .summary-main {
        margin: 0.5rem 0;
    }

    .summary-focus {
        font-size: 1.1rem;
        color: var(--text-secondary);
        line-height: 1.6;
        margin: 0;
    }

    .summary-focus strong {
        color: var(--primary-color);
        font-weight: 700;
        font-size: 1.25rem;
        letter-spacing: 0.01em;
    }

    .summary-details {
        display: flex;
        align-items: center;
        gap: 1rem;
        padding: 1rem 1.5rem;
        background: var(--background-color);
        border-radius: 0.75rem;
        border: 1px solid var(--border-color);
    }

    .detail-item {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.25rem;
    }

    .detail-value {
        font-size: 1.5rem;
        font-weight: 700;
        color: var(--primary-color);
        letter-spacing: 0.02em;
    }

    .detail-label {
        font-size: 0.8rem;
        color: var(--text-secondary);
        text-transform: lowercase;
        letter-spacing: 0.02em;
        opacity: 0.85;
    }

    .detail-separator {
        font-size: 1.25rem;
        color: var(--border-color);
        opacity: 0.5;
    }

    .summary-message {
        font-size: 0.95rem;
        color: var(--text-secondary);
        font-style: italic;
        margin: 0.5rem 0 0 0;
        opacity: 0.9;
    }

    .summary-dismiss {
        margin-top: 1rem;
        padding: 0.75rem 2rem;
        background: var(--primary-color);
        color: white;
        border: none;
        border-radius: 0.75rem;
        font-weight: 600;
        font-size: 0.95rem;
        cursor: pointer;
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
        letter-spacing: 0.01em;
    }

    .summary-dismiss:hover {
        background: var(--primary-dark);
        transform: translateY(-1px);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    }

    .summary-dismiss:active {
        transform: translateY(0);
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }

    /* Respect reduced motion preference */
    @media (prefers-reduced-motion: reduce) {
        .summary-overlay,
        .summary-modal {
            animation: none;
        }

        .summary-dismiss {
            transition: background-color 0.2s ease;
        }

        .summary-dismiss:hover {
            transform: none;
        }

        .summary-dismiss:active {
            transform: none;
        }
    }

    @media (max-width: 480px) {
        .summary-modal {
            max-width: 95%;
        }

        .summary-content {
            padding: 2rem 1.5rem;
            gap: 1rem;
        }

        .summary-icon {
            font-size: 2rem;
        }

        .summary-title {
            font-size: 1.3rem;
        }

        .summary-focus {
            font-size: 1rem;
        }

        .summary-focus strong {
            font-size: 1.15rem;
        }
    }
</style>
