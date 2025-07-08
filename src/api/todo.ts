import { api } from './index';
import type { Todo } from '../model/todo';

/**
 * 添加待办事项的参数接口
 */
export interface AddTodoParams {
  title: string;
  startTime: number;
  endTime?: number | null;
  notes?: string | null;
  level: number;
  cycle: string;
}

/**
 * 更新待办事项的参数接口
 */
export interface UpdateTodoParams {
  id: number;
  title?: string;
  startTime?: number;
  endTime?: number | null;
  notes?: string | null;
  level?: number;
  cycle?: string;
  completed?: boolean;
}

/**
 * 待办事项相关 API 服务
 */
export class TodoApi {
  /**
   * 添加待办事项
   */
  static async addTodo(params: AddTodoParams): Promise<boolean> {
    const response = await api.call('add_todo', { params });
    return response.success;
  }

  /**
   * 获取所有待办事项
   */
  static async getAllTodos(): Promise<Todo[]> {
    const response = await api.call<Todo[]>('get_all_todos');
    return response.success ? response.data || [] : [];
  }

  /**
   * 获取最近的待办事项
   */
  static async getRecentTodos(days: number = 5): Promise<Todo[]> {
    const response = await api.call<Todo[]>('get_recent_todos', { days });
    return response.success ? response.data || [] : [];
  }

  /**
   * 更新待办事项
   */
  static async updateTodo(params: UpdateTodoParams): Promise<boolean> {
    const response = await api.call('update_todo', { params });
    return response.success;
  }

  /**
   * 根据ID获取待办事项
   */
  static async getTodoById(id: number): Promise<Todo | null> {
    const response = await api.call<Todo>('get_todo_by_id', { id });
    return response.success ? response.data || null : null;
  }

  /**
   * 删除待办事项（逻辑删除）
   */
  static async deleteTodo(id: number): Promise<boolean> {
    const response = await api.call('delete_todo', { id });
    return response.success;
  }

  /**
   * 标记待办事项为完成
   */
  static async completeTodo(id: number): Promise<boolean> {
    return this.updateTodo({ id, completed: true });
  }

  /**
   * 取消完成待办事项
   */
  static async uncompleteTodo(id: number): Promise<boolean> {
    return this.updateTodo({ id, completed: false });
  }

  /**
   * 切换待办事项完成状态
   */
  static async toggleTodoComplete(id: number, completed: boolean): Promise<boolean> {
    return this.updateTodo({ id, completed });
  }

  /**
   * 加载 TODO 颜色设置
   */
  static async loadColorSettings(): Promise<Record<string, string> | null> {
    const response = await api.call<Record<string, string>>('load_todo_color_settings');
    return response.success ? response.data || null : null;
  }

  /**
   * 保存 TODO 颜色设置
   */
  static async saveColorSettings(colors: Record<string, string>): Promise<boolean> {
    const response = await api.call('save_todo_color_settings', { colors });
    return response.success;
  }

  /**
   * 应用 TODO 颜色设置到主窗口
   */
  static async applyColorsToMainWindow(colors: Record<string, string>): Promise<boolean> {
    const response = await api.call('apply_todo_colors_to_main_window', { colors });
    return response.success;
  }
}

/**
 * 待办事项 API 的便捷导出
 */
export const todoApi = {
  add: TodoApi.addTodo,
  getAll: TodoApi.getAllTodos,
  getRecent: TodoApi.getRecentTodos,
  getById: TodoApi.getTodoById,
  update: TodoApi.updateTodo,
  delete: TodoApi.deleteTodo,
  complete: TodoApi.completeTodo,
  uncomplete: TodoApi.uncompleteTodo,
  toggleComplete: TodoApi.toggleTodoComplete,
  loadColorSettings: TodoApi.loadColorSettings,
  saveColorSettings: TodoApi.saveColorSettings,
  applyColorsToMainWindow: TodoApi.applyColorsToMainWindow,
};