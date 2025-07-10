/**
 * 窗口位置类型定义
 */
export type WindowPosition = 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right' | 'center';

/**
 * 窗口透明度范围
 */
export type WindowOpacity = number; // 0.0 - 1.0

/**
 * 显示器信息类型定义
 */
export interface MonitorInfo {
  name: string;
  index: number;
  size: [number, number];
  position: [number, number];
  is_primary: boolean;
}

/**
 * 窗口设置相关的类型定义
 */
export interface WindowPositionConfig {
  position: WindowPosition;
  opacity?: WindowOpacity;
  alwaysOnTop?: boolean;
}