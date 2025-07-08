import { api } from './index';
import type { HolidayYear, StoredHolidayYear, SyncHolidayParams, ProxySettings, UpdateProxyParams } from '../model/holiday';

/**
 * 节假日相关 API 服务
 */
export class HolidayApi {
  /**
   * 同步指定年份的节假日数据
   */
  static async syncHolidayData(params: SyncHolidayParams): Promise<boolean> {
    const response = await api.call('sync_holiday_data', { params });
    return response.success;
  }

  /**
   * 获取已存储的节假日年份列表
   */
  static async getStoredHolidayYears(): Promise<StoredHolidayYear[]> {
    const response = await api.call<StoredHolidayYear[]>('get_stored_holiday_years');
    return response.success ? response.data || [] : [];
  }

  /**
   * 获取指定年份的节假日数据
   */
  static async getHolidaysByYear(year: number): Promise<HolidayYear | null> {
    const response = await api.call<HolidayYear>('get_holidays_by_year', { year });
    return response.success ? response.data || null : null;
  }

  /**
   * 删除指定年份的节假日数据
   */
  static async deleteHolidayYear(year: number): Promise<boolean> {
    const response = await api.call('delete_holiday_year', { year });
    return response.success;
  }

  /**
   * 获取代理配置
   */
  static async getProxySettings(): Promise<ProxySettings | null> {
    const response = await api.call<ProxySettings>('get_proxy_settings');
    return response.success ? response.data || null : null;
  }

  /**
   * 保存代理配置
   */
  static async saveProxySettings(params: UpdateProxyParams): Promise<boolean> {
    const response = await api.call('save_proxy_settings', { params });
    return response.success;
  }
}

/**
 * 节假日 API 的便捷导出
 */
export const holidayApi = {
  sync: HolidayApi.syncHolidayData,
  getStoredYears: HolidayApi.getStoredHolidayYears,
  getByYear: HolidayApi.getHolidaysByYear,
  deleteYear: HolidayApi.deleteHolidayYear,
  getProxySettings: HolidayApi.getProxySettings,
  saveProxySettings: HolidayApi.saveProxySettings,
};