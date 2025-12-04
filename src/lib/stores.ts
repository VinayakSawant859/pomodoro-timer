import { writable } from 'svelte/store';
import type { Writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface Task {
    id: string;
    text: string;
    completed: boolean;
    created_at: string;
    completed_at?: string;
    priority: number;
    estimated_pomodoros: number;
    actual_pomodoros: number;
}

export interface PomodoroSession {
    id: string;
    task_id?: string;
    session_type: 'work' | 'short_break' | 'long_break';
    duration_minutes: number;
    started_at: string;
    completed_at?: string;
    interrupted: boolean;
}

export interface DailyStats {
    date: string;
    pomodoros_completed: number;
    total_work_time: number;
    tasks_completed: number;
}

export interface TaskWithStats {
    task: Task;
    pomodoro_sessions: PomodoroSession[];
    total_time_spent: number;
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
    currentSessionId?: string;
}

// Timer Store
const createTimerStore = () => {
    const initialState: TimerState = {
        isRunning: false,
        isPaused: false,
        currentSession: { type: 'work', duration: 25 },
        timeRemaining: 25 * 60, // 25 minutes in seconds
        sessionsCompleted: 0,
        currentTaskId: undefined,
        currentSessionId: undefined
    };

    const { subscribe, set, update } = writable(initialState);

    return {
        subscribe,
        start: async (taskId?: string) => {
            try {
                const session = await invoke<PomodoroSession>('start_pomodoro_session', {
                    taskId,
                    sessionType: 'work',
                    durationMinutes: 25
                });

                update(state => ({
                    ...state,
                    isRunning: true,
                    isPaused: false,
                    currentTaskId: taskId,
                    currentSessionId: session.id
                }));
            } catch (error) {
                console.error('Failed to start session:', error);
                // Fallback to local state
                update(state => ({
                    ...state,
                    isRunning: true,
                    isPaused: false,
                    currentTaskId: taskId
                }));
            }
        },
        pause: () => update(state => ({
            ...state,
            isPaused: true
        })),
        resume: () => update(state => ({
            ...state,
            isPaused: false
        })),
        stop: async () => {
            return new Promise<void>((resolve) => {
                update(state => {
                    // Complete session in database as interrupted
                    if (state.currentSessionId) {
                        invoke('complete_pomodoro_session', {
                            sessionId: state.currentSessionId,
                            interrupted: true
                        }).catch(console.error);
                    }

                    const newState = {
                        ...state,
                        isRunning: false,
                        isPaused: false,
                        currentTaskId: undefined,
                        currentSessionId: undefined
                    };

                    resolve();
                    return newState;
                });
            });
        },
        tick: () => update(state => ({
            ...state,
            timeRemaining: Math.max(0, state.timeRemaining - 1)
        })),
        completeSession: async (interrupted: boolean = false) => {
            return new Promise<void>((resolve) => {
                update(state => {
                    // Complete session in database
                    if (state.currentSessionId) {
                        invoke('complete_pomodoro_session', {
                            sessionId: state.currentSessionId,
                            interrupted
                        }).catch(console.error);
                    }

                    const newSessionType: 'work' | 'break' = state.currentSession.type === 'work' ? 'break' : 'work';
                    const newDuration = newSessionType === 'work' ? 25 : 5; // Default durations

                    const newState = {
                        ...state,
                        sessionsCompleted: interrupted ? state.sessionsCompleted : state.sessionsCompleted + 1,
                        currentSession: { type: newSessionType, duration: newDuration },
                        timeRemaining: newDuration * 60,
                        isRunning: false,
                        isPaused: false,
                        currentSessionId: undefined
                    };

                    resolve();
                    return newState;
                });
                
                // Refresh stats after completing a session
                if (!interrupted) {
                    statsStore.loadToday();
                }
            });
        },
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
                console.log('Loading tasks from database...');
                const tasks = await invoke<Task[]>('get_tasks');
                console.log('Tasks loaded:', tasks.length, 'tasks');
                set(tasks);
            } catch (error) {
                console.warn('Tauri not available, using local storage');
                // Fallback to localStorage
                const stored = localStorage.getItem('pomodoro-tasks');
                if (stored) {
                    const tasks = JSON.parse(stored);
                    console.log('Fallback tasks loaded:', tasks.length, 'tasks');
                    set(tasks);
                }
            }
        },
        add: async (text: string) => {
            const task: Task = {
                id: crypto.randomUUID(),
                text,
                completed: false,
                created_at: new Date().toISOString(),
                priority: 0,
                estimated_pomodoros: 1,
                actual_pomodoros: 0
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
                        t.id === id ? { ...t, completed: true, completed_at: new Date().toISOString() } : t
                    );
                    return newTasks;
                });
                // Refresh stats after completing a task
                statsStore.loadToday();
            } catch (error) {
                // Fallback to localStorage
                update(tasks => {
                    const newTasks = tasks.map(t => 
                        t.id === id ? { ...t, completed: true, completed_at: new Date().toISOString() } : t
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
                        t.id === id ? { ...t, completed: false, completed_at: undefined } : t
                    );
                    return newTasks;
                });
            } catch (error) {
                // Fallback to localStorage
                update(tasks => {
                    const newTasks = tasks.map(t =>
                        t.id === id ? { ...t, completed: false, completed_at: undefined } : t
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

// Statistics Store
const createStatsStore = () => {
    const { subscribe, set, update } = writable<DailyStats | null>(null);

    return {
        subscribe,
        loadDaily: async (date: string) => {
            try {
                console.log('Loading daily stats for:', date);
                const stats = await invoke<DailyStats>('get_daily_stats', { date });
                console.log('Daily stats loaded:', stats);
                set(stats);
                return stats;
            } catch (error) {
                console.error('Failed to load daily stats:', error);
                // Fallback to empty stats
                const emptyStats: DailyStats = {
                    date,
                    pomodoros_completed: 0,
                    total_work_time: 0,
                    tasks_completed: 0
                };
                console.log('Using fallback stats:', emptyStats);
                set(emptyStats);
                return emptyStats;
            }
        },
        loadToday: async function() {
            const today = new Date().toISOString().split('T')[0];
            return await this.loadDaily(today);
        }
    };
};

// Session Store
const createSessionStore = () => {
    const { subscribe, set, update } = writable<TaskWithStats | null>(null);

    return {
        subscribe,
        loadTaskStats: async (taskId: string) => {
            try {
                const stats = await invoke<TaskWithStats>('get_task_with_stats', { taskId });
                set(stats);
                return stats;
            } catch (error) {
                console.error('Failed to load task stats:', error);
                set(null);
                return null;
            }
        },
        clear: () => set(null)
    };
};

// Export/Import Store
const createDataStore = () => {
    return {
        exportData: async (): Promise<string> => {
            try {
                return await invoke<string>('export_data');
            } catch (error) {
                console.error('Failed to export data:', error);
                // Fallback to localStorage data
                const tasks = localStorage.getItem('pomodoro-tasks') || '[]';
                const exportData = {
                    tasks: JSON.parse(tasks),
                    exported_at: new Date().toISOString(),
                    version: '2.0'
                };
                return JSON.stringify(exportData, null, 2);
            }
        },
        importData: async (data: string): Promise<boolean> => {
            try {
                const parsed = JSON.parse(data);
                // For now, we'll handle import through the task store
                // This could be enhanced to import sessions and stats too
                return true;
            } catch (error) {
                console.error('Failed to import data:', error);
                return false;
            }
        }
    };
};

export const timerStore = createTimerStore();
export const taskStore = createTaskStore();
export const themeStore = createThemeStore();
export const audioStore = createAudioStore();
export const statsStore = createStatsStore();
export const sessionStore = createSessionStore();
export const dataStore = createDataStore();