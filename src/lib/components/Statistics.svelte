<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { stats, sessionHistory } from "$lib/state.svelte";
    import Chart from "chart.js/auto";

    interface Props {
        onClose: () => void;
    }

    let { onClose }: Props = $props();

    let todayChartCanvas: HTMLCanvasElement;
    let weeklyChartCanvas: HTMLCanvasElement;
    let todayChart: Chart | null = null;
    let weeklyChart: Chart | null = null;

    let weeklyData: any[] = [];
    let showDetails = $state(false);

    // Derived insights for calm reflection
    const focusQuality = $derived(() => {
        if (!stats.dailyStats) return null;
        const pomodoros = stats.dailyStats.pomodoros_completed;
        if (pomodoros === 0) return "Just getting started";
        if (pomodoros <= 2) return "Building momentum";
        if (pomodoros <= 4) return "Steady progress";
        if (pomodoros <= 6) return "Deep focus";
        return "Exceptional dedication";
    });

    const timeInvestment = $derived(() => {
        if (!stats.dailyStats) return null;
        const minutes = stats.dailyStats.total_work_time;
        if (minutes < 30) return "Taking it easy today";
        if (minutes < 60) return "Good effort invested";
        if (minutes < 120) return "Solid work session";
        if (minutes < 180) return "Deeply engaged";
        return "Remarkable commitment";
    });

    onMount(async () => {
        // Load today's data
        await stats.loadToday();
        await sessionHistory.loadToday();

        // Load weekly data
        await loadWeeklyData();

        // Create charts
        createTodayChart();
        createWeeklyChart();
    });

    onDestroy(() => {
        if (todayChart) todayChart.destroy();
        if (weeklyChart) weeklyChart.destroy();
    });

    async function loadWeeklyData() {
        const today = new Date();
        const weekData = [];

        for (let i = 6; i >= 0; i--) {
            const date = new Date(today);
            date.setDate(date.getDate() - i);
            const dateStr = date.toISOString().split("T")[0];

            try {
                const dailyData = await stats.loadDaily(dateStr);
                weekData.push({
                    date: dateStr,
                    label: formatWeekDay(date),
                    pomodoros: dailyData?.pomodoros_completed || 0,
                    workTime: dailyData?.total_work_time || 0,
                    tasks: dailyData?.tasks_completed || 0,
                });
            } catch {
                weekData.push({
                    date: dateStr,
                    label: formatWeekDay(date),
                    pomodoros: 0,
                    workTime: 0,
                    tasks: 0,
                });
            }
        }

        weeklyData = weekData;
    }

    function formatWeekDay(date: Date): string {
        const days = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
        return days[date.getDay()];
    }

    function createTodayChart() {
        if (!todayChartCanvas) return;

        const ctx = todayChartCanvas.getContext("2d");
        if (!ctx) return;

        const dailyHistory = sessionHistory.history;
        const workSessions =
            dailyHistory?.sessions.filter((s) => s.type === "work").length || 0;
        const shortBreaks =
            dailyHistory?.sessions.filter((s) => s.type === "short_break")
                .length || 0;
        const longBreaks =
            dailyHistory?.sessions.filter((s) => s.type === "long_break")
                .length || 0;
        const workTime = dailyHistory?.total_work_time || 0;

        todayChart = new Chart(ctx, {
            type: "bar",
            data: {
                labels: [
                    "Work Sessions",
                    "Short Breaks",
                    "Long Breaks",
                    "Work Time (min)",
                ],
                datasets: [
                    {
                        label: "Today's Activity",
                        data: [workSessions, shortBreaks, longBreaks, workTime],
                        backgroundColor: [
                            "rgba(99, 102, 241, 0.7)",
                            "rgba(16, 185, 129, 0.7)",
                            "rgba(245, 158, 11, 0.7)",
                            "rgba(139, 92, 246, 0.7)",
                        ],
                        borderColor: [
                            "rgba(99, 102, 241, 1)",
                            "rgba(16, 185, 129, 1)",
                            "rgba(245, 158, 11, 1)",
                            "rgba(139, 92, 246, 1)",
                        ],
                        borderWidth: 2,
                    },
                ],
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    legend: {
                        display: false,
                    },
                    title: {
                        display: true,
                        text: "Today's Session Statistics",
                        font: {
                            size: 16,
                            weight: "bold",
                        },
                    },
                },
                scales: {
                    y: {
                        beginAtZero: true,
                        ticks: {
                            stepSize: 1,
                        },
                    },
                },
            },
        });
    }

    function createWeeklyChart() {
        if (!weeklyChartCanvas || weeklyData.length === 0) return;

        const ctx = weeklyChartCanvas.getContext("2d");
        if (!ctx) return;

        weeklyChart = new Chart(ctx, {
            type: "line",
            data: {
                labels: weeklyData.map((d) => d.label),
                datasets: [
                    {
                        label: "Pomodoros",
                        data: weeklyData.map((d) => d.pomodoros),
                        borderColor: "rgba(99, 102, 241, 1)",
                        backgroundColor: "rgba(99, 102, 241, 0.1)",
                        tension: 0.4,
                        fill: true,
                    },
                    {
                        label: "Tasks Completed",
                        data: weeklyData.map((d) => d.tasks),
                        borderColor: "rgba(16, 185, 129, 1)",
                        backgroundColor: "rgba(16, 185, 129, 0.1)",
                        tension: 0.4,
                        fill: true,
                    },
                ],
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    legend: {
                        display: true,
                        position: "top",
                    },
                    title: {
                        display: true,
                        text: "Weekly Progress (Last 7 Days)",
                        font: {
                            size: 16,
                            weight: "bold",
                        },
                    },
                },
                scales: {
                    y: {
                        beginAtZero: true,
                        ticks: {
                            stepSize: 1,
                        },
                    },
                },
            },
        });
    }

    function getTodayDate(): string {
        const today = new Date();
        const day = String(today.getDate()).padStart(2, "0");
        const month = String(today.getMonth() + 1).padStart(2, "0");
        return `${day}/${month}`;
    }
</script>

<div class="stats-overlay" onclick={onClose} role="presentation">
    <div
        class="stats-modal"
        onclick={(e) => e.stopPropagation()}
        role="dialog"
        aria-modal="true"
    >
        <div class="stats-header">
            <h2>📊 Your Focus Journey</h2>
            <button
                class="close-btn"
                onclick={onClose}
                aria-label="Close statistics"
            >
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                    <line x1="18" y1="6" x2="6" y2="18"></line>
                    <line x1="6" y1="6" x2="18" y2="18"></line>
                </svg>
            </button>
        </div>

        <div class="stats-content">
            <!-- Calm Reflection Summary -->
            {#if stats.dailyStats}
                <div class="reflection-section">
                    <h3 class="reflection-title">Today's Reflection</h3>
                    <p class="reflection-insight">{focusQuality()}</p>
                    <p class="reflection-subtext">{timeInvestment()}</p>
                </div>

                <div class="summary-cards">
                    <div class="summary-card primary">
                        <div class="card-icon">🍅</div>
                        <div class="card-content">
                            <div class="card-value">
                                {stats.dailyStats.pomodoros_completed}
                            </div>
                            <div class="card-label">Focus Sessions</div>
                        </div>
                    </div>
                    <div class="summary-card">
                        <div class="card-icon">⏱️</div>
                        <div class="card-content">
                            <div class="card-value">
                                {Math.floor(
                                    stats.dailyStats.total_work_time / 60,
                                )}h {stats.dailyStats.total_work_time % 60}m
                            </div>
                            <div class="card-label">Time Invested</div>
                        </div>
                    </div>
                    <div class="summary-card">
                        <div class="card-icon">✅</div>
                        <div class="card-content">
                            <div class="card-value">
                                {stats.dailyStats.tasks_completed}
                            </div>
                            <div class="card-label">Tasks Completed</div>
                        </div>
                    </div>
                </div>
            {/if}

            <!-- Expandable Details -->
            <div class="details-toggle">
                <button
                    class="toggle-btn"
                    onclick={() => (showDetails = !showDetails)}
                >
                    {showDetails ? "Hide" : "Show"} Detailed Insights
                    <svg
                        class="toggle-icon"
                        class:rotated={showDetails}
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                    >
                        <polyline points="6,9 12,15 18,9"></polyline>
                    </svg>
                </button>
            </div>

            {#if showDetails}
                <!-- Charts -->
                <div class="charts-container">
                    <div class="chart-wrapper">
                        <canvas bind:this={todayChartCanvas}></canvas>
                    </div>
                    <div class="chart-wrapper">
                        <canvas bind:this={weeklyChartCanvas}></canvas>
                    </div>
                </div>
            {/if}
        </div>
    </div>
</div>

<style>
    .stats-overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
        padding: 1rem;
    }

    .stats-modal {
        background: var(--surface-color);
        border-radius: 1.25rem;
        max-width: 1200px;
        width: 100%;
        max-height: 90vh;
        overflow-y: auto;
        box-shadow:
            0 20px 60px rgba(0, 0, 0, 0.2),
            0 8px 24px rgba(0, 0, 0, 0.12),
            0 0 0 1px rgba(0, 0, 0, 0.05);
        border: none;
    }

    .stats-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 2rem 2.5rem;
        border-bottom: 1px solid var(--border-color);
    }

    .stats-header h2 {
        margin: 0;
        color: var(--text-color);
        font-size: 1.5rem;
    }

    .close-btn {
        background: none;
        border: none;
        cursor: pointer;
        color: var(--text-secondary);
        width: 2rem;
        height: 2rem;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 0.5rem;
        transition: all 0.2s ease;
    }

    .close-btn:hover {
        background: var(--background-color);
        color: var(--text-color);
        transform: scale(1.1);
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .close-btn svg {
        width: 1.5rem;
        height: 1.5rem;
        stroke-width: 2.5;
    }

    .stats-content {
        padding: 2.5rem;
        overflow-y: auto;
        max-height: calc(90vh - 80px);
    }

    .reflection-section {
        text-align: center;
        margin-bottom: 2.5rem;
        padding: 2rem;
        background: linear-gradient(
            135deg,
            rgba(99, 102, 241, 0.05),
            rgba(139, 92, 246, 0.05)
        );
        border-radius: 1rem;
        border: 1px solid rgba(99, 102, 241, 0.1);
    }

    .reflection-title {
        font-size: 1.1rem;
        font-weight: 600;
        color: var(--text-secondary);
        margin-bottom: 0.75rem;
    }

    .reflection-insight {
        font-size: 1.5rem;
        font-weight: 700;
        letter-spacing: -0.01em;
        color: var(--primary-color);
        margin-bottom: 0.5rem;
    }

    .reflection-subtext {
        font-size: 1rem;
        color: var(--text-secondary);
        font-weight: 500;
    }

    .summary-cards {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
        gap: 1rem;
        margin-bottom: 2rem;
    }

    .summary-card {
        background: var(--background-color);
        border: 2px solid var(--border-color);
        border-radius: 1rem;
        padding: 1.75rem;
        display: flex;
        align-items: center;
        gap: 1.25rem;
        transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .summary-card:hover {
        border-color: var(--primary-color);
        transform: translateY(-3px) scale(1.01);
        box-shadow:
            0 8px 20px rgba(0, 0, 0, 0.1),
            0 4px 8px rgba(0, 0, 0, 0.06);
    }

    .summary-card.primary {
        border-color: var(--primary-color);
        background: linear-gradient(
            135deg,
            rgba(99, 102, 241, 0.05),
            rgba(139, 92, 246, 0.05)
        );
        box-shadow:
            0 4px 12px rgba(99, 102, 241, 0.12),
            0 2px 4px rgba(99, 102, 241, 0.08);
    }

    .card-icon {
        font-size: 2rem;
    }

    .card-content {
        flex: 1;
    }

    .card-value {
        font-size: 1.5rem;
        font-weight: 700;
        letter-spacing: 0.02em;
        color: var(--primary-color);
        margin-bottom: 0.25rem;
    }

    .card-label {
        font-size: 0.875rem;
        color: var(--text-secondary);
        font-weight: 500;
        letter-spacing: 0.02em;
        opacity: 0.85;
    }

    .details-toggle {
        margin: 2rem 0;
        text-align: center;
    }

    .toggle-btn {
        display: inline-flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.75rem 1.5rem;
        background: var(--surface-color);
        border: 2px solid var(--border-color);
        border-radius: 0.5rem;
        color: var(--text-color);
        font-weight: 600;
        cursor: pointer;
        transition: all 0.2s ease;
    }

    .toggle-btn:hover {
        border-color: var(--primary-color);
        background: var(--hover-bg);
        transform: translateY(-1px);
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .toggle-icon {
        width: 1rem;
        height: 1rem;
        transition: transform 0.3s ease;
    }

    .toggle-icon.rotated {
        transform: rotate(180deg);
    }

    .charts-container {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(450px, 1fr));
        gap: 2rem;
        animation: slideDown 0.3s ease-out;
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

    .chart-wrapper {
        background: var(--background-color);
        border: 1px solid var(--border-color);
        border-radius: 0.75rem;
        padding: 1.5rem;
        height: 350px;
    }

    @media (max-width: 1024px) {
        .charts-container {
            grid-template-columns: 1fr;
        }
    }

    @media (max-width: 768px) {
        .stats-modal {
            max-height: 95vh;
        }

        .stats-header {
            padding: 1rem 1.5rem;
        }

        .stats-content {
            padding: 1.5rem;
        }

        .summary-cards {
            grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
        }

        .charts-container {
            gap: 1.5rem;
        }

        .chart-wrapper {
            height: 300px;
        }
    }
</style>
