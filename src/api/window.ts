import { api } from './index';
import type { WindowPosition, MonitorInfo } from '@/model/window';

/**
 * 窗口相关 API 服务
 */
export class WindowApi {
  /**
   * 隐藏窗口到系统托盘
   */
  static async hideToTray(): Promise<boolean> {
    const response = await api.call('hide_to_tray');
    return response.success;
  }

  /**
   * 显示设置窗口
   */
  static async showSettings(): Promise<boolean> {
    const response = await api.call('show_settings_window');
    return response.success;
  }

  /**
   * 切换窗口置顶状态
   */
  static async toggleAlwaysOnTop(): Promise<boolean> {
    const response = await api.call('toggle_always_on_top');
    return response.success;
  }

  /**
   * 设置窗口置顶
   */
  static async setAlwaysOnTop(alwaysOnTop: boolean): Promise<boolean> {
    const response = await api.call('set_always_on_top', { always_on_top: alwaysOnTop });
    return response.success;
  }

  /**
   * 切换窗口阴影/装饰
   */
  static async toggleShadow(): Promise<boolean> {
    const response = await api.call('toggle_shadow');
    return response.success;
  }

  /**
   * 设置窗口位置
   */
  static async setPosition(position: WindowPosition): Promise<boolean> {
    const response = await api.call('set_window_position', { position });
    return response.success;
  }

  /**
   * 设置主窗口位置
   */
  static async setMainWindowPosition(position: WindowPosition): Promise<boolean> {
    const response = await api.call('set_main_window_position', { position });
    return response.success;
  }

  /**
   * 设置窗口透明度
   */
  static async setOpacity(opacity: number): Promise<boolean> {
    const response = await api.call('set_window_opacity', { opacity });
    return response.success;
  }

  /**
   * 在主窗口中执行脚本
   */
  static async evalScript(script: string): Promise<boolean> {
    const response = await api.call('eval_script_in_main_window', { script });
    return response.success;
  }

  /**
   * 获取所有显示器信息
   */
  static async getMonitors(): Promise<MonitorInfo[]> {
    const response = await api.call<MonitorInfo[]>('get_monitors');
    return response.success ? (response.data || []) : [];
  }

  /**
   * 设置窗口到指定显示器的指定位置
   */
  static async setWindowMonitor(monitorIndex: number, position: WindowPosition): Promise<boolean> {
    const response = await api.call('set_window_monitor', { monitor_index: monitorIndex, position });
    return response.success;
  }
}

/**
 * 窗口 API 的便捷导出
 */
export const windowApi = {
  hideToTray: WindowApi.hideToTray,
  showSettings: WindowApi.showSettings,
  toggleAlwaysOnTop: WindowApi.toggleAlwaysOnTop,
  setAlwaysOnTop: WindowApi.setAlwaysOnTop,
  toggleShadow: WindowApi.toggleShadow,
  setPosition: WindowApi.setPosition,
  setMainWindowPosition: WindowApi.setMainWindowPosition,
  setOpacity: WindowApi.setOpacity,
  evalScript: WindowApi.evalScript,
  getMonitors: WindowApi.getMonitors,
  setWindowMonitor: WindowApi.setWindowMonitor,
};