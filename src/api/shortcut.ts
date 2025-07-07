import { api } from './index';
import type { ShortcutSettings } from '../model/settings';

/**
 * 快捷键相关 API 服务
 */
export class ShortcutApi {
  /**
   * 从数据库加载快捷键设置
   */
  static async loadSettings(): Promise<ShortcutSettings | null> {
    const response = await api.call<ShortcutSettings>('load_shortcut_settings_from_db');
    return response.success ? response.data || null : null;
  }

  /**
   * 保存快捷键设置到数据库
   */
  static async saveSettings(settings: ShortcutSettings): Promise<boolean> {
    const response = await api.call('save_shortcut_settings_to_db', { settings });
    return response.success;
  }

  /**
   * 注册全局快捷键
   */
  static async register(settings: ShortcutSettings): Promise<boolean> {
    const response = await api.call('register_global_shortcuts', { settings });
    return response.success;
  }
}

/**
 * 快捷键 API 的便捷导出
 */
export const shortcutApi = {
  load: ShortcutApi.loadSettings,
  save: ShortcutApi.saveSettings,
  register: ShortcutApi.register,
};