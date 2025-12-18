<script lang="ts">
    import { flip } from "svelte/animate";
    import { onMount } from "svelte";
    import { tasks, timer } from "$lib/state.svelte";
    import ConfirmDialog from "$lib/components/ConfirmDialog.svelte";

    interface Props {
        onStartTask?: () => void;
    }

    let { onStartTask }: Props = $props();

    const priorityOptions = [
        { value: 3, label: "High", color: "#ff6b6b" },
        { value: 2, label: "Med", color: "#f7c948" },
        { value: 1, label: "Low", color: "#4da3ff" },
        { value: 0, label: "None", color: "#cbd5e1" },
    ];

    const tomatoRange = [1, 2, 3, 4, 5, 6];

    let newTaskText = $state("");
    let newTaskPriority = $state(2);
    let newTaskEstimate = $state(2);
    let editingTaskId = $state<string | null>(null);
    let editingText = $state("");
    let isLoading = $state(false);
    let showDeleteConfirm = $state(false);
    let taskToDelete = $state<string | null>(null);

    const sortedTasks = $derived(
        [...tasks.tasks].sort((a, b) => {
            if (b.priority !== a.priority) return b.priority - a.priority;
            return (
                new Date(b.created_at).getTime() -
                new Date(a.created_at).getTime()
            );
        }),
    );

    onMount(() => {
        tasks.load();
    });

    async function addTask() {
        if (!newTaskText.trim()) return;
        isLoading = true;
        try {
            await tasks.add(
                newTaskText.trim(),
                newTaskPriority,
                newTaskEstimate,
            );
            newTaskText = "";
        } catch (error) {
            console.error("Failed to add task:", error);
        }
        isLoading = false;
    }

    function beginEdit(taskId: string, text: string) {
        editingTaskId = taskId;
        editingText = text;
    }

    async function saveEdit(taskId: string) {
        const next = editingText.trim();
        if (next) {
            await tasks.updateText(taskId, next);
        }
        editingTaskId = null;
        editingText = "";
    }

    function cancelEdit() {
        editingTaskId = null;
        editingText = "";
    }

    async function toggleCompletion(taskId: string) {
        const task = tasks.tasks.find((t) => t.id === taskId);
        if (!task) return;
        if (task.completed) {
            await tasks.uncomplete(taskId);
        } else {
            await tasks.complete(taskId);
        }
    }

    async function removeTask(taskId: string) {
        taskToDelete = taskId;
        showDeleteConfirm = true;
    }

    async function confirmDelete() {
        if (taskToDelete) {
            await tasks.remove(taskToDelete);
            taskToDelete = null;
        }
    }

    function cancelDelete() {
        taskToDelete = null;
    }

    async function setPriority(taskId: string, priority: number) {
        await tasks.updatePriority(taskId, priority);
    }

    async function setEstimate(taskId: string, estimated_pomodoros: number) {
        await tasks.updateEstimate(taskId, estimated_pomodoros);
    }

    function focusTask(taskId: string) {
        timer.setActiveTask(taskId);
        onStartTask?.();
    }

    function priorityColor(priority: number) {
        return (
            priorityOptions.find((p) => p.value === priority)?.color ??
            "var(--border-color)"
        );
    }
</script>

<div class="task-manager">
    <div class="header-row">
        <h2>Tasks</h2>
        <div class="meta">
            <span>{sortedTasks.length} total</span>
            <span>{sortedTasks.filter((t) => !t.completed).length} open</span>
        </div>
    </div>

    <div class="new-task-card">
        <div class="new-input">
            <input
                type="text"
                placeholder="Add a focused task"
                bind:value={newTaskText}
                onkeydown={(e) => e.key === "Enter" && addTask()}
            />
            <button onclick={addTask} disabled={isLoading}>
                {isLoading ? "Adding…" : "Add task"}
            </button>
        </div>

        <div class="new-meta">
            <div class="priority-group">
                <span class="label">Priority</span>
                <div class="priority-chips">
                    {#each priorityOptions as option}
                        <button
                            class:active={newTaskPriority === option.value}
                            style={`--chip-color: ${option.color}`}
                            onclick={() => (newTaskPriority = option.value)}
                        >
                            {option.label}
                        </button>
                    {/each}
                </div>
            </div>

            <div class="tomato-picker">
                <span class="label">Estimate</span>
                <div class="tomatoes">
                    {#each tomatoRange as count}
                        <button
                            class:active={newTaskEstimate === count}
                            onclick={() => (newTaskEstimate = count)}
                        >
                            {#each Array(count) as _}
                                <img
                                    src="/icons/tomato.svg"
                                    alt="Tomato"
                                    class="tomato-icon"
                                />
                            {/each}
                        </button>
                    {/each}
                </div>
            </div>
        </div>
    </div>

    {#if sortedTasks.length === 0}
        <p class="empty-state">No tasks yet. Add your first focus item.</p>
    {:else}
        <div class="tasks-grid">
            {#each sortedTasks as task (task.id)}
                <article
                    class={`task-card ${task.completed ? "completed" : ""} ${timer.currentTaskId === task.id ? "active" : ""}`}
                    style={`--priority-color: ${priorityColor(task.priority)}`}
                    animate:flip
                >
                    <div class="card-top">
                        <div
                            class="badge priority"
                            title={`Priority ${task.priority}`}
                        >
                            {priorityOptions.find(
                                (p) => p.value === task.priority,
                            )?.label ?? "None"}
                        </div>
                        {#if timer.currentTaskId === task.id}
                            <div class="badge focus">Focusing</div>
                        {/if}
                    </div>

                    <div class="title-row">
                        <div class="title-group">
                            <input
                                type="checkbox"
                                checked={task.completed}
                                onchange={() => toggleCompletion(task.id)}
                                aria-label={`Mark ${task.text} as ${task.completed ? "incomplete" : "complete"}`}
                            />
                            {#if editingTaskId === task.id}
                                <input
                                    class="edit-input"
                                    bind:value={editingText}
                                    onkeydown={(e) => {
                                        if (e.key === "Enter")
                                            saveEdit(task.id);
                                        if (e.key === "Escape") cancelEdit();
                                    }}
                                    onblur={() => saveEdit(task.id)}
                                />
                            {:else}
                                <button
                                    class="task-title"
                                    onclick={() =>
                                        beginEdit(task.id, task.text)}
                                >
                                    {task.text}
                                </button>
                            {/if}
                        </div>
                        <button
                            class="ghost small"
                            onclick={() => beginEdit(task.id, task.text)}
                        >
                            Edit
                        </button>
                    </div>

                    <div class="meta-row">
                        <div class="tomato-actual" title="Actual / Estimated">
                            {#each Array(task.actual_pomodoros) as _, i}
                                <img
                                    src="/icons/tomato.svg"
                                    alt="Tomato"
                                    class="tomato filled"
                                    aria-label={`Actual ${i + 1}`}
                                />
                            {/each}
                            {#each Array(Math.max(task.estimated_pomodoros - task.actual_pomodoros, 0)) as _, i}
                                <span
                                    class="tomato empty"
                                    aria-label={`Remaining ${i + 1}`}>○</span
                                >
                            {/each}
                            <span class="ratio"
                                >{task.actual_pomodoros}/{task.estimated_pomodoros}</span
                            >
                        </div>
                        <div class="estimate-picker">
                            {#each tomatoRange as count}
                                <button
                                    class:active={task.estimated_pomodoros ===
                                        count}
                                    onclick={() => setEstimate(task.id, count)}
                                    aria-label={`Set estimate to ${count} pomodoros`}
                                >
                                    {#each Array(count) as _}
                                        <img
                                            src="/icons/tomato.svg"
                                            alt="Tomato"
                                            class="tomato-icon"
                                        />
                                    {/each}
                                </button>
                            {/each}
                        </div>
                    </div>

                    <div class="footer-row">
                        <div class="priority-controls">
                            {#each priorityOptions as option}
                                <button
                                    class:active={task.priority ===
                                        option.value}
                                    style={`--chip-color: ${option.color}`}
                                    onclick={() =>
                                        setPriority(task.id, option.value)}
                                >
                                    {option.label}
                                </button>
                            {/each}
                        </div>
                        <div class="actions">
                            <button
                                class="primary"
                                onclick={() => focusTask(task.id)}
                                aria-label={`Focus ${task.text}`}
                            >
                                ▶ Focus
                            </button>
                            <button
                                class="ghost danger"
                                onclick={() => removeTask(task.id)}
                                >Delete</button
                            >
                        </div>
                    </div>
                </article>
            {/each}
        </div>
    {/if}
</div>

<ConfirmDialog
    bind:isOpen={showDeleteConfirm}
    title="Delete Task"
    message="Are you sure you want to delete this task? This action cannot be undone."
    confirmText="Delete"
    cancelText="Cancel"
    variant="danger"
    onConfirm={confirmDelete}
    onCancel={cancelDelete}
/>

<style>
    .task-manager {
        background: var(--surface-color);
        border-radius: 20px;
        padding: 28px;
        box-shadow: 0 20px 60px var(--shadow);
        display: flex;
        flex-direction: column;
        gap: 20px;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .header-row {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 4px;
    }

    .header-row h2 {
        color: var(--text-color);
        font-size: 1.75rem;
        font-weight: 700;
        letter-spacing: -0.02em;
    }

    .meta {
        display: flex;
        gap: 14px;
        color: var(--text-secondary);
        font-weight: 600;
        font-size: 0.9rem;
    }

    .new-task-card {
        border: 1px solid var(--border-color);
        border-radius: 16px;
        padding: 20px;
        background: var(--background-color);
        display: flex;
        flex-direction: column;
        gap: 18px;
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .new-input {
        display: grid;
        grid-template-columns: 1fr auto;
        gap: 12px;
    }

    .new-input input {
        padding: 14px 16px;
        border: 1px solid var(--border-color);
        border-radius: 12px;
        background: var(--surface-color);
        color: var(--text-color);
        font-size: 0.95rem;
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .new-input input:focus {
        outline: none;
        border-color: var(--primary-color);
        box-shadow: 0 0 0 3px rgba(var(--primary-color), 0.1);
    }

    .new-input button {
        padding: 14px 20px;
        border: none;
        border-radius: 12px;
        background: var(--primary-color);
        color: white;
        font-weight: 700;
        font-size: 0.95rem;
        cursor: pointer;
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .new-input button:hover:not(:disabled) {
        background: var(--primary-light);
        transform: translateY(-2px);
        box-shadow: 0 8px 20px var(--shadow);
    }

    .new-input button:active:not(:disabled) {
        transform: translateY(0);
    }

    .new-input button:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .new-meta {
        display: grid;
        gap: 16px;
    }

    .label {
        font-size: 11px;
        text-transform: uppercase;
        letter-spacing: 0.05em;
        color: var(--text-secondary);
        font-weight: 700;
        margin-bottom: 2px;
    }

    .priority-group,
    .tomato-picker {
        display: grid;
        gap: 8px;
    }

    .priority-chips,
    .tomatoes {
        display: flex;
        flex-wrap: wrap;
        gap: 10px;
    }

    .priority-chips button,
    .priority-controls button {
        border: 2px solid var(--chip-color, var(--border-color));
        background: transparent;
        color: var(--text-color);
        padding: 10px 16px;
        border-radius: 999px;
        cursor: pointer;
        font-weight: 700;
        font-size: 0.85rem;
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .priority-chips button:hover,
    .priority-controls button:hover {
        transform: translateY(-1px);
        box-shadow: 0 4px 12px var(--shadow);
    }

    .priority-chips button.active,
    .priority-controls button.active {
        background: var(--chip-color);
        color: white;
        border-color: var(--chip-color);
        box-shadow: 0 4px 14px var(--shadow);
    }

    .tomatoes button,
    .estimate-picker button {
        border: 1px solid var(--border-color);
        background: var(--surface-color);
        color: var(--text-color);
        padding: 8px 12px;
        border-radius: 12px;
        cursor: pointer;
        display: flex;
        align-items: center;
        gap: 4px;
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .tomatoes button:hover,
    .estimate-picker button:hover {
        transform: translateY(-1px);
        box-shadow: 0 4px 12px var(--shadow);
    }

    .tomatoes button.active,
    .estimate-picker button.active {
        border-color: var(--primary-color);
        background: var(--primary-color);
        box-shadow: 0 6px 18px var(--shadow);
    }

    .tomato-icon {
        width: 18px;
        height: 18px;
        transition: transform 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .tomatoes button:hover .tomato-icon,
    .estimate-picker button:hover .tomato-icon {
        transform: scale(1.1);
    }

    .tasks-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
        gap: 16px;
    }

    .task-card {
        position: relative;
        border: 1px solid var(--border-color);
        border-radius: 18px;
        background: var(--background-color);
        overflow: hidden;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        border-left: 5px solid var(--priority-color);
        padding: 18px 20px 20px;
        display: grid;
        gap: 16px;
    }

    .task-card:hover {
        transform: translateY(-4px);
        box-shadow: 0 20px 50px var(--shadow);
        border-color: var(--priority-color);
    }

    .task-card.active {
        box-shadow:
            0 0 0 2px var(--primary-color),
            0 20px 50px var(--shadow);
        border-left-color: var(--primary-color);
    }

    .task-card.completed {
        opacity: 0.6;
        filter: grayscale(0.3);
    }

    .card-top {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 10px;
        margin-bottom: 4px;
    }

    .badge {
        padding: 7px 14px;
        border-radius: 999px;
        border: 1px solid var(--border-color);
        font-weight: 700;
        font-size: 0.8rem;
        color: var(--text-secondary);
        background: var(--surface-color);
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .badge.priority {
        border-color: var(--priority-color);
        color: var(--text-color);
        background: transparent;
    }

    .badge.focus {
        border-color: var(--primary-color);
        color: var(--primary-color);
        background: var(--primary-color);
        color: white;
        box-shadow: 0 0 0 3px var(--shadow);
        animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
    }

    @keyframes pulse {
        0%,
        100% {
            opacity: 1;
        }
        50% {
            opacity: 0.8;
        }
    }

    .title-row {
        display: flex;
        justify-content: space-between;
        gap: 14px;
        align-items: center;
    }

    .title-group {
        display: flex;
        gap: 12px;
        align-items: center;
        flex: 1;
    }

    .title-group input[type="checkbox"] {
        width: 20px;
        height: 20px;
        cursor: pointer;
        accent-color: var(--primary-color);
    }

    .task-title {
        border: none;
        background: transparent;
        color: var(--text-color);
        font-weight: 700;
        font-size: 1.05rem;
        text-align: left;
        cursor: pointer;
        width: 100%;
        padding: 4px 0;
        line-height: 1.5;
        transition: color 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .task-title:hover {
        color: var(--primary-color);
    }

    .ghost.small {
        padding: 7px 12px;
        border-radius: 10px;
        border: 1px solid var(--border-color);
        background: transparent;
        cursor: pointer;
        color: var(--text-secondary);
        font-size: 0.85rem;
        font-weight: 600;
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .ghost.small:hover {
        background: var(--surface-color);
        color: var(--text-color);
        transform: translateY(-1px);
    }

    .meta-row {
        display: flex;
        flex-direction: column;
        gap: 14px;
        padding: 12px 0;
        border-top: 1px solid var(--border-color);
        border-bottom: 1px solid var(--border-color);
    }

    .tomato-actual {
        display: flex;
        gap: 8px;
        align-items: center;
        flex-wrap: wrap;
        font-weight: 700;
    }

    .tomato {
        transition: transform 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .tomato.filled {
        width: 20px;
        height: 20px;
        filter: drop-shadow(0 2px 4px var(--shadow));
    }

    .tomato.empty {
        font-size: 18px;
        opacity: 0.4;
        color: var(--text-secondary);
    }

    .ratio {
        margin-left: 8px;
        color: var(--text-secondary);
        font-size: 0.9rem;
        font-weight: 600;
    }

    .estimate-picker {
        display: flex;
        flex-wrap: wrap;
        gap: 10px;
    }

    .footer-row {
        display: flex;
        justify-content: space-between;
        gap: 12px;
        align-items: center;
        padding-top: 4px;
    }

    .priority-controls {
        display: flex;
        flex-wrap: wrap;
        gap: 8px;
    }

    .actions {
        display: flex;
        gap: 10px;
    }

    .actions button {
        border-radius: 12px;
        padding: 10px 16px;
        font-weight: 700;
        font-size: 0.9rem;
        border: 1px solid var(--border-color);
        cursor: pointer;
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .actions .primary {
        background: var(--primary-color);
        border-color: var(--primary-color);
        color: white;
    }

    .actions .primary:hover {
        background: var(--primary-light);
        box-shadow: 0 8px 20px var(--shadow);
        transform: translateY(-2px);
    }

    .actions .primary:active {
        transform: translateY(0);
    }

    .actions .ghost {
        background: transparent;
        color: var(--text-secondary);
    }

    .actions .ghost:hover {
        background: var(--surface-color);
        color: var(--text-color);
        transform: translateY(-1px);
    }

    .actions .danger {
        color: var(--error-color);
        border-color: var(--error-color);
    }

    .actions .danger:hover {
        background: var(--error-color);
        color: white;
    }

    .edit-input {
        width: 100%;
        padding: 10px 12px;
        border: 1px solid var(--border-color);
        border-radius: 10px;
        background: var(--surface-color);
        color: var(--text-color);
        font-size: 1rem;
        font-weight: 600;
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .edit-input:focus {
        outline: none;
        border-color: var(--primary-color);
        box-shadow: 0 0 0 3px var(--shadow);
    }

    .empty-state {
        color: var(--text-secondary);
        padding: 40px 20px;
        text-align: center;
        font-size: 1rem;
        font-weight: 500;
        opacity: 0.7;
    }

    @media (max-width: 640px) {
        .task-manager {
            padding: 20px;
        }

        .tasks-grid {
            grid-template-columns: 1fr;
        }

        .footer-row {
            flex-direction: column;
            align-items: flex-start;
            gap: 12px;
        }

        .priority-controls {
            width: 100%;
        }

        .actions {
            width: 100%;
            justify-content: space-between;
        }
    }
</style>
