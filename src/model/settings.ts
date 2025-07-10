import type { WindowPosition } from './window'

// 窗口设置类型定义
export interface WindowSettings {
  theme: string
  window_position: WindowPosition
  opacity: number
  always_on_top: boolean
  accent_color: string
  recent_days: number
  default_startup: string
  monitor_index?: number
}

// AI设置类型定义
export interface AISettings {
  api_key: string
  base_url: string
  model: string
}



// 快捷键设置类型定义
export interface ShortcutSettings {
  toggle_window: string
  quick_add_todo: string
}