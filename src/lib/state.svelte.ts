import { invoke } from '@tauri-apps/api/core';

// ============= Type Interfaces (Keep all existing interfaces) =============

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

// ============= Timer State Class =============

export class TimerState {
    isRunning = $state(false);
    isPaused = $state(false);
    currentSession = $state<TimerSession>({ type: 'work', duration: 25 });
    timeRemaining = $state(25 * 60); // in seconds
    sessionsCompleted = $state(0);
    currentTaskId = $state<string | undefined>(undefined);
    currentSessionId = $state<string | undefined>(undefined);
    sessionNumber = $state(1);
    dailySessionCount = $state(0);
    sessionStartTime = $state<string | undefined>(undefined);

    async start(taskId?: string) {
        try {
            const session = await invoke<PomodoroSession>('start_pomodoro_session', {
                taskId,
                sessionType: 'work',
                durationMinutes: 25
            });

            this.isRunning = true;
            this.isPaused = false;
            this.currentTaskId = taskId;
            this.currentSessionId = session.id;
            this.sessionStartTime = new Date().toISOString();
        } catch (error) {
            console.error('Failed to start session:', error);
            // Fallback to local state
            this.isRunning = true;
            this.isPaused = false;
            this.currentTaskId = taskId;
            this.sessionStartTime = new Date().toISOString();
        }
    }

    pause() {
        this.isPaused = true;
    }

    resume() {
        this.isPaused = false;
    }

    async stop() {
        // Complete session in database as interrupted
        if (this.currentSessionId) {
            try {
                await invoke('complete_pomodoro_session', {
                    sessionId: this.currentSessionId,
                    interrupted: true
                });
            } catch (error) {
                console.error('Failed to complete session:', error);
            }
        }

        this.isRunning = false;
        this.isPaused = false;
        this.currentTaskId = undefined;
        this.currentSessionId = undefined;
    }

    tick() {
        this.timeRemaining = Math.max(0, this.timeRemaining - 1);
    }

    async completeSession(interrupted: boolean = false) {
        // Complete session in database
        if (this.currentSessionId) {
            try {
                await invoke('complete_pomodoro_session', {
                    sessionId: this.currentSessionId,
                    interrupted
                });
            } catch (error) {
                console.error('Failed to complete session:', error);
            }
        }

        // Auto-complete the task when a work session completes successfully
        if (!interrupted && this.currentSession.type === 'work' && this.currentTaskId) {
            console.log('Work session completed, auto-completing task:', this.currentTaskId);
            try {
                await tasks.complete(this.currentTaskId);
            } catch (err) {
                console.error('Failed to auto-complete task:', err);
            }
        }

        // Record session in daily history
        if (!interrupted) {
            sessionHistory.addSession({
                type: this.currentSession.type === 'work' ? 'work' :
                    (this.sessionsCompleted + 1) % 4 === 0 ? 'long_break' : 'short_break',
                duration: this.currentSession.duration,
                completed: true,
                startTime: this.sessionStartTime
            });
        }

        const newSessionType: 'work' | 'break' = this.currentSession.type === 'work' ? 'break' : 'work';
        const newDuration = newSessionType === 'work' ? 25 :
            (this.sessionsCompleted + 1) % 4 === 0 ? 15 : 5; // Long break every 4th session

        this.sessionsCompleted = interrupted ? this.sessionsCompleted : this.sessionsCompleted + 1;
        this.currentSession = { type: newSessionType, duration: newDuration };
        this.timeRemaining = newDuration * 60;
        this.isRunning = false;
        this.isPaused = false;
        this.currentSessionId = undefined;
        this.currentTaskId = undefined; // Clear current task after completion
        this.sessionNumber = newSessionType === 'work' ? this.sessionNumber + 1 : this.sessionNumber;
        this.dailySessionCount = interrupted ? this.dailySessionCount : this.dailySessionCount + 1;

        // Refresh stats after completing a session
        if (!interrupted) {
            console.log('Session completed, refreshing stats...');
            try {
                await stats.loadToday();
            } catch (err) {
                console.error('Failed to refresh stats:', err);
            }
        }
    }

    setSession(type: 'work' | 'break', duration: number) {
        this.currentSession = { type, duration };
        this.timeRemaining = duration * 60;
        this.isRunning = false;
        this.isPaused = false;
    }

    reset() {
        audio.playDelete();
        this.isRunning = false;
        this.isPaused = false;
        this.currentSession = { type: 'work', duration: 25 };
        this.timeRemaining = 25 * 60;
        this.sessionsCompleted = 0;
        this.currentTaskId = undefined;
        this.currentSessionId = undefined;
        this.sessionNumber = 1;
        this.dailySessionCount = 0;
        this.sessionStartTime = undefined;
    }
}

// ============= Audio State Class =============

export class AudioState {
    private async playSound(soundName: string) {
        try {
            // Try Tauri audio first, fallback to web audio
            try {
                await invoke('play_sound', { soundName });
            } catch {
                // Fallback to web audio
                const extension = soundName.includes('.mp3') ? '' : '.wav';
                const fileName = soundName.includes('.') ? soundName : `${soundName}${extension}`;
                const audio = new Audio(`/${fileName}`);
                audio.volume = 0.5;
                await audio.play();
            }
        } catch (error) {
            console.warn('Failed to play sound:', error);
        }
    }

    playStart() {
        return this.playSound('timer-start');
    }

    playComplete() {
        return this.playSound('timer-complete');
    }

    playStop() {
        return this.playSound('timer-stop');
    }

    playDelete() {
        return this.playSound('task-delete-timer-reset');
    }

    playTaskAdd() {
        return this.playSound('task-add');
    }

    playBreakStart() {
        return this.playSound('5min-break-start');
    }

    playBreakComplete() {
        return this.playSound('5min-break-complete');
    }
}

// ============= Task State Class =============

export class TaskState {
    tasks = $state<Task[]>([]);

    async load() {
        try {
            console.log('Loading tasks from database...');
            const loadedTasks = await invoke<Task[]>('get_tasks');
            console.log('Tasks loaded from database:', loadedTasks.length, 'tasks');
            console.log('Tasks:', loadedTasks);
            this.tasks = loadedTasks;
        } catch (error) {
            console.error('Failed to load tasks from database:', error);
            console.warn('Falling back to localStorage');
            // Fallback to localStorage
            const stored = localStorage.getItem('pomodoro-tasks');
            if (stored) {
                const loadedTasks = JSON.parse(stored);
                console.log('Fallback tasks loaded from localStorage:', loadedTasks.length, 'tasks');
                this.tasks = loadedTasks;
            } else {
                console.log('No tasks found in localStorage either');
                this.tasks = [];
            }
        }
    }

    async add(text: string): Promise<Task> {
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
            console.log('Adding task to database:', text);
            const tauriTask = await invoke<Task>('add_task', { text });
            console.log('Task added to database:', tauriTask);
            audio.playTaskAdd();
            this.tasks = [tauriTask, ...this.tasks];
            return tauriTask;
        } catch (error) {
            console.error('Failed to add task to database:', error);
            console.log('Falling back to localStorage for task:', text);
            audio.playTaskAdd();
            // Fallback to localStorage
            this.tasks = [task, ...this.tasks];
            localStorage.setItem('pomodoro-tasks', JSON.stringify(this.tasks));
            return task;
        }
    }

    async complete(id: string) {
        try {
            await invoke('complete_task', { taskId: id, completed: true });
            this.tasks = this.tasks.map(t =>
                t.id === id ? { ...t, completed: true, completed_at: new Date().toISOString() } : t
            );
            // Refresh stats after completing a task
            await stats.loadToday();
        } catch (error) {
            // Fallback to localStorage
            this.tasks = this.tasks.map(t =>
                t.id === id ? { ...t, completed: true, completed_at: new Date().toISOString() } : t
            );
            localStorage.setItem('pomodoro-tasks', JSON.stringify(this.tasks));
        }
    }

    async uncomplete(id: string) {
        try {
            await invoke('complete_task', { taskId: id, completed: false });
            this.tasks = this.tasks.map(t =>
                t.id === id ? { ...t, completed: false, completed_at: undefined } : t
            );
        } catch (error) {
            // Fallback to localStorage
            this.tasks = this.tasks.map(t =>
                t.id === id ? { ...t, completed: false, completed_at: undefined } : t
            );
            localStorage.setItem('pomodoro-tasks', JSON.stringify(this.tasks));
        }
    }

    async updateText(id: string, text: string) {
        try {
            await invoke('update_task', { taskId: id, text });
            this.tasks = this.tasks.map(t =>
                t.id === id ? { ...t, text } : t
            );
        } catch (error) {
            // Fallback to localStorage
            this.tasks = this.tasks.map(t =>
                t.id === id ? { ...t, text } : t
            );
            localStorage.setItem('pomodoro-tasks', JSON.stringify(this.tasks));
        }
    }

    async remove(id: string) {
        audio.playDelete();
        try {
            await invoke('delete_task', { taskId: id });
            this.tasks = this.tasks.filter(t => t.id !== id);
        } catch (error) {
            // Fallback to localStorage
            this.tasks = this.tasks.filter(t => t.id !== id);
            localStorage.setItem('pomodoro-tasks', JSON.stringify(this.tasks));
        }
    }
}

// ============= Theme State Class =============

type ThemeType = 'light' | 'dark' | 'academia' | 'sakura' | 'coffee' | 'forest' | 'flame' | 'anime';

export class ThemeState {
    current = $state<ThemeType>('light');

    toggle() {
        // Cycle through: light -> dark -> academia -> sakura -> coffee -> forest -> flame -> anime -> light
        const newTheme: ThemeType = this.current === 'light' ? 'dark' :
            this.current === 'dark' ? 'academia' :
                this.current === 'academia' ? 'sakura' :
                    this.current === 'sakura' ? 'coffee' :
                        this.current === 'coffee' ? 'forest' :
                            this.current === 'forest' ? 'flame' :
                                this.current === 'flame' ? 'anime' : 'light';

        this.set(newTheme);
    }

    set(theme: ThemeType) {
        this.current = theme;
        if (typeof window !== 'undefined') {
            localStorage.setItem('theme', theme);
            document.documentElement.setAttribute('data-theme', theme);
        }
    }

    init() {
        if (typeof window !== 'undefined') {
            const stored = localStorage.getItem('theme') as ThemeType | null;
            const theme = stored || 'light';
            this.current = theme;
            document.documentElement.setAttribute('data-theme', theme);
        }
    }
}

// ============= Font State Class =============

type FontType = 'default' | 'josefin' | 'cause' | 'cabin' | 'inconsolata' | 'poppins';

export class FontState {
    current = $state<FontType>('default');

    toggle() {
        // Cycle through: default -> josefin -> cause -> cabin -> inconsolata -> poppins -> default
        const newFont: FontType = this.current === 'default' ? 'josefin' :
            this.current === 'josefin' ? 'cause' :
                this.current === 'cause' ? 'cabin' :
                    this.current === 'cabin' ? 'inconsolata' :
                        this.current === 'inconsolata' ? 'poppins' : 'default';

        this.set(newFont);
    }

    set(font: FontType) {
        this.current = font;
        if (typeof window !== 'undefined') {
            localStorage.setItem('font', font);
            document.documentElement.setAttribute('data-font', font);
        }
    }

    init() {
        if (typeof window !== 'undefined') {
            const stored = localStorage.getItem('font') as FontType | null;
            const font = stored || 'default';
            this.current = font;
            document.documentElement.setAttribute('data-font', font);
        }
    }
}

// ============= Statistics State Class =============

export class StatsState {
    dailyStats = $state<DailyStats | null>(null);

    async loadDaily(date: string): Promise<DailyStats> {
        try {
            console.log('Loading daily stats for:', date);
            const stats = await invoke<DailyStats>('get_daily_stats', { date });
            console.log('Daily stats loaded:', stats);
            if (stats) {
                this.dailyStats = stats;
                return stats;
            } else {
                console.warn('Stats returned null, using empty stats');
                const emptyStats: DailyStats = {
                    date,
                    pomodoros_completed: 0,
                    total_work_time: 0,
                    tasks_completed: 0
                };
                this.dailyStats = emptyStats;
                return emptyStats;
            }
        } catch (error) {
            console.error('Failed to load daily stats:', error);
            // Fallback to empty stats
            const emptyStats: DailyStats = {
                date,
                pomodoros_completed: 0,
                total_work_time: 0,
                tasks_completed: 0
            };
            console.log('Using fallback stats due to error:', emptyStats);
            this.dailyStats = emptyStats;
            return emptyStats;
        }
    }

    async loadToday(): Promise<DailyStats> {
        const today = new Date().toISOString().split('T')[0];
        console.log('Loading today\'s stats for date:', today);
        return await this.loadDaily(today);
    }
}

// ============= Session Store Class =============

export class SessionStatsState {
    taskStats = $state<TaskWithStats | null>(null);

    async loadTaskStats(taskId: string): Promise<TaskWithStats | null> {
        try {
            const stats = await invoke<TaskWithStats>('get_task_with_stats', { taskId });
            this.taskStats = stats;
            return stats;
        } catch (error) {
            console.error('Failed to load task stats:', error);
            this.taskStats = null;
            return null;
        }
    }

    clear() {
        this.taskStats = null;
    }
}

// ============= Data Export/Import Class =============

export class DataState {
    async exportData(): Promise<string> {
        try {
            return await invoke<string>('export_data');
        } catch (error) {
            console.error('Failed to export data:', error);
            // Fallback to localStorage data
            const tasksData = localStorage.getItem('pomodoro-tasks') || '[]';
            const exportData = {
                tasks: JSON.parse(tasksData),
                exported_at: new Date().toISOString(),
                version: '2.0'
            };
            return JSON.stringify(exportData, null, 2);
        }
    }

    async importData(data: string): Promise<boolean> {
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
}

// ============= Session History State Class =============

export class SessionHistoryState {
    history = $state<DailySessionHistory | null>(null);

    private getStorageKey(date: string) {
        return `pomodoro-sessions-${date}`;
    }

    private getTodayKey() {
        const today = new Date().toISOString().split('T')[0];
        return this.getStorageKey(today);
    }

    async loadDaily(date: string): Promise<DailySessionHistory> {
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

            this.history = history;
            return history;
        } catch (error) {
            // Fallback to localStorage
            const stored = localStorage.getItem(this.getStorageKey(date));
            if (stored) {
                const history = JSON.parse(stored) as DailySessionHistory;
                this.history = history;
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
                this.history = emptyHistory;
                return emptyHistory;
            }
        }
    }

    async loadToday(): Promise<DailySessionHistory> {
        const today = new Date().toISOString().split('T')[0];
        return await this.loadDaily(today);
    }

    addSession(sessionData: { type: 'work' | 'short_break' | 'long_break', duration: number, completed: boolean, startTime?: string }) {
        const now = new Date();
        const today = now.toISOString().split('T')[0];
        const startTime = sessionData.startTime ? new Date(sessionData.startTime) : now;

        const session: SessionRecord = {
            id: crypto.randomUUID(),
            type: sessionData.type,
            duration: sessionData.duration,
            completed: sessionData.completed,
            started_at: startTime.toISOString(),
            completed_at: sessionData.completed ? now.toISOString() : undefined
        };

        // Try to save to Tauri backend
        try {
            invoke('add_session_record', { session }).catch(console.error);
        } catch (error) {
            console.log('Tauri not available, using localStorage');
        }

        let history: DailySessionHistory;

        if (this.history && this.history.date === today) {
            history = {
                ...this.history,
                sessions: [...this.history.sessions, session]
            };
        } else {
            // Load or create history for today
            const stored = localStorage.getItem(this.getTodayKey());
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
        localStorage.setItem(this.getTodayKey(), JSON.stringify(history));

        this.history = history;
    }

    getWeeklyStats(): DailySessionHistory[] {
        const weekData = [];
        for (let i = 6; i >= 0; i--) {
            const date = new Date();
            date.setDate(date.getDate() - i);
            const dateStr = date.toISOString().split('T')[0];

            const stored = localStorage.getItem(this.getStorageKey(dateStr));
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
}

// ============= Export Global Singleton Instances =============

export const timer = new TimerState();
export const audio = new AudioState();
export const tasks = new TaskState();
export const theme = new ThemeState();
export const font = new FontState();
export const stats = new StatsState();
export const sessionStats = new SessionStatsState();
export const dataExport = new DataState();
export const sessionHistory = new SessionHistoryState();
