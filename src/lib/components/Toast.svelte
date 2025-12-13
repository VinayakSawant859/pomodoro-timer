<script lang="ts">
    import { toastStore, type Toast } from "$lib/stores/toastStore";
    import { fade, fly } from "svelte/transition";

    let toasts = $state<Toast[]>([]);

    $effect(() => {
        const unsubscribe = toastStore.subscribe((value) => {
            toasts = value;
        });
        return unsubscribe;
    });
</script>

<div class="toast-container">
    {#each toasts as toast (toast.id)}
        <div
            class="toast toast-{toast.type}"
            transition:fly={{ y: 20, duration: 300 }}
        >
            <div class="toast-content">
                <div class="toast-icon">
                    {#if toast.type === "success"}
                        <svg
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                        >
                            <path d="M20 6L9 17l-5-5"></path>
                        </svg>
                    {:else if toast.type === "error"}
                        <svg
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                        >
                            <circle cx="12" cy="12" r="10"></circle>
                            <line x1="15" y1="9" x2="9" y2="15"></line>
                            <line x1="9" y1="9" x2="15" y2="15"></line>
                        </svg>
                    {:else if toast.type === "warning"}
                        <svg
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                        >
                            <path
                                d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"
                            ></path>
                            <line x1="12" y1="9" x2="12" y2="13"></line>
                            <line x1="12" y1="17" x2="12.01" y2="17"></line>
                        </svg>
                    {:else}
                        <svg
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                        >
                            <circle cx="12" cy="12" r="10"></circle>
                            <line x1="12" y1="16" x2="12" y2="12"></line>
                            <line x1="12" y1="8" x2="12.01" y2="8"></line>
                        </svg>
                    {/if}
                </div>
                <span class="toast-message">{toast.message}</span>
            </div>
            <button
                class="toast-close"
                onclick={() => toastStore.dismiss(toast.id)}
                aria-label="Close"
            >
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                    <line x1="18" y1="6" x2="6" y2="18"></line>
                    <line x1="6" y1="6" x2="18" y2="18"></line>
                </svg>
            </button>
        </div>
    {/each}
</div>

<style>
    .toast-container {
        position: fixed;
        bottom: 2rem;
        right: 2rem;
        z-index: 9999;
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        pointer-events: none;
    }

    .toast {
        pointer-events: auto;
        background: var(--surface-color);
        border: 1px solid var(--border-color);
        border-radius: 0.75rem;
        padding: 1rem 1.25rem;
        box-shadow: 0 8px 24px var(--shadow);
        display: flex;
        align-items: center;
        gap: 1rem;
        min-width: 300px;
        max-width: 500px;
        backdrop-filter: blur(10px);
    }

    .toast-content {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        flex: 1;
    }

    .toast-icon {
        flex-shrink: 0;
        width: 1.5rem;
        height: 1.5rem;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .toast-icon svg {
        width: 100%;
        height: 100%;
        stroke-width: 2;
    }

    .toast-success {
        border-left: 3px solid var(--success-color);
    }

    .toast-success .toast-icon {
        color: var(--success-color);
    }

    .toast-error {
        border-left: 3px solid var(--error-color);
    }

    .toast-error .toast-icon {
        color: var(--error-color);
    }

    .toast-warning {
        border-left: 3px solid var(--warning-color);
    }

    .toast-warning .toast-icon {
        color: var(--warning-color);
    }

    .toast-info {
        border-left: 3px solid var(--primary-color);
    }

    .toast-info .toast-icon {
        color: var(--primary-color);
    }

    .toast-message {
        color: var(--text-color);
        font-size: 0.95rem;
        line-height: 1.4;
    }

    .toast-close {
        flex-shrink: 0;
        background: transparent;
        border: none;
        color: var(--text-secondary);
        cursor: pointer;
        padding: 0.25rem;
        border-radius: 0.25rem;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: all 0.2s ease;
    }

    .toast-close:hover {
        background: var(--background-color);
        color: var(--text-color);
    }

    .toast-close svg {
        width: 1rem;
        height: 1rem;
        stroke-width: 2;
    }

    @media (max-width: 640px) {
        .toast-container {
            bottom: 1rem;
            right: 1rem;
            left: 1rem;
        }

        .toast {
            min-width: unset;
            max-width: unset;
        }
    }
</style>
