<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { statsStore, sessionHistoryStore } from "$lib/stores";
    import Chart from "chart.js/auto";

    export let onClose: () => void;

    let todayChartCanvas: HTMLCanvasElement;
    let weeklyChartCanvas: HTMLCanvasElement;
    let todayChart: Chart | null = null;
    let weeklyChart: Chart | null = null;

    let weeklyData: any[] = [];

    onMount(async () => {
        // Load today's data
        await statsStore.loadToday();
        await sessionHistoryStore.loadToday();

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
                const stats = await statsStore.loadDaily(dateStr);
                weekData.push({
                    date: dateStr,
                    label: formatWeekDay(date),
                    pomodoros: stats?.pomodoros_completed || 0,
                    workTime: stats?.total_work_time || 0,
                    tasks: stats?.tasks_completed || 0,
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

        const dailyHistory = $sessionHistoryStore;
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

<div class="stats-overlay" on:click={onClose} role="presentation">
    <div
        class="stats-modal"
        on:click|stopPropagation
        role="dialog"
        aria-modal="true"
    >
        <div class="stats-header">
            <h2>📊 Statistics Dashboard</h2>
            <button
                class="close-btn"
                on:click={onClose}
                aria-label="Close statistics"
            >
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                    <line x1="18" y1="6" x2="6" y2="18"></line>
                    <line x1="6" y1="6" x2="18" y2="18"></line>
                </svg>
            </button>
        </div>

        <div class="stats-content">
            <!-- Today's Summary -->
            {#if $statsStore}
                <div class="summary-cards">
                    <div class="summary-card">
                        <div class="card-icon">🍅</div>
                        <div class="card-content">
                            <div class="card-value">
                                {$statsStore.pomodoros_completed}
                            </div>
                            <div class="card-label">Pomodoros Today</div>
                        </div>
                    </div>
                    <div class="summary-card">
                        <div class="card-icon">⏱️</div>
                        <div class="card-content">
                            <div class="card-value">
                                {Math.floor($statsStore.total_work_time / 60)}h {$statsStore.total_work_time %
                                    60}m
                            </div>
                            <div class="card-label">Work Time</div>
                        </div>
                    </div>
                    <div class="summary-card">
                        <div class="card-icon">✅</div>
                        <div class="card-content">
                            <div class="card-value">
                                {$statsStore.tasks_completed}
                            </div>
                            <div class="card-label">Tasks Done</div>
                        </div>
                    </div>
                    <div class="summary-card">
                        <div class="card-icon">📅</div>
                        <div class="card-content">
                            <div class="card-value">{getTodayDate()}</div>
                            <div class="card-label">Today</div>
                        </div>
                    </div>
                </div>
            {/if}

            <!-- Charts -->
            <div class="charts-container">
                <div class="chart-wrapper">
                    <canvas bind:this={todayChartCanvas}></canvas>
                </div>
                <div class="chart-wrapper">
                    <canvas bind:this={weeklyChartCanvas}></canvas>
                </div>
            </div>
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
        border-radius: 1rem;
        max-width: 1200px;
        width: 100%;
        max-height: 90vh;
        overflow-y: auto;
        box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
        border: 1px solid var(--border-color);
    }

    .stats-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1.5rem 2rem;
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
    }

    .close-btn svg {
        width: 1.5rem;
        height: 1.5rem;
        stroke-width: 2;
    }

    .stats-content {
        padding: 2rem;
    }

    .summary-cards {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
        gap: 1rem;
        margin-bottom: 2rem;
    }

    .summary-card {
        background: var(--background-color);
        border: 1px solid var(--border-color);
        border-radius: 0.75rem;
        padding: 1.5rem;
        display: flex;
        align-items: center;
        gap: 1rem;
        transition: all 0.2s ease;
    }

    .summary-card:hover {
        transform: translateY(-2px);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
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
        color: var(--primary-color);
        margin-bottom: 0.25rem;
    }

    .card-label {
        font-size: 0.875rem;
        color: var(--text-secondary);
        font-weight: 500;
    }

    .charts-container {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(450px, 1fr));
        gap: 2rem;
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
