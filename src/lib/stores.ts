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

export interface SessionRecord {
    id: string;
    type: 'work' | 'short_break' | 'long_break';
    duration: number; // in minutes
    completed: boolean;
    started_at: string;
    completed_at?: string;
}

export interface DailySessionHistory {
    date: string;
    sessions: SessionRecord[];
    total_work_sessions: number;
    total_break_sessions: number;
    total_work_time: number;
    completion_rate: number;
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
    sessionNumber: number; // Current session number for the day
    dailySessionCount: number; // Total sessions completed today
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
        currentSessionId: undefined,
        sessionNumber: 1,
        dailySessionCount: 0
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

                    // Record session in daily history
                    if (!interrupted) {
                        sessionHistoryStore.addSession({
                            type: state.currentSession.type === 'work' ? 'work' :
                                (state.sessionsCompleted + 1) % 4 === 0 ? 'long_break' : 'short_break',
                            duration: state.currentSession.duration,
                            completed: true
                        });
                    }

                    const newSessionType: 'work' | 'break' = state.currentSession.type === 'work' ? 'break' : 'work';
                    const newDuration = newSessionType === 'work' ? 25 :
                        (state.sessionsCompleted + 1) % 4 === 0 ? 15 : 5; // Long break every 4th session

                    const newState = {
                        ...state,
                        sessionsCompleted: interrupted ? state.sessionsCompleted : state.sessionsCompleted + 1,
                        currentSession: { type: newSessionType, duration: newDuration },
                        timeRemaining: newDuration * 60,
                        isRunning: false,
                        isPaused: false,
                        currentSessionId: undefined,
                        sessionNumber: newSessionType === 'work' ? state.sessionNumber + 1 : state.sessionNumber,
                        dailySessionCount: interrupted ? state.dailySessionCount : state.dailySessionCount + 1
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
        reset: () => set(initialState),
        update
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
        loadToday: async function () {
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

// Session History Store
const createSessionHistoryStore = () => {
    const { subscribe, set, update } = writable<DailySessionHistory | null>(null);

    const getStorageKey = (date: string) => `pomodoro-sessions-${date}`;

    const getTodayKey = () => {
        const today = new Date().toISOString().split('T')[0];
        return getStorageKey(today);
    };

    return {
        subscribe,
        loadDaily: async (date: string) => {
            try {
                // Try to load from Tauri backend first
                const sessions = await invoke<SessionRecord[]>('get_daily_sessions', { date });

                const history: DailySessionHistory = {
                    date,
                    sessions,
                    total_work_sessions: sessions.filter(s => s.type === 'work' && s.completed).length,
                    total_break_sessions: sessions.filter(s => s.type !== 'work' && s.completed).length,
                    total_work_time: sessions
                        .filter(s => s.type === 'work' && s.completed)
                        .reduce((sum, s) => sum + s.duration, 0),
                    completion_rate: sessions.length > 0 ?
                        (sessions.filter(s => s.completed).length / sessions.length) * 100 : 0
                };

                set(history);
                return history;
            } catch (error) {
                // Fallback to localStorage
                const stored = localStorage.getItem(getStorageKey(date));
                if (stored) {
                    const history = JSON.parse(stored) as DailySessionHistory;
                    set(history);
                    return history;
                } else {
                    const emptyHistory: DailySessionHistory = {
                        date,
                        sessions: [],
                        total_work_sessions: 0,
                        total_break_sessions: 0,
                        total_work_time: 0,
                        completion_rate: 0
                    };
                    set(emptyHistory);
                    return emptyHistory;
                }
            }
        },

        loadToday: async function () {
            const today = new Date().toISOString().split('T')[0];
            return await this.loadDaily(today);
        },

        addSession: (sessionData: { type: 'work' | 'short_break' | 'long_break', duration: number, completed: boolean }) => {
            const now = new Date();
            const today = now.toISOString().split('T')[0];

            const session: SessionRecord = {
                id: crypto.randomUUID(),
                type: sessionData.type,
                duration: sessionData.duration,
                completed: sessionData.completed,
                started_at: now.toISOString(),
                completed_at: sessionData.completed ? now.toISOString() : undefined
            };

            // Try to save to Tauri backend
            try {
                invoke('add_session_record', { session }).catch(console.error);
            } catch (error) {
                console.log('Tauri not available, using localStorage');
            }

            update(currentHistory => {
                let history: DailySessionHistory;

                if (currentHistory && currentHistory.date === today) {
                    history = {
                        ...currentHistory,
                        sessions: [...currentHistory.sessions, session]
                    };
                } else {
                    // Load or create history for today
                    const stored = localStorage.getItem(getTodayKey());
                    if (stored) {
                        const storedHistory = JSON.parse(stored) as DailySessionHistory;
                        history = {
                            ...storedHistory,
                            sessions: [...storedHistory.sessions, session]
                        };
                    } else {
                        history = {
                            date: today,
                            sessions: [session],
                            total_work_sessions: 0,
                            total_break_sessions: 0,
                            total_work_time: 0,
                            completion_rate: 0
                        };
                    }
                }

                // Recalculate stats
                history.total_work_sessions = history.sessions.filter(s => s.type === 'work' && s.completed).length;
                history.total_break_sessions = history.sessions.filter(s => s.type !== 'work' && s.completed).length;
                history.total_work_time = history.sessions
                    .filter(s => s.type === 'work' && s.completed)
                    .reduce((sum, s) => sum + s.duration, 0);
                history.completion_rate = history.sessions.length > 0 ?
                    (history.sessions.filter(s => s.completed).length / history.sessions.length) * 100 : 0;

                // Save to localStorage as backup
                localStorage.setItem(getTodayKey(), JSON.stringify(history));

                return history;
            });
        },

        getWeeklyStats: () => {
            const weekData = [];
            for (let i = 6; i >= 0; i--) {
                const date = new Date();
                date.setDate(date.getDate() - i);
                const dateStr = date.toISOString().split('T')[0];

                const stored = localStorage.getItem(getStorageKey(dateStr));
                if (stored) {
                    const history = JSON.parse(stored) as DailySessionHistory;
                    weekData.push(history);
                } else {
                    weekData.push({
                        date: dateStr,
                        sessions: [],
                        total_work_sessions: 0,
                        total_break_sessions: 0,
                        total_work_time: 0,
                        completion_rate: 0
                    });
                }
            }
            return weekData;
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
export const sessionHistoryStore = createSessionHistoryStore();