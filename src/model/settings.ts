// 窗口设置类型定义
export interface WindowSettings {
  theme: string
  window_position: string
  opacity: number
  always_on_top: boolean
  accent_color: string
}

// AI设置类型定义
export interface AISettings {
  api_key: string
  base_url: string
  model: string
}

// 倒计时配置类型定义
export interface CustomCountdown {
  name: string
  target: string
}

export interface CountdownConfig {
  workEndTime: string
  customCountdown: CustomCountdown
  showSeconds: boolean
  timeDisplayMode: string
}