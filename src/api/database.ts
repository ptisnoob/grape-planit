import { api } from './index';
import type { CountdownConfig } from '../model/countdown';
import type { WindowSettings } from '../model/settings';

/**
 * 数据库相关 API 服务
 */
export class DatabaseApi {
  /**
   * 从数据库加载倒计时配置
   */
  static async loadCountdownConfig(): Promise<CountdownConfig | null> {
    const response = await api.call<CountdownConfig>('load_countdown_config_from_db');
    return response.success ? response.data || null : null;
  }

  /**
   * 保存倒计时配置到数据库
   */
  static async saveCountdownConfig(config: CountdownConfig): Promise<boolean> {
    const response = await api.call('save_countdown_config_to_db', { config });
    return response.success;
  }

  /**
   * 更新倒计时配置（同时更新内存和数据库）
   */
  static async updateCountdownConfig(config: CountdownConfig): Promise<boolean> {
    const response = await api.call('update_countdown_config', { config });
    return response.success;
  }

  /**
   * 保存倒计时记录到数据库
   */
  static async saveCountdownRecord(
    mode: string,
    targetTime?: string,
    duration?: number
  ): Promise<boolean> {
    const response = await api.call('save_countdown_record', {
      mode,
      targetTime,
      duration,
    });
    return response.success;
  }

  /**
   * 启动倒计时定时器
   */
  static async startCountdownTimer(): Promise<boolean> {
    const response = await api.call('start_countdown_timer');
    return response.success;
  }

  /**
   * 重置下班倒计时到下一天
   */
  static async resetWorkEndToNextDay(): Promise<boolean> {
    const response = await api.call('reset_work_end_countdown_to_next_day');
    return response.success;
  }

  /**
   * 重置自定义倒计时
   */
  static async resetCustomCountdown(): Promise<boolean> {
    const response = await api.call('reset_custom_countdown');
    return response.success;
  }

  /**
   * 从数据库加载窗口设置
   */
  static async loadWindowSettings(): Promise<WindowSettings | null> {
    const response = await api.call<WindowSettings>('load_window_settings_from_db');
    return response.success ? response.data || null : null;
  }

  /**
   * 保存窗口设置到数据库
   */
  static async saveWindowSettings(settings: WindowSettings): Promise<boolean> {
    const response = await api.call('save_window_settings_to_db', { settings });
    return response.success;
  }
}

/**
 * 数据库 API 的便捷导出
 */
export const databaseApi = {
  countdown: {
    load: DatabaseApi.loadCountdownConfig,
    save: DatabaseApi.saveCountdownConfig,
    update: DatabaseApi.updateCountdownConfig,
    saveRecord: DatabaseApi.saveCountdownRecord,
    startTimer: DatabaseApi.startCountdownTimer,
    resetWorkEndToNextDay: DatabaseApi.resetWorkEndToNextDay,
    resetCustom: DatabaseApi.resetCustomCountdown,
  },
  window: {
    load: DatabaseApi.loadWindowSettings,
    save: DatabaseApi.saveWindowSettings,
  },
};