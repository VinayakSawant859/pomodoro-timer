import { writable } from 'svelte/store';

export interface Toast {
    id: number;
    message: string;
    type?: 'info' | 'success' | 'warning' | 'error';
    duration?: number;
}

function createToastStore() {
    const { subscribe, update } = writable<Toast[]>([]);

    let nextId = 0;

    return {
        subscribe,
        show: (message: string, type: 'info' | 'success' | 'warning' | 'error' = 'info', duration = 3000) => {
            const id = nextId++;
            const toast: Toast = { id, message, type, duration };

            update(toasts => [...toasts, toast]);

            setTimeout(() => {
                update(toasts => toasts.filter(t => t.id !== id));
            }, duration);
        },
        dismiss: (id: number) => {
            update(toasts => toasts.filter(t => t.id !== id));
        }
    };
}

export const toastStore = createToastStore();
