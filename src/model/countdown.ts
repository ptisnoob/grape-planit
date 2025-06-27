// 类型定义

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
    loadConfigFromDb(): Promise<CountdownConfig>;
    saveConfigToDb(config: CountdownConfig): Promise<void>;
    saveCountdownRecord(
        mode: string,
        targetTime?: string,
        duration?: number
    ): Promise<void>;
    updateCountdownConfig(config: CountdownConfig): Promise<void>;
}