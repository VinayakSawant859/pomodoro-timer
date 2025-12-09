<script lang="ts">
    import { taskStore, timerStore, statsStore } from "$lib/stores";
    import type { Task, DailyStats } from "$lib/stores";
    import { onMount } from "svelte";

    let newTaskText = "";
    let editingTask: Task | null = null;
    let editText = "";
    let showStats = false;

    // Callback prop to flip card and start timer
    export let onStartTask: (() => void) | undefined = undefined;

    $: incompleteTasks = $taskStore.filter((task) => !task.completed);
    $: completedTasks = $taskStore.filter((task) => task.completed);

    // Format date as dd/mm
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

    // Get today's date in dd/mm format
    function getTodayDate(): string {
        const today = new Date();
        const day = String(today.getDate()).padStart(2, "0");
        const month = String(today.getMonth() + 1).padStart(2, "0");
        return `${day}/${month}`;
    }

    onMount(async () => {
        // Load tasks and daily stats for today
        await taskStore.load();
        await statsStore.loadToday();
    });

    async function addTask() {
        if (newTaskText.trim()) {
            try {
                await taskStore.add(newTaskText.trim());
                newTaskText = "";
            } catch (error) {
                console.error("Failed to add task:", error);
            }
        }
    }

    function startEditTask(task: Task) {
        editingTask = task;
        editText = task.text;
    }

    async function saveEditTask() {
        if (editingTask && editText.trim()) {
            try {
                await taskStore.updateText(editingTask.id, editText.trim());
            } catch (error) {
                console.error("Failed to update task:", error);
            }
        }
        editingTask = null;
        editText = "";
    }

    function cancelEdit() {
        editingTask = null;
        editText = "";
    }

    async function toggleTaskComplete(task: Task) {
        try {
            if (task.completed) {
                await taskStore.uncomplete(task.id);
            } else {
                await taskStore.complete(task.id);
            }
        } catch (error) {
            console.error("Failed to toggle task completion:", error);
        }
    }

    async function deleteTask(taskId: string) {
        try {
            await taskStore.remove(taskId);
        } catch (error) {
            console.error("Failed to delete task:", error);
        }
    }

    function startTimerWithTask(taskId: string) {
        if (!$timerStore.isRunning) {
            timerStore.start(taskId);
            // Flip card to show timer when task is started
            if (onStartTask) {
                onStartTask();
            }
        }
    }

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === "Enter") {
            addTask();
        }
    }

    function handleEditKeydown(event: KeyboardEvent) {
        if (event.key === "Enter") {
            saveEditTask();
        } else if (event.key === "Escape") {
            cancelEdit();
        }
    }
</script>

<div class="task-manager">
    <h2>Tasks</h2>

    <div class="add-task">
        <input
            type="text"
            placeholder="Add a new task..."
            bind:value={newTaskText}
            on:keydown={handleKeydown}
            class="task-input"
        />
        <button
            class="btn btn-primary"
            on:click={addTask}
            disabled={!newTaskText.trim()}
        >
            <svg
                class="icon"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
            >
                <line x1="12" y1="5" x2="12" y2="19"></line>
                <line x1="5" y1="12" x2="19" y2="12"></line>
            </svg>
            Add
        </button>
    </div>

    <!-- Stats Section -->
    <div class="stats-section">
        <button class="stats-toggle" on:click={() => (showStats = !showStats)}>
            📊 Today's Stats ({getTodayDate()}) {showStats ? "▼" : "▶"}
        </button>

        {#if showStats && $statsStore}
            <div class="stats-content">
                <div class="stat-item">
                    <span class="stat-label">🍅 Pomodoros:</span>
                    <span class="stat-value"
                        >{$statsStore.pomodoros_completed}</span
                    >
                </div>
                <div class="stat-item">
                    <span class="stat-label">⏱️ Work Time:</span>
                    <span class="stat-value"
                        >{Math.floor($statsStore.total_work_time / 60)}h {$statsStore.total_work_time %
                            60}m</span
                    >
                </div>
                <div class="stat-item">
                    <span class="stat-label">✅ Tasks Done:</span>
                    <span class="stat-value">{$statsStore.tasks_completed}</span
                    >
                </div>
            </div>
        {/if}
    </div>

    <div class="task-sections">
        <!-- Incomplete Tasks -->
        {#if incompleteTasks.length > 0}
            <div class="task-section">
                <h3>To Do ({incompleteTasks.length})</h3>
                <ul class="task-list">
                    {#each incompleteTasks as task (task.id)}
                        <li
                            class="task-item"
                            class:current={$timerStore.currentTaskId ===
                                task.id}
                        >
                            <div class="task-content">
                                <div
                                    class="task-checkbox"
                                    title="Task will be marked complete when timer finishes"
                                >
                                    <svg
                                        class="icon"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                    >
                                        <circle cx="12" cy="12" r="10"></circle>
                                    </svg>
                                </div>

                                {#if editingTask?.id === task.id}
                                    <input
                                        type="text"
                                        bind:value={editText}
                                        on:keydown={handleEditKeydown}
                                        on:blur={saveEditTask}
                                        class="edit-input"
                                    />
                                {:else}
                                    <div class="task-text-wrapper">
                                        <span
                                            class="task-text"
                                            on:dblclick={() =>
                                                startEditTask(task)}
                                            role="button"
                                            tabindex="0"
                                        >
                                            {task.text}
                                        </span>
                                        <span class="task-date"
                                            >{formatDate(task.created_at)}</span
                                        >
                                    </div>
                                {/if}
                            </div>

                            <div class="task-actions">
                                {#if !$timerStore.isRunning && $timerStore.currentTaskId !== task.id}
                                    <button
                                        class="btn-icon"
                                        on:click={() =>
                                            startTimerWithTask(task.id)}
                                        title="Start timer for this task"
                                    >
                                        <svg
                                            class="icon"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="currentColor"
                                        >
                                            <polygon points="5,3 19,12 5,21 5,3"
                                            ></polygon>
                                        </svg>
                                    </button>
                                {/if}

                                {#if editingTask?.id !== task.id}
                                    <button
                                        class="btn-icon"
                                        on:click={() => startEditTask(task)}
                                        title="Edit task"
                                    >
                                        <svg
                                            class="icon"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="currentColor"
                                        >
                                            <path
                                                d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"
                                            ></path>
                                            <path
                                                d="M18.5 2.5a2.12 2.12 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"
                                            ></path>
                                        </svg>
                                    </button>

                                    <button
                                        class="btn-icon btn-icon-danger"
                                        on:click={() => deleteTask(task.id)}
                                        title="Delete task"
                                    >
                                        <svg
                                            class="icon"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="currentColor"
                                        >
                                            <polyline points="3,6 5,6 21,6"
                                            ></polyline>
                                            <path
                                                d="M19,6v14a2,2,0,0,1-2,2H7a2,2,0,0,1-2-2V6m3,0V4a2,2,0,0,1,2-2h4a2,2,0,0,1,2,2V6"
                                            ></path>
                                        </svg>
                                    </button>
                                {/if}
                            </div>
                        </li>
                    {/each}
                </ul>
            </div>
        {/if}

        <!-- Completed Tasks -->
        {#if completedTasks.length > 0}
            <div class="task-section">
                <h3>Completed ({completedTasks.length})</h3>
                <ul class="task-list">
                    {#each completedTasks as task (task.id)}
                        <li class="task-item completed">
                            <div class="task-content">
                                <button
                                    class="task-checkbox checked"
                                    on:click={() => toggleTaskComplete(task)}
                                    aria-label="Mark task as incomplete"
                                >
                                    <svg
                                        class="icon"
                                        viewBox="0 0 24 24"
                                        fill="currentColor"
                                        stroke="currentColor"
                                    >
                                        <circle cx="12" cy="12" r="10"></circle>
                                        <polyline points="9,12 12,15 16,10"
                                        ></polyline>
                                    </svg>
                                </button>

                                <div class="task-text-wrapper">
                                    <span class="task-text">{task.text}</span>
                                    {#if task.completed_at}
                                        <span class="task-date"
                                            >✓ {formatDate(
                                                task.completed_at,
                                            )}</span
                                        >
                                    {/if}
                                </div>
                            </div>

                            <div class="task-actions">
                                <button
                                    class="btn-icon btn-icon-danger"
                                    on:click={() => deleteTask(task.id)}
                                    title="Delete task"
                                >
                                    <svg
                                        class="icon"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                    >
                                        <polyline points="3,6 5,6 21,6"
                                        ></polyline>
                                        <path
                                            d="M19,6v14a2,2,0,0,1-2,2H7a2,2,0,0,1-2-2V6m3,0V4a2,2,0,0,1,2-2h4a2,2,0,0,1,2,2V6"
                                        ></path>
                                    </svg>
                                </button>
                            </div>
                        </li>
                    {/each}
                </ul>
            </div>
        {/if}

        {#if incompleteTasks.length === 0 && completedTasks.length === 0}
            <div class="empty-state">
                <svg
                    class="empty-icon"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                >
                    <rect x="3" y="4" width="18" height="18" rx="2" ry="2"
                    ></rect>
                    <line x1="16" y1="2" x2="16" y2="6"></line>
                    <line x1="8" y1="2" x2="8" y2="6"></line>
                    <line x1="3" y1="10" x2="21" y2="10"></line>
                </svg>
                <h3>No tasks yet</h3>
                <p>Add a task to get started with your Pomodoro sessions!</p>
            </div>
        {/if}
    </div>
</div>

<style>
    .task-manager {
        background: var(--surface-color);
        border-radius: 1rem;
        padding: 2rem;
        box-shadow: 0 4px 6px -1px var(--shadow);
        border: 1px solid var(--border-color);
    }

    h2 {
        margin-bottom: 1.5rem;
        color: var(--text-color);
    }

    .add-task {
        display: flex;
        gap: 0.5rem;
        margin-bottom: 2rem;
    }

    .task-input {
        flex: 1;
        padding: 0.75rem;
        border: 1px solid var(--border-color);
        border-radius: 0.5rem;
        background: var(--background-color);
        color: var(--text-color);
        font-size: 1rem;
    }

    .task-input:focus {
        outline: none;
        border-color: var(--primary-color);
    }

    .task-sections {
        display: flex;
        flex-direction: column;
        gap: 2rem;
    }

    .task-section h3 {
        margin-bottom: 1rem;
        color: var(--text-secondary);
        font-size: 1rem;
        font-weight: 600;
    }

    .task-list {
        list-style: none;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .task-item {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 1rem;
        background: var(--background-color);
        border: 1px solid var(--border-color);
        border-radius: 0.5rem;
        transition: all 0.2s ease;
    }

    .task-item:hover {
        border-color: var(--primary-color);
    }

    .task-item.current {
        border-color: var(--primary-color);
        background: rgba(99, 102, 241, 0.05);
    }

    .task-item.completed {
        opacity: 0.7;
    }

    .task-content {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        flex: 1;
    }

    .task-checkbox {
        background: none;
        border: none;
        cursor: pointer;
        color: var(--border-color);
        transition: color 0.2s ease;
        padding: 0;
    }

    .task-checkbox:hover {
        color: var(--primary-color);
    }

    .task-checkbox.checked {
        color: var(--success-color);
    }

    .task-text-wrapper {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        flex: 1;
    }

    .task-text {
        color: var(--text-color);
        cursor: pointer;
    }

    .task-date {
        font-size: 0.8rem;
        color: var(--text-secondary);
        font-weight: 500;
    }

    .completed .task-text {
        text-decoration: line-through;
        color: var(--text-secondary);
    }

    .edit-input {
        flex: 1;
        padding: 0.25rem 0.5rem;
        border: 1px solid var(--primary-color);
        border-radius: 0.25rem;
        background: var(--background-color);
        color: var(--text-color);
        font-size: inherit;
    }

    .task-actions {
        display: flex;
        gap: 0.25rem;
    }

    .btn {
        display: inline-flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.5rem 1rem;
        border: none;
        border-radius: 0.5rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s ease;
    }

    .btn:hover:not(:disabled) {
        background-color: var(--primary-color);
        color: white;
        border-color: var(--primary-color);
    }

    .btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
        background-color: var(--surface-color);
        color: var(--text-secondary);
    }

    .btn-primary {
        background: var(--primary-color);
        color: white;
    }

    .btn-primary:hover:not(:disabled) {
        background: var(--primary-dark);
    }

    .btn-icon {
        background: none;
        border: none;
        cursor: pointer;
        padding: 0.5rem;
        border-radius: 0.25rem;
        color: var(--text-secondary);
        transition: all 0.2s ease;
    }

    .btn-icon:hover {
        background: var(--surface-color);
        color: var(--text-color);
    }

    .btn-icon-danger:hover {
        background: var(--error-color);
        color: white;
    }

    .icon {
        width: 1rem;
        height: 1rem;
        stroke-width: 2;
    }

    .empty-state {
        text-align: center;
        padding: 3rem 1rem;
        color: var(--text-secondary);
    }

    .empty-icon {
        width: 3rem;
        height: 3rem;
        margin: 0 auto 1rem;
        color: var(--border-color);
    }

    .empty-state h3 {
        margin-bottom: 0.5rem;
        color: var(--text-secondary);
    }

    /* Stats Section */
    .stats-section {
        margin-bottom: 2rem;
        border: 1px solid var(--border-color);
        border-radius: 0.5rem;
        overflow: hidden;
    }

    .stats-toggle {
        width: 100%;
        padding: 1rem;
        background: var(--surface-color);
        border: none;
        cursor: pointer;
        font-weight: 500;
        text-align: left;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        transition: all 0.2s ease;
        color: var(--text-color);
    }

    .stats-toggle:hover {
        background: var(--surface-hover);
    }

    .stats-content {
        padding: 1rem;
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
        gap: 1rem;
    }

    .stat-item {
        display: flex;
        flex-direction: column;
        align-items: center;
        text-align: center;
    }

    .stat-label {
        font-size: 0.875rem;
        color: var(--text-secondary);
        margin-bottom: 0.25rem;
    }

    .stat-value {
        font-size: 1.25rem;
        font-weight: 600;
        color: var(--accent-color);
    }

    @media (max-width: 768px) {
        .task-manager {
            padding: 1.5rem;
        }

        .task-item {
            padding: 0.75rem;
        }

        .add-task {
            flex-direction: column;
        }
    }
</style>
