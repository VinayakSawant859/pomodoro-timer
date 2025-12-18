<script lang="ts">
    interface Props {
        isOpen: boolean;
        title: string;
        message: string;
        confirmText?: string;
        cancelText?: string;
        onConfirm: () => void;
        onCancel: () => void;
        variant?: "danger" | "warning" | "info";
    }

    let {
        isOpen = $bindable(),
        title,
        message,
        confirmText = "Confirm",
        cancelText = "Cancel",
        onConfirm,
        onCancel,
        variant = "warning",
    }: Props = $props();

    function handleConfirm() {
        onConfirm();
        isOpen = false;
    }

    function handleCancel() {
        onCancel();
        isOpen = false;
    }

    function handleOverlayClick(e: MouseEvent) {
        if (e.target === e.currentTarget) {
            handleCancel();
        }
    }
</script>

{#if isOpen}
    <div
        class="dialog-overlay"
        onclick={handleOverlayClick}
        role="presentation"
    >
        <div
            class="dialog-modal"
            class:danger={variant === "danger"}
            class:warning={variant === "warning"}
            class:info={variant === "info"}
        >
            <div class="dialog-header">
                <h3>{title}</h3>
            </div>
            <div class="dialog-body">
                <p>{message}</p>
            </div>
            <div class="dialog-footer">
                <button class="btn btn-cancel" onclick={handleCancel}>
                    {cancelText}
                </button>
                <button
                    class="btn btn-confirm"
                    class:danger={variant === "danger"}
                    onclick={handleConfirm}
                >
                    {confirmText}
                </button>
            </div>
        </div>
    </div>
{/if}

<style>
    .dialog-overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: rgba(0, 0, 0, 0.6);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 2000;
        padding: 1rem;
        animation: fadeIn 0.2s ease;
    }

    @keyframes fadeIn {
        from {
            opacity: 0;
        }
        to {
            opacity: 1;
        }
    }

    .dialog-modal {
        background: var(--surface-color);
        border-radius: 1rem;
        max-width: 420px;
        width: 100%;
        box-shadow:
            0 20px 60px rgba(0, 0, 0, 0.3),
            0 8px 24px rgba(0, 0, 0, 0.15);
        border: 1px solid var(--border-color);
        animation: slideUp 0.25s cubic-bezier(0.4, 0, 0.2, 1);
        overflow: hidden;
    }

    @keyframes slideUp {
        from {
            opacity: 0;
            transform: translateY(20px) scale(0.95);
        }
        to {
            opacity: 1;
            transform: translateY(0) scale(1);
        }
    }

    .dialog-header {
        padding: 1.5rem 1.5rem 1rem;
        border-bottom: 1px solid var(--border-color);
    }

    .dialog-header h3 {
        margin: 0;
        font-size: 1.25rem;
        font-weight: 600;
        color: var(--text-color);
    }

    .dialog-body {
        padding: 1.5rem;
    }

    .dialog-body p {
        margin: 0;
        color: var(--text-secondary);
        font-size: 0.95rem;
        line-height: 1.6;
    }

    .dialog-footer {
        padding: 1rem 1.5rem 1.5rem;
        display: flex;
        gap: 0.75rem;
        justify-content: flex-end;
    }

    .btn {
        padding: 0.6rem 1.25rem;
        border-radius: 0.5rem;
        border: none;
        font-size: 0.95rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
        font-family: inherit;
    }

    .btn-cancel {
        background: var(--background-color);
        color: var(--text-color);
        border: 1px solid var(--border-color);
    }

    .btn-cancel:hover {
        background: var(--surface-color);
        transform: translateY(-1px);
        box-shadow: 0 2px 8px var(--shadow);
    }

    .btn-confirm {
        background: var(--primary-color);
        color: white;
    }

    .btn-confirm:hover {
        opacity: 0.9;
        transform: translateY(-1px);
        box-shadow: 0 4px 12px rgba(var(--primary-rgb), 0.3);
    }

    .btn-confirm.danger {
        background: #ef4444;
    }

    .btn-confirm.danger:hover {
        box-shadow: 0 4px 12px rgba(239, 68, 68, 0.3);
    }

    .btn:active {
        transform: translateY(0);
    }

    @media (max-width: 640px) {
        .dialog-modal {
            max-width: 100%;
            margin: 1rem;
        }

        .dialog-footer {
            flex-direction: column-reverse;
        }

        .btn {
            width: 100%;
        }
    }
</style>
