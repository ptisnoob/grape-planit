import { api } from './index';
import type { 
  AIConfig, 
  ChatMessage, 
  ChatCompletionOptions, 
  ChatCompletionResponse,
  AIServiceStatus 
} from '@/model/ai';

/**
 * AI 相关 API 服务
 */
export class AIApi {
  /**
   * 从数据库加载 AI 配置
   */
  static async loadAIConfig(): Promise<AIConfig | null> {
    const response = await api.call<AIConfig>('load_ai_settings_from_db');
    return response.success ? response.data || null : null;
  }

  /**
   * 保存 AI 配置到数据库
   */
  static async saveAIConfig(config: AIConfig): Promise<boolean> {
    const response = await api.call('save_ai_settings_to_db', config);
    return response.success;
  }

  /**
   * 更新 AI 配置（部分更新）
   */
  static async updateAIConfig(updates: Partial<AIConfig>): Promise<boolean> {
    // 先加载当前配置
    const currentConfig = await this.loadAIConfig();
    if (!currentConfig) {
      return false;
    }

    // 合并更新
    const newConfig = { ...currentConfig, ...updates };
    return this.saveAIConfig(newConfig);
  }

  /**
   * 检查 AI 配置是否完整
   */
  static async isAIConfigured(): Promise<boolean> {
    const config = await this.loadAIConfig();
    return !!(config?.api_key && config?.base_url && config?.model);
  }

  /**
   * 获取 AI 配置状态
   */
  static async getAIConfigStatus(): Promise<AIServiceStatus> {
    const config = await this.loadAIConfig();
    
    if (!config) {
      return {
        enabled: false,
        hasApiKey: false,
        hasBaseUrl: false,
        hasModel: false,
        isFullyConfigured: false,
      };
    }

    const hasApiKey = !!(config.api_key && config.api_key.trim());
    const hasBaseUrl = !!(config.base_url && config.base_url.trim());
    const hasModel = !!(config.model && config.model.trim());
    const isFullyConfigured = hasApiKey && hasBaseUrl && hasModel;

    return {
      enabled: isFullyConfigured,
      hasApiKey,
      hasBaseUrl,
      hasModel,
      isFullyConfigured,
    };
  }

  /**
   * 清空 AI 配置（相当于禁用）
   */
  static async clearAIConfig(): Promise<boolean> {
    return this.saveAIConfig({
      api_key: '',
      base_url: 'https://open.bigmodel.cn/api/paas/v4',
      model: 'glm-4-flash-250414'
    });
  }

  /**
   * 更新 API Key
   */
  static async updateApiKey(apiKey: string): Promise<boolean> {
    return this.updateAIConfig({ api_key: apiKey });
  }

  /**
   * 更新 Base URL
   */
  static async updateBaseUrl(baseUrl: string): Promise<boolean> {
    return this.updateAIConfig({ base_url: baseUrl });
  }

  /**
   * 更新模型
   */
  static async updateModel(model: string): Promise<boolean> {
    return this.updateAIConfig({ model });
  }

  /**
   * 测试 AI 连接
   */
  static async testAIConnection(): Promise<boolean> {
    const response = await api.call('test_ai_connection');
    return response.success;
  }

  /**
   * 发送聊天请求
   */
  static async chatCompletion(options: ChatCompletionOptions): Promise<ChatCompletionResponse | null> {
    const response = await api.call<ChatCompletionResponse>('ai_chat_completion', { options });
    return response.success ? response.data || null : null;
  }

  /**
   * 简单聊天
   */
  static async chat(
    prompt: string,
    systemPrompt?: string,
    options?: {
      temperature?: number;
      max_tokens?: number;
    }
  ): Promise<string | null> {
    const messages: ChatMessage[] = [];
    
    if (systemPrompt) {
      messages.push({ role: 'system', content: systemPrompt });
    }
    
    messages.push({ role: 'user', content: prompt });

    const chatOptions: ChatCompletionOptions = {
      messages,
      temperature: options?.temperature ?? 0.7,
      max_tokens: options?.max_tokens,
    };

    const result = await this.chatCompletion(chatOptions);
    return result?.choices?.[0]?.message?.content || null;
  }
}

/**
 * AI API 的便捷导出
 */
export const aiApi = {
  load: AIApi.loadAIConfig,
  save: AIApi.saveAIConfig,
  update: AIApi.updateAIConfig,
  isConfigured: AIApi.isAIConfigured,
  getStatus: AIApi.getAIConfigStatus,
  clear: AIApi.clearAIConfig,
  updateApiKey: AIApi.updateApiKey,
  updateBaseUrl: AIApi.updateBaseUrl,
  updateModel: AIApi.updateModel,
  testConnection: AIApi.testAIConnection,
  chatCompletion: AIApi.chatCompletion,
  chat: AIApi.chat,
};