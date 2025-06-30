import { defineStore } from 'pinia';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { CountdownConfig } from '@/model/countdown';
import { useDatabase } from '@/composables/useDatabase';

export const useModeStore = defineStore('mode', () => {
    const currentMode = ref('current');
    const { loadConfigFromDb, updateCountdownConfig } = useDatabase();

    const switchMode = async (mode: string) => {
        currentMode.value = mode;
        try {
            const config = await loadConfigFromDb();
            config.timeDisplayMode = mode;
            await updateCountdownConfig(config);
        } catch (error) {
            console.error('Failed to switch mode:', error);
        }
    };

    const loadInitialMode = async () => {
        try {
            const config = await loadConfigFromDb();
            currentMode.value = config.timeDisplayMode;
        } catch (error) {
            console.error('Failed to load initial mode:', error);
        }
    };

    return { currentMode, switchMode, loadInitialMode };
});