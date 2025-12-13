import { writable } from 'svelte/store';

type DropdownType = 'theme' | 'font' | null;

function createDropdownStore() {
    const { subscribe, set } = writable<DropdownType>(null);

    return {
        subscribe,
        open: (dropdown: DropdownType) => {
            set(dropdown);
        },
        close: () => {
            set(null);
        }
    };
}

export const dropdownStore = createDropdownStore();
