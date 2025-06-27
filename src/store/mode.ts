import { defineStore } from 'pinia';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { CountdownConfig } from '@/model/countdown';

export const useModeStore = defineStore('mode', () => {
    const currentMode = ref('current');

    const switchMode = async (mode: string) => {
        currentMode.value = mode;
        try {
            const config = await invoke('get_countdown_config') as CountdownConfig;
            config.timeDisplayMode = mode;
            await invoke('update_countdown_config', { config });
        } catch (error) {
            console.error('Failed to switch mode:', error);
        }
    };

    const loadInitialMode = async () => {
        try {
            const config = await invoke('get_countdown_config') as CountdownConfig;
            currentMode.value = config.timeDisplayMode;
        } catch (error) {
            console.error('Failed to load initial mode:', error);
        }
    };

    return { currentMode, switchMode, loadInitialMode };
});