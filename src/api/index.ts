import { invoke } from '@tauri-apps/api/core';

/**
 * API 响应包装器
 */
export interface ApiResponse<T = any> {
  success: boolean;
  data?: T;
  error?: string;
  code?: string;
}

/**
 * API 错误类
 */
export class ApiError extends Error {
  constructor(
    message: string,
    public code?: string,
    public originalError?: any
  ) {
    super(message);
    this.name = 'ApiError';
  }
}

/**
 * 统一的 API 调用器
 */
export class ApiClient {
  /**
   * 执行 Tauri 命令调用
   * @param command 命令名称
   * @param params 参数
   * @param options 选项
   */
  static async invoke<T = any>(
    command: string,
    params?: Record<string, any>,
    options?: {
      timeout?: number;
      retries?: number;
      silent?: boolean;
    }
  ): Promise<ApiResponse<T>> {
    const { timeout = 30000, retries = 0, silent = false } = options || {};
    
    let lastError: any;
    
    for (let attempt = 0; attempt <= retries; attempt++) {
      try {
        if (!silent) {
          console.log(`🔧 [API] 调用命令: ${command}`, params ? { params } : '');
        }
        
        // 创建超时 Promise
        const timeoutPromise = new Promise((_, reject) => {
          setTimeout(() => reject(new Error('API调用超时')), timeout);
        });
        
        // 执行 invoke 调用
        const invokePromise = params 
          ? invoke<T>(command, params)
          : invoke<T>(command);
        
        const result = await Promise.race([invokePromise, timeoutPromise]) as T;
        
        if (!silent) {
          console.log(`✅ [API] 命令 ${command} 执行成功:`, result);
        }
        
        return {
          success: true,
          data: result
        };
      } catch (error) {
        lastError = error;
        
        if (!silent) {
          console.error(`❌ [API] 命令 ${command} 执行失败 (尝试 ${attempt + 1}/${retries + 1}):`, error);
        }
        
        // 如果还有重试次数，等待一段时间后重试
        if (attempt < retries) {
          await new Promise(resolve => setTimeout(resolve, 1000 * (attempt + 1)));
        }
      }
    }
    
    // 所有重试都失败了
    const errorMessage = lastError?.message || '未知错误';
    const errorCode = lastError?.code || 'UNKNOWN_ERROR';
    
    return {
      success: false,
      error: errorMessage,
      code: errorCode
    };
  }
  
  /**
   * 执行 Tauri 命令调用（抛出异常版本）
   * @param command 命令名称
   * @param params 参数
   * @param options 选项
   */
  static async invokeOrThrow<T = any>(
    command: string,
    params?: Record<string, any>,
    options?: {
      timeout?: number;
      retries?: number;
      silent?: boolean;
    }
  ): Promise<T> {
    const response = await this.invoke<T>(command, params, options);
    
    if (!response.success) {
      throw new ApiError(
        response.error || '未知错误',
        response.code,
        response
      );
    }
    
    return response.data as T;
  }
}

/**
 * API 调用的便捷函数
 */
export const api = {
  /**
   * 安全调用（返回 ApiResponse）
   */
  call: ApiClient.invoke,
  
  /**
   * 抛出异常调用（直接返回数据或抛出异常）
   */
  callOrThrow: ApiClient.invokeOrThrow,
  
  /**
   * 静默调用（不打印日志）
   */
  silent: <T = any>(command: string, params?: Record<string, any>) => 
    ApiClient.invoke<T>(command, params, { silent: true }),
  
  /**
   * 带重试的调用
   */
  retry: <T = any>(command: string, params?: Record<string, any>, retries = 2) => 
    ApiClient.invoke<T>(command, params, { retries }),
  
  /**
   * 带超时的调用
   */
  timeout: <T = any>(command: string, params?: Record<string, any>, timeout = 10000) => 
    ApiClient.invoke<T>(command, params, { timeout })
};
