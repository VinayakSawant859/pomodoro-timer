import { writable } from 'svelte/store';
import type { Writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface Task {
    id: string;
    text: string;
    completed: boolean;
    createdAt: Date;
    completedAt?: Date;
}

export interface TimerSession {
    type: 'work' | 'break';
    duration: number; // in minutes
}

export interface TimerState {
    isRunning: boolean;
    isPaused: boolean;
    currentSession: TimerSession;
    timeRemaining: number; // in seconds
    sessionsCompleted: number;
    currentTaskId?: string;
}

// Timer Store
const createTimerStore = () => {
    const initialState: TimerState = {
        isRunning: false,
        isPaused: false,
        currentSession: { type: 'work', duration: 25 },
        timeRemaining: 25 * 60, // 25 minutes in seconds
        sessionsCompleted: 0,
        currentTaskId: undefined
    };

    const { subscribe, set, update } = writable(initialState);

    return {
        subscribe,
        start: (taskId?: string) => update(state => ({
            ...state,
            isRunning: true,
            isPaused: false,
            currentTaskId: taskId
        })),
        pause: () => update(state => ({
            ...state,
            isPaused: true
        })),
        resume: () => update(state => ({
            ...state,
            isPaused: false
        })),
        stop: () => update(state => ({
            ...state,
            isRunning: false,
            isPaused: false,
            currentTaskId: undefined
        })),
        tick: () => update(state => ({
            ...state,
            timeRemaining: Math.max(0, state.timeRemaining - 1)
        })),
        completeSession: () => update(state => {
            const newSessionType = state.currentSession.type === 'work' ? 'break' : 'work';
            const newDuration = newSessionType === 'work' ? 25 : 5; // Default durations

            return {
                ...state,
                sessionsCompleted: state.sessionsCompleted + 1,
                currentSession: { type: newSessionType, duration: newDuration },
                timeRemaining: newDuration * 60,
                isRunning: false,
                isPaused: false
            };
        }),
        setSession: (type: 'work' | 'break', duration: number) => update(state => ({
            ...state,
            currentSession: { type, duration },
            timeRemaining: duration * 60,
            isRunning: false,
            isPaused: false
        })),
        reset: () => set(initialState)
    };
};

// Audio Store
const createAudioStore = () => {
    const playSound = async (soundName: 'start' | 'complete' | 'stop') => {
        try {
            // Try Tauri audio first, fallback to web audio
            try {
                await invoke('play_sound', { soundName });
            } catch {
                // Fallback to web audio
                const audio = new Audio(`/timer-${soundName}.wav`);
                audio.volume = 0.5;
                await audio.play();
            }
        } catch (error) {
            console.warn('Failed to play sound:', error);
        }
    };

    return {
        playStart: () => playSound('start'),
        playComplete: () => playSound('complete'),
        playStop: () => playSound('stop')
    };
};

// Task Store
const createTaskStore = () => {
    const { subscribe, set, update } = writable<Task[]>([]);

    return {
        subscribe,
        load: async () => {
            try {
                const tasks = await invoke<Task[]>('get_tasks');
                set(tasks);
            } catch (error) {
                console.warn('Tauri not available, using local storage');
                // Fallback to localStorage
                const stored = localStorage.getItem('pomodoro-tasks');
                if (stored) {
                    set(JSON.parse(stored));
                }
            }
        },
        add: async (text: string) => {
            const task: Task = {
                id: crypto.randomUUID(),
                text,
                completed: false,
                createdAt: new Date()
            };
            
            try {
                const tauriTask = await invoke<Task>('add_task', { text });
                update(tasks => {
                    const newTasks = [tauriTask, ...tasks];
                    return newTasks;
                });
                return tauriTask;
            } catch (error) {
                // Fallback to localStorage
                update(tasks => {
                    const newTasks = [task, ...tasks];
                    localStorage.setItem('pomodoro-tasks', JSON.stringify(newTasks));
                    return newTasks;
                });
                return task;
            }
        },
        complete: async (id: string) => {
            try {
                await invoke('complete_task', { taskId: id, completed: true });
                update(tasks => {
                    const newTasks = tasks.map(t => 
                        t.id === id ? { ...t, completed: true, completedAt: new Date() } : t
                    );
                    return newTasks;
                });
            } catch (error) {
                // Fallback to localStorage
                update(tasks => {
                    const newTasks = tasks.map(t => 
                        t.id === id ? { ...t, completed: true, completedAt: new Date() } : t
                    );
                    localStorage.setItem('pomodoro-tasks', JSON.stringify(newTasks));
                    return newTasks;
                });
            }
        },
        uncomplete: async (id: string) => {
            try {
                await invoke('complete_task', { taskId: id, completed: false });
                update(tasks => {
                    const newTasks = tasks.map(t => 
                        t.id === id ? { ...t, completed: false, completedAt: undefined } : t
                    );
                    return newTasks;
                });
            } catch (error) {
                // Fallback to localStorage
                update(tasks => {
                    const newTasks = tasks.map(t => 
                        t.id === id ? { ...t, completed: false, completedAt: undefined } : t
                    );
                    localStorage.setItem('pomodoro-tasks', JSON.stringify(newTasks));
                    return newTasks;
                });
            }
        },
        updateText: async (id: string, text: string) => {
            try {
                await invoke('update_task', { taskId: id, text });
                update(tasks => {
                    const newTasks = tasks.map(t => 
                        t.id === id ? { ...t, text } : t
                    );
                    return newTasks;
                });
            } catch (error) {
                // Fallback to localStorage
                update(tasks => {
                    const newTasks = tasks.map(t => 
                        t.id === id ? { ...t, text } : t
                    );
                    localStorage.setItem('pomodoro-tasks', JSON.stringify(newTasks));
                    return newTasks;
                });
            }
        },
        remove: async (id: string) => {
            try {
                await invoke('delete_task', { taskId: id });
                update(tasks => {
                    const newTasks = tasks.filter(t => t.id !== id);
                    return newTasks;
                });
            } catch (error) {
                // Fallback to localStorage
                update(tasks => {
                    const newTasks = tasks.filter(t => t.id !== id);
                    localStorage.setItem('pomodoro-tasks', JSON.stringify(newTasks));
                    return newTasks;
                });
            }
        },
        set
    };
};

// Theme Store
const createThemeStore = () => {
    const { subscribe, set, update } = writable<'light' | 'dark'>('light');

    return {
        subscribe,
        toggle: () => update(theme => {
            const newTheme = theme === 'light' ? 'dark' : 'light';
            if (typeof window !== 'undefined') {
                localStorage.setItem('theme', newTheme);
                document.documentElement.setAttribute('data-theme', newTheme);
            }
            return newTheme;
        }),
        set: (theme: 'light' | 'dark') => {
            set(theme);
            if (typeof window !== 'undefined') {
                localStorage.setItem('theme', theme);
                document.documentElement.setAttribute('data-theme', theme);
            }
        },
        init: () => {
            if (typeof window !== 'undefined') {
                const stored = localStorage.getItem('theme') as 'light' | 'dark' | null;
                const theme = stored || 'light';
                set(theme);
                document.documentElement.setAttribute('data-theme', theme);
            }
        }
    };
};

export const timerStore = createTimerStore();
export const taskStore = createTaskStore();
export const themeStore = createThemeStore();
export const audioStore = createAudioStore();