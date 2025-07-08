import { invoke } from '@tauri-apps/api/core'

export interface MotivationContent {
  content: string
}

export class MotivationApi {
  /**
   * 获取今日励志文案缓存
   */
  async getTodayCache(): Promise<MotivationContent | null> {
    try {
      const result = await invoke<MotivationContent | null>('get_today_motivation_cache')
      return result
    } catch (error) {
      console.error('获取励志文案缓存失败:', error)
      return null
    }
  }

  /**
   * 保存今日励志文案缓存
   */
  async saveTodayCache(content: string): Promise<boolean> {
    try {
      await invoke('save_today_motivation_cache', { content })
      return true
    } catch (error) {
      console.error('保存励志文案缓存失败:', error)
      return false
    }
  }

  /**
   * 清理过期的励志文案缓存
   */
  async cleanupCache(): Promise<boolean> {
    try {
      await invoke('cleanup_motivation_cache')
      return true
    } catch (error) {
      console.error('清理励志文案缓存失败:', error)
      return false
    }
  }
}

// 创建默认实例
export const motivationApi = new MotivationApi()