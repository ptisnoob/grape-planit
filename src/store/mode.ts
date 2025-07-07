import { defineStore } from 'pinia';
import { ref } from 'vue';
import { useDatabase } from '@/composables/useDatabase';

export const useModeStore = defineStore('mode', () => {
    const currentMode = ref('current');
    const { loadConfigFromDb, updateCountdownConfig } = useDatabase();

    const switchMode = async (mode: string) => {
        currentMode.value = mode;
        try {
            const config = await loadConfigFromDb();
            if (config) {
                config.timeDisplayMode = mode;
                await updateCountdownConfig(config);
            }
        } catch (error) {
            console.error('Failed to switch mode:', error);
        }
    };

    const loadInitialMode = async () => {
        try {
            const config = await loadConfigFromDb();
            if (config) {
                currentMode.value = config.timeDisplayMode;
            }
        } catch (error) {
            console.error('Failed to load initial mode:', error);
        }
    };

    return { currentMode, switchMode, loadInitialMode };
});