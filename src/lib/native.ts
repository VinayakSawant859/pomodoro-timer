import { invoke } from '@tauri-apps/api/core';
import { sendNotification, isPermissionGranted, requestPermission } from '@tauri-apps/plugin-notification';

/**
 * Updates the window title and tray tooltip with timer status
 */
export async function updateStatus(text: string): Promise<void> {
    try {
        await invoke('update_status', { text });
    } catch (error) {
        console.error('Failed to update status:', error);
    }
}

/**
 * Shows a native desktop notification
 */
export async function showNotification(title: string, body: string): Promise<void> {
    try {
        let permissionGranted = await isPermissionGranted();
        
        if (!permissionGranted) {
            const permission = await requestPermission();
            permissionGranted = permission === 'granted';
        }
        
        if (permissionGranted) {
            await sendNotification({
                title,
                body,
            });
        }
    } catch (error) {
        console.error('Failed to show notification:', error);
    }
}

/**
 * Formats timer display for window title
 */
export function formatTimerTitle(minutes: number, seconds: number, sessionType: string): string {
    const timeStr = `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
    return `${timeStr} - ${sessionType}`;
}
