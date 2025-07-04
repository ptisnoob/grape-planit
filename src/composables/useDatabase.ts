import { invoke } from '@tauri-apps/api/core';
import type { CountdownConfig } from '../model/countdown';
import type { WindowSettings } from '../model/settings';

/**
 * 数据库操作的组合式函数
 */
export function useDatabase() {
  /**
   * 从数据库加载倒计时配置
   */
  const loadConfigFromDb = async (): Promise<CountdownConfig | null> => {
    try {
      console.log('🔧 [useDatabase] 开始调用 load_countdown_config_from_db...');
      const result = await invoke<CountdownConfig>('load_countdown_config_from_db');
      console.log('🔧 [useDatabase] load_countdown_config_from_db 成功返回:', result);
      return result;
    } catch (error) {
      console.error('❌ [useDatabase] 从数据库加载配置失败:', error);
      throw error;
    }
  };

  /**
   * 保存倒计时配置到数据库
   */
  const saveConfigToDb = async (config: CountdownConfig): Promise<void> => {
    try {
      await invoke('save_countdown_config_to_db', { config });
    } catch (error) {
      console.error('Failed to save config to database:', error);
      throw error;
    }
  };

  /**
   * 更新倒计时配置（同时更新内存和数据库）
   */
  const updateCountdownConfig = async (config: CountdownConfig): Promise<void> => {
    try {
      console.log('🔧 [useDatabase] 开始调用 update_countdown_config...', config);
      await invoke('update_countdown_config', { config });
      console.log('🔧 [useDatabase] update_countdown_config 成功完成');
    } catch (error) {
      console.error('❌ [useDatabase] 更新倒计时配置失败:', error);
      throw error;
    }
  };

  /**
   * 保存倒计时记录到数据库
   */
  const saveCountdownRecord = async (
    mode: string,
    targetTime?: string,
    duration?: number
  ): Promise<void> => {
    try {
      await invoke('save_countdown_record', {
        mode,
        targetTime,
        duration,
      });
    } catch (error) {
      console.error('Failed to save countdown record:', error);
      throw error;
    }
  };

  /**
   * 启动倒计时定时器
   */
  const startCountdownTimer = async (): Promise<void> => {
    try {
      await invoke('start_countdown_timer');
    } catch (error) {
      console.error('Failed to start countdown timer:', error);
      throw error;
    }
  };

  /**
   * 从数据库加载窗口设置
   */
  const loadWindowSettings = async (): Promise<WindowSettings> => {
    try {
      const result = await invoke<WindowSettings>('load_window_settings_from_db');
      return result;
    } catch (error) {
      console.error('Failed to load window settings from database:', error);
      throw error;
    }
  };

  /**
   * 保存窗口设置到数据库
   */
  const saveWindowSettings = async (settings: WindowSettings): Promise<void> => {
    try {
      await invoke('save_window_settings_to_db', { settings });
    } catch (error) {
      console.error('Failed to save window settings to database:', error);
      throw error;
    }
  };

  return {
    loadConfigFromDb,
    saveConfigToDb,
    updateCountdownConfig,
    saveCountdownRecord,
    startCountdownTimer,
    loadWindowSettings,
    saveWindowSettings,
  };
}