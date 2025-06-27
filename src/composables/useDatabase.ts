import { invoke } from '@tauri-apps/api/core';
import type { CountdownConfig, CountdownRecord } from '../model/countdown';

/**
 * 数据库操作的组合式函数
 */
export function useDatabase() {
  /**
   * 从数据库加载倒计时配置
   */
  const loadConfigFromDb = async (): Promise<CountdownConfig> => {
    try {
      return await invoke('load_config_from_db');
    } catch (error) {
      console.error('Failed to load config from database:', error);
      throw error;
    }
  };

  /**
   * 保存倒计时配置到数据库
   */
  const saveConfigToDb = async (config: CountdownConfig): Promise<void> => {
    try {
      await invoke('save_config_to_db', { config });
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
      await invoke('update_countdown_config', { config });
    } catch (error) {
      console.error('Failed to update countdown config:', error);
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

  return {
    loadConfigFromDb,
    saveConfigToDb,
    updateCountdownConfig,
    saveCountdownRecord,
    startCountdownTimer,
  };
}