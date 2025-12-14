# Svelte 5 Runes Migration Guide

## Overview
This document describes the refactoring of the Pomodoro Timer app from Svelte 4 stores to Svelte 5 runes-based state management.

## Architecture Changes

### Before (Svelte 4 Stores)
```typescript
// src/lib/stores.ts
const createTimerStore = () => {
    const { subscribe, set, update } = writable(initialState);
    return {
        subscribe,
        start: async (taskId?: string) => { ... },
        stop: () => { ... }
    };
};

export const timerStore = createTimerStore();
```

### After (Svelte 5 Runes)
```typescript
// src/lib/state.svelte.ts
export class TimerState {
    isRunning = $state(false);
    timeRemaining = $state(25 * 60);
    
    async start(taskId?: string) {
        this.isRunning = true;
    }
}

export const timer = new TimerState();
```

## Key Changes

### 1. State Declaration
**Before:**
```typescript
const { subscribe, set, update } = writable<Type>(initialValue);
```

**After:**
```typescript
propertyName = $state<Type>(initialValue);
```

### 2. Computed Values
**Before:**
```typescript
$: computedValue = $store.property * 2;
```

**After:**
```typescript
const computedValue = $derived(state.property * 2);
```

### 3. Component Usage
**Before:**
```svelte
<script>
import { timerStore } from '$lib/stores';
</script>

{#if $timerStore.isRunning}
    <p>{$timerStore.timeRemaining}</p>
{/if}
```

**After:**
```svelte
<script>
import { timer } from '$lib/state.svelte';
</script>

{#if timer.isRunning}
    <p>{timer.timeRemaining}</p>
{/if}
```

### 4. Props Declaration
**Before:**
```svelte
<script>
export let onStartTask: (() => void) | undefined = undefined;
</script>
```

**After:**
```svelte
<script>
interface Props {
    onStartTask?: () => void;
}

let { onStartTask }: Props = $props();
</script>
```

## Refactored Components

### Core State Classes

1. **TimerState** (`src/lib/state.svelte.ts`)
   - Properties: `isRunning`, `isPaused`, `currentSession`, `timeRemaining`, etc.
   - Methods: `start()`, `stop()`, `pause()`, `resume()`, `tick()`, `completeSession()`
   - No more store subscriptions - direct property access

2. **TaskState** (`src/lib/state.svelte.ts`)
   - Properties: `tasks` (array of Task objects)
   - Methods: `load()`, `add()`, `complete()`, `uncomplete()`, `updateText()`, `remove()`
   - Maintains Tauri integration with localStorage fallback

3. **ThemeState** (`src/lib/state.svelte.ts`)
   - Properties: `current` (theme name)
   - Methods: `toggle()`, `set()`, `init()`
   - Persists to localStorage automatically

4. **FontState** (`src/lib/state.svelte.ts`)
   - Properties: `current` (font name)
   - Methods: `toggle()`, `set()`, `init()`

5. **StatsState** (`src/lib/state.svelte.ts`)
   - Properties: `dailyStats`
   - Methods: `loadDaily()`, `loadToday()`

6. **SessionHistoryState** (`src/lib/state.svelte.ts`)
   - Properties: `history`
   - Methods: `loadDaily()`, `loadToday()`, `addSession()`, `getWeeklyStats()`

7. **AudioState** (`src/lib/state.svelte.ts`)
   - Methods: `playStart()`, `playComplete()`, `playStop()`, etc.
   - No reactive properties (stateless utility class)

### Updated Components

1. **Timer.svelte**
   - Changed: `$timerStore` → `timer`
   - Changed: `timerStore.method()` → `timer.method()`
   - Uses `$derived` for computed values

2. **TaskManager.svelte**
   - Changed: `$taskStore` → `tasks.tasks`
   - Changed: `taskStore.method()` → `tasks.method()`
   - Updated to use `$props()` instead of `export let`
   - Local state uses `$state()`

3. **ThemeToggle.svelte**
   - Changed: `$themeStore` → `theme.current`
   - Changed: `themeStore.method()` → `theme.method()`

4. **FontToggle.svelte**
   - Changed: `$fontStore` → `font.current`
   - Changed: `fontStore.method()` → `font.method()`

5. **Statistics.svelte**
   - Changed: `$statsStore` → `stats.dailyStats`
   - Changed: `$sessionHistoryStore` → `sessionHistory.history`

6. **SessionProgress.svelte**
   - Changed: `sessionHistoryStore` → `sessionHistory`
   - Uses `$derived` for reactive history access
   - Uses `$effect` to watch timer changes

7. **+page.svelte**
   - Updated imports to use new state module
   - Changed: `data-theme={$themeStore}` → `data-theme={theme.current}`
   - Direct property assignment instead of store update functions

## Benefits of Runes-Based Approach

1. **Simpler Mental Model**: Direct property access instead of store subscriptions
2. **Better TypeScript Support**: Class-based approach provides better type inference
3. **Less Boilerplate**: No need for subscribe/unsubscribe patterns
4. **Fine-Grained Reactivity**: Svelte 5 compiler optimizes reactivity automatically
5. **Cleaner Code**: Remove `$` prefix for most reactive values (except runes themselves)
6. **Better Performance**: Svelte 5's compiler generates more efficient code

## Migration Checklist

- [x] Create `src/lib/state.svelte.ts` with class-based state
- [x] Export singleton instances of all state classes
- [x] Update `Timer.svelte` to use new state
- [x] Update `TaskManager.svelte` to use new state
- [x] Update `ThemeToggle.svelte` to use new state
- [x] Update `FontToggle.svelte` to use new state  
- [x] Update `Statistics.svelte` to use new state
- [x] Update `SessionProgress.svelte` to use new state
- [x] Update `+page.svelte` main layout
- [x] Replace deprecated event handlers (`on:click` → `onclick`)
- [x] Update local component state to use `$state()`
- [x] Update computed values to use `$derived`
- [x] Update props to use `$props()`
- [x] Preserve all Tauri backend integration
- [x] Preserve all localStorage fallbacks
- [x] Preserve all audio functionality

## Testing Recommendations

1. Test timer start/stop/pause functionality
2. Test task CRUD operations (add, edit, complete, delete)
3. Test theme switching and persistence
4. Test font switching and persistence
5. Test statistics loading and display
6. Test session history tracking
7. Test Tauri backend communication
8. Test localStorage fallbacks when Tauri unavailable
9. Test audio playback for different events
10. Test session completion and auto-task-completion flow

## Backwards Compatibility

The old `src/lib/stores.ts` file is still present but no longer used. It can be safely removed after testing confirms the new implementation works correctly.

## Known Issues / Warnings

Some accessibility and deprecated event handler warnings remain in the codebase:
- `on:click` → `onclick` migrations completed for most components
- Accessibility labels added where needed
- Some a11y warnings for keyboard event handlers can be addressed incrementally

## Future Enhancements

1. Consider extracting Toast and Dropdown stores to runes
2. Consider adding more derived values for complex computations
3. Add unit tests for state classes
4. Consider extracting audio state to its own module
5. Add JSDoc comments for better IDE support
