/**
 * AI 配置接口
 */
export interface AIConfig {
  api_key: string;
  base_url: string;
  model: string;
}

/**
 * AI 聊天消息接口
 */
export interface ChatMessage {
  role: 'system' | 'user' | 'assistant' | 'function';
  content: string;
  name?: string;
}

/**
 * AI 聊天完成选项
 */
export interface ChatCompletionOptions {
  model?: string;
  temperature?: number;
  max_tokens?: number;
  stream?: boolean;
  messages: ChatMessage[];
}

/**
 * AI 聊天完成响应
 */
export interface ChatCompletionResponse {
  id: string;
  object: string;
  created: number;
  model: string;
  choices: {
    index: number;
    message: ChatMessage;
    finish_reason: string;
  }[];
  usage: {
    prompt_tokens: number;
    completion_tokens: number;
    total_tokens: number;
  };
}

/**
 * AI 服务状态
 */
export interface AIServiceStatus {
  enabled: boolean;
  hasApiKey: boolean;
  hasBaseUrl: boolean;
  hasModel: boolean;
  isFullyConfigured: boolean;
}