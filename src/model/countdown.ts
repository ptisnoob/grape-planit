// 类型定义



export interface CountdownConfig {
    workEndTime: string
    showSeconds: boolean
    timeDisplayMode: string
    enableWorkEndCountdown: boolean
    finalCountdownMinutes: number  // 进入最后倒计时的分钟数
    endStateKeepMinutes: number    // 结束状态保持的分钟数
    workDays: string
}

export interface CountdownData {
    mode: string
    timestamp: number
    target_info: string
    status: string
}

// 数据库记录接口
export interface CountdownRecord {
    id: number
    mode: string
    targetTime?: string
    duration?: number
    status: string
    createdAt: string
    finishedAt?: string
}

// 数据库操作结果接口
export interface DatabaseResult<T = any> {
    success: boolean
    data?: T
    error?: string
}

// Tauri命令接口
export interface DatabaseCommands {
  loadCountdownConfig(): Promise<CountdownConfig>;
  saveCountdownConfig(config: CountdownConfig): Promise<void>;
  saveCountdownRecord(record: CountdownRecord): Promise<void>;
  updateCountdownConfig(config: Partial<CountdownConfig>): Promise<void>;
}