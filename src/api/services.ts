// 统一的 API 服务导出文件

import { aiApi } from './ai';
import { databaseApi } from './database';
import { api } from './index';
import { shortcutApi } from './shortcut';
import { todoApi } from './todo';
import { weatherApi } from './weather';
import { windowApi } from './window';

// 核心 API 客户端
export { api, ApiClient, ApiError } from './index';
export type { ApiResponse } from './index';

// 数据库 API
export { DatabaseApi, databaseApi } from './database';

// 待办事项 API
export { TodoApi, todoApi } from './todo';
export type { AddTodoParams, UpdateTodoParams } from './todo';

// 天气 API
export { WeatherApi, weatherApi } from './weather';

// AI API
export { AIApi, aiApi } from './ai';
export type { ChatMessage, ChatCompletionOptions, ChatCompletionResponse } from '@/model/ai';

// 窗口 API
export { WindowApi, windowApi } from './window';
export type { WindowPosition, WindowOpacity, WindowPositionConfig } from '@/model/window';

// 快捷键 API
export { ShortcutApi, shortcutApi } from './shortcut';
export type { ShortcutSettings } from '@/model/settings';

/**
 * 统一的 API 服务对象
 * 提供所有 API 服务的统一访问入口
 */
export const apiServices = {
  // 核心 API 客户端
  core: {
    call: api.call,
    callOrThrow: api.callOrThrow,
    silent: api.silent,
    retry: api.retry,
    timeout: api.timeout,
  },

  // 数据库服务
  database: databaseApi,

  // 待办事项服务
  todo: todoApi,

  // 天气服务
  weather: weatherApi,

  // AI 服务
  ai: aiApi,

  // 窗口服务
  window: windowApi,

  // 快捷键服务
  shortcut: shortcutApi,
};

/**
 * 默认导出统一的 API 服务
 */
export default apiServices;