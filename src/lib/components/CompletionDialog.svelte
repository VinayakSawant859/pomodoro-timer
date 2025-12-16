<script lang="ts">
    import { timer, tasks } from "$lib/state.svelte";

    interface Props {
        taskId?: string;
        taskName?: string;
        onClose: () => void;
    }

    let { taskId, taskName, onClose }: Props = $props();

    let dialogElement: HTMLDialogElement;

    $effect(() => {
        if (dialogElement && taskId) {
            dialogElement.showModal();
        }
    });

    async function handleYes() {
        if (taskId) {
            await tasks.complete(taskId);
        }
        await startBreak();
        onClose();
    }

    async function handleNo() {
        // Just log the time, don't complete the task
        await startBreak();
        onClose();
    }

    async function startBreak() {
        // The session has already been completed in the timer
        // Just start the break timer automatically
        const newSessionType: "work" | "break" = "break";
        const newDuration = timer.sessionsCompleted % 4 === 0 ? 15 : 5; // Long break every 4th session
        timer.setSession(newSessionType, newDuration);
        await timer.start(); // Auto-start the break
    }

    function handleBackdropClick(e: MouseEvent) {
        // Prevent closing by clicking backdrop - user must choose
        e.stopPropagation();
    }
</script>

<dialog
    bind:this={dialogElement}
    class="completion-dialog"
    onclick={handleBackdropClick}
>
    <div class="dialog-content" onclick={(e) => e.stopPropagation()}>
        <div class="success-animation">
            <svg class="success-icon" viewBox="0 0 52 52">
                <circle
                    class="success-circle"
                    cx="26"
                    cy="26"
                    r="25"
                    fill="none"
                />
                <path
                    class="success-check"
                    fill="none"
                    d="M14.1 27.2l7.1 7.2 16.7-16.8"
                />
            </svg>
        </div>

        <div class="dialog-header">
            <h2>
                <img
                    src="/icons/goal.svg"
                    alt="Goal"
                    style="width: 28px; height: 28px; vertical-align: middle; margin-right: 8px;"
                />Session Complete!
            </h2>
        </div>

        <div class="dialog-body">
            {#if taskName}
                <p class="question">Did you finish</p>
                <p class="task-name">"{taskName}"?</p>
            {:else}
                <p class="question">Did you complete your work?</p>
            {/if}
        </div>

        <div class="dialog-actions">
            <button class="btn btn-success" onclick={handleYes}>
                <svg
                    class="icon"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                >
                    <polyline points="20,6 9,17 4,12"></polyline>
                </svg>
                Yes, Complete It
            </button>

            <button class="btn btn-secondary" onclick={handleNo}>
                <svg
                    class="icon"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                >
                    <path d="M18 6L6 18M6 6l12 12"></path>
                </svg>
                No, Continue Later
            </button>
        </div>

        <div class="dialog-footer">
            <p class="break-notice">
                <img
                    src="/icons/timer.svg"
                    alt="Timer"
                    style="width: 16px; height: 16px; vertical-align: middle; margin-right: 4px;"
                />Break timer will start automatically
            </p>
        </div>
    </div>
</dialog>

<style>
    .completion-dialog {
        border: none;
        border-radius: 1rem;
        padding: 0;
        max-width: 500px;
        width: 90%;
        box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
        background: var(--surface-color);
        backdrop-filter: blur(10px);
        animation: slideUp 0.3s ease-out;
    }

    @keyframes slideUp {
        from {
            opacity: 0;
            transform: translateY(20px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }

    .completion-dialog::backdrop {
        background: rgba(0, 0, 0, 0.7);
        backdrop-filter: blur(4px);
        animation: fadeIn 0.3s ease-out;
    }

    @keyframes fadeIn {
        from {
            opacity: 0;
        }
        to {
            opacity: 1;
        }
    }

    .dialog-content {
        padding: 2rem;
    }

    .success-animation {
        display: flex;
        justify-content: center;
        margin-bottom: 1rem;
    }

    .success-icon {
        width: 80px;
        height: 80px;
    }

    .success-circle {
        stroke: #10b981;
        stroke-width: 2;
        stroke-dasharray: 166;
        stroke-dashoffset: 166;
        animation: stroke 0.6s cubic-bezier(0.65, 0, 0.45, 1) forwards;
    }

    .success-check {
        stroke: #10b981;
        stroke-width: 2;
        stroke-linecap: round;
        stroke-dasharray: 48;
        stroke-dashoffset: 48;
        animation: stroke 0.3s cubic-bezier(0.65, 0, 0.45, 1) 0.8s forwards;
    }

    @keyframes stroke {
        100% {
            stroke-dashoffset: 0;
        }
    }

    .dialog-header {
        text-align: center;
        margin-bottom: 1.5rem;
    }

    .dialog-header h2 {
        font-size: 1.75rem;
        font-weight: 700;
        color: var(--text-primary);
        margin: 0;
    }

    .dialog-body {
        text-align: center;
        margin-bottom: 2rem;
    }

    .question {
        font-size: 1.1rem;
        color: var(--text-secondary);
        margin-bottom: 0.5rem;
    }

    .task-name {
        font-size: 1.3rem;
        font-weight: 600;
        color: var(--primary-color);
        margin: 0.5rem 0;
        padding: 0.5rem 1rem;
        background: var(--hover-bg);
        border-radius: 0.5rem;
        word-wrap: break-word;
    }

    .dialog-actions {
        display: flex;
        gap: 1rem;
        margin-bottom: 1rem;
    }

    .dialog-actions button {
        flex: 1;
        padding: 1rem 1.5rem;
        font-size: 1rem;
        font-weight: 600;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        transition: all 0.2s;
        border-radius: 0.75rem;
        cursor: pointer;
    }

    .btn-success {
        background: linear-gradient(135deg, #10b981, #059669);
        color: white;
        border: none;
        box-shadow: 0 4px 12px rgba(16, 185, 129, 0.3);
    }

    .btn-success:hover {
        transform: translateY(-2px);
        box-shadow: 0 6px 16px rgba(16, 185, 129, 0.4);
    }

    .btn-secondary {
        background: var(--surface-color);
        border: 2px solid var(--border-color);
        color: var(--text-secondary);
    }

    .btn-secondary:hover {
        border-color: var(--text-secondary);
        background: var(--hover-bg);
    }

    .btn-success:hover {
        background: linear-gradient(135deg, #059669, #047857);
        transform: translateY(-2px);
        box-shadow: 0 4px 12px rgba(16, 185, 129, 0.3);
    }

    .btn-secondary {
        background: var(--surface-color);
        color: var(--text-primary);
        border: 2px solid var(--border-color);
    }

    .btn-secondary:hover {
        background: var(--hover-bg);
        border-color: var(--primary-color);
        transform: translateY(-2px);
    }

    .icon {
        width: 20px;
        height: 20px;
        stroke-width: 2.5;
    }

    .dialog-footer {
        text-align: center;
        padding-top: 1rem;
        border-top: 1px solid var(--border-color);
    }

    .break-notice {
        font-size: 0.9rem;
        color: var(--text-secondary);
        font-style: italic;
        margin: 0;
    }

    /* Animations */
    .completion-dialog[open] {
        animation: slideIn 0.3s ease-out;
    }

    @keyframes slideIn {
        from {
            opacity: 0;
            transform: translateY(-20px) scale(0.95);
        }
        to {
            opacity: 1;
            transform: translateY(0) scale(1);
        }
    }

    /* Dark mode adjustments */
    @media (prefers-color-scheme: dark) {
        .completion-dialog {
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.6);
        }
    }
</style>
