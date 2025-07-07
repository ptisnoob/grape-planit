import { databaseApi } from '../api/services';
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
      const result = await databaseApi.countdown.load();
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
      const success = await databaseApi.countdown.save(config);
      if (!success) {
        throw new Error('保存配置失败');
      }
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
      const success = await databaseApi.countdown.update(config);
      if (!success) {
        throw new Error('更新配置失败');
      }
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
      const success = await databaseApi.countdown.saveRecord(mode, targetTime, duration);
      if (!success) {
        throw new Error('保存记录失败');
      }
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
      const success = await databaseApi.countdown.startTimer();
      if (!success) {
        throw new Error('启动定时器失败');
      }
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
      const result = await databaseApi.window.load();
      if (!result) {
        throw new Error('加载窗口设置失败');
      }
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
      const success = await databaseApi.window.save(settings);
      if (!success) {
        throw new Error('保存窗口设置失败');
      }
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