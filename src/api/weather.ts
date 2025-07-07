import { api } from './index';
import type { WeatherSettings } from '../model/weather';

/**
 * 天气相关 API 服务
 */
export class WeatherApi {
  /**
   * 从数据库加载天气设置
   */
  static async loadWeatherSettings(): Promise<WeatherSettings | null> {
    const response = await api.call<WeatherSettings>('load_weather_settings_from_db');
    return response.success ? response.data || null : null;
  }

  /**
   * 保存天气设置到数据库
   */
  static async saveWeatherSettings(settings: WeatherSettings): Promise<boolean> {
    const response = await api.call('save_weather_settings_to_db', { settings });
    return response.success;
  }

  /**
   * 更新天气设置（部分更新）
   */
  static async updateWeatherSettings(updates: Partial<WeatherSettings>): Promise<boolean> {
    // 先加载当前设置
    const currentSettings = await this.loadWeatherSettings();
    if (!currentSettings) {
      return false;
    }

    // 合并更新
    const newSettings = { ...currentSettings, ...updates };
    return this.saveWeatherSettings(newSettings);
  }

  /**
   * 启用天气功能
   */
  static async enableWeather(): Promise<boolean> {
    return this.updateWeatherSettings({ enabled: true });
  }

  /**
   * 禁用天气功能
   */
  static async disableWeather(): Promise<boolean> {
    return this.updateWeatherSettings({ enabled: false });
  }

  /**
   * 切换天气功能开关
   */
  static async toggleWeather(): Promise<boolean> {
    const settings = await this.loadWeatherSettings();
    if (!settings) {
      return false;
    }
    return this.updateWeatherSettings({ enabled: !settings.enabled });
  }

  /**
   * 更新 API Key
   */
  static async updateApiKey(apiKey: string): Promise<boolean> {
    return this.updateWeatherSettings({ api_key: apiKey });
  }

  /**
   * 更新位置信息
   */
  static async updateLocation(locationData: {
    location_name?: string;
    latitude?: number | null;
    longitude?: number | null;
    adcode?: string | null;
    province?: string | null;
    city?: string | null;
    district?: string | null;
  }): Promise<boolean> {
    return this.updateWeatherSettings(locationData);
  }

  /**
   * 检查天气配置是否完整
   */
  static async isWeatherConfigured(): Promise<boolean> {
    const settings = await this.loadWeatherSettings();
    return !!(settings?.enabled && settings?.api_key && settings?.adcode);
  }

  /**
   * 获取天气配置状态
   */
  static async getWeatherConfigStatus(): Promise<{
    enabled: boolean;
    hasApiKey: boolean;
    hasLocation: boolean;
    isFullyConfigured: boolean;
  }> {
    const settings = await this.loadWeatherSettings();
    
    if (!settings) {
      return {
        enabled: false,
        hasApiKey: false,
        hasLocation: false,
        isFullyConfigured: false,
      };
    }

    const hasApiKey = !!(settings.api_key && settings.api_key.trim());
    const hasLocation = !!(settings.adcode);
    const isFullyConfigured = settings.enabled && hasApiKey && hasLocation;

    return {
      enabled: settings.enabled,
      hasApiKey,
      hasLocation,
      isFullyConfigured,
    };
  }
}

/**
 * 天气 API 的便捷导出
 */
export const weatherApi = {
  load: WeatherApi.loadWeatherSettings,
  save: WeatherApi.saveWeatherSettings,
  update: WeatherApi.updateWeatherSettings,
  enable: WeatherApi.enableWeather,
  disable: WeatherApi.disableWeather,
  toggle: WeatherApi.toggleWeather,
  updateApiKey: WeatherApi.updateApiKey,
  updateLocation: WeatherApi.updateLocation,
  isConfigured: WeatherApi.isWeatherConfigured,
  getStatus: WeatherApi.getWeatherConfigStatus,
};