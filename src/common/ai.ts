import { dailyMotivationalQuotePrompt, ExtractionTaskPrompt } from "./prompt"
import { GDate } from "./date"
import { TodoVo } from '@/model/todo';
import { aiApi } from '@/api/services';
import type { 
    ChatMessage, 
    ChatCompletionOptions, 
    ChatCompletionResponse 
} from '@/model/ai';

// AI服务内部配置（用于本地实例）
export interface AIServiceConfig {
    baseUrl: string;
    apiKey: string;
    model: string;
}

/**
 * AI服务类
 * 提供与OpenAI API或兼容服务的交互功能
 */
export class AIService {
    private config: AIServiceConfig;

    constructor(config?: Partial<AIServiceConfig>) {
        // 从环境变量或传入参数获取配置
        this.config = {
            baseUrl: config?.baseUrl || import.meta.env.VITE_OPENAI_BASE_URL || 'https://open.bigmodel.cn/api/paas/v4',
            apiKey: config?.apiKey || import.meta.env.VITE_OPENAI_API_KEY || '',
            model: config?.model || import.meta.env.VITE_OPENAI_MODEL || 'glm-4-flash-250414',
        };

        // 验证配置
        if (!this.config.apiKey) {
            console.warn('AI服务初始化警告: 未提供API密钥，请检查.env文件或传入配置');
        }
    }

    /**
     * 创建聊天完成
     * @param options 聊天完成选项
     * @returns 聊天完成响应
     */
    async createChatCompletion(options: ChatCompletionOptions): Promise<ChatCompletionResponse> {
        const url = `${this.config.baseUrl}/chat/completions`;
        const model = options.model || this.config.model;

        try {
            const response = await fetch(url, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${this.config.apiKey}`,
                },
                body: JSON.stringify({
                    model,
                    messages: options.messages,
                    temperature: options.temperature ?? 0.7,
                    max_tokens: options.max_tokens,
                    stream: options.stream ?? false,
                }),
            });

            if (!response.ok) {
                const errorData = await response.json().catch(() => ({}));
                throw new Error(`AI API请求失败: ${response.status} ${response.statusText}\n${JSON.stringify(errorData)}`);
            }

            return await response.json();
        } catch (error) {
            console.error('AI聊天完成请求失败:', error);
            throw error;
        }
    }

    /**
     * 创建流式聊天完成
     * @param options 聊天完成选项
     * @returns 响应流
     */
    async createStreamChatCompletion(options: ChatCompletionOptions,
        onMessage: (message: string) => void,
        onError?: (error: Error) => void,
        onComplete?: () => void
    ): Promise<void> {
        const url = `${this.config.baseUrl}/chat/completions`;
        const model = options.model || this.config.model;

        try {
            const response = await fetch(url, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${this.config.apiKey}`,
                },
                body: JSON.stringify({
                    model,
                    messages: options.messages,
                    temperature: options.temperature ?? 0.7,
                    max_tokens: options.max_tokens,
                    stream: true,
                }),
            });

            if (!response.ok) {
                const errorData = await response.json().catch(() => ({}));
                throw new Error(`AI API流式请求失败: ${response.status} ${response.statusText}\n${JSON.stringify(errorData)}`);
            }

            const reader = response.body?.getReader();
            if (!reader) {
                throw new Error('无法获取响应流');
            }

            const decoder = new TextDecoder('utf-8');
            let buffer = '';

            // 处理流式响应
            const processStream = async (): Promise<void> => {
                try {
                    while (true) {
                        const { done, value } = await reader.read();
                        if (done) {
                            onComplete?.();
                            break;
                        }

                        // 解码并处理数据块
                        buffer += decoder.decode(value, { stream: true });
                        const lines = buffer.split('\n');
                        buffer = lines.pop() || '';

                        for (const line of lines) {
                            const trimmedLine = line.trim();
                            if (!trimmedLine || trimmedLine === 'data: [DONE]') continue;

                            try {
                                // 移除 'data: ' 前缀并解析JSON
                                const jsonStr = trimmedLine.replace(/^data: /, '');
                                const json = JSON.parse(jsonStr);
                                const content = json.choices?.[0]?.delta?.content || '';
                                if (content) {
                                    onMessage(content);
                                }
                            } catch (e) {
                                console.warn('解析流式响应失败:', trimmedLine, e);
                            }
                        }
                    }
                } catch (error) {
                    onError?.(error instanceof Error ? error : new Error(String(error)));
                }
            };

            await processStream();
        } catch (error) {
            onError?.(error instanceof Error ? error : new Error(String(error)));
        }
    }

    /**
     * 简单聊天方法
     * @param prompt 用户输入
     * @param stream 是否流式响应
     * @param systemPrompt 系统提示（可选）
     * @returns AI响应文本
     */
    async chat(prompt: string, stream: boolean = false, systemPrompt?: string,): Promise<string> {
        const messages: ChatMessage[] = [];

        if (systemPrompt) {
            messages.push({
                role: 'system',
                content: systemPrompt
            });
        }

        messages.push({
            role: 'user',
            content: prompt
        });

        try {
            const response = await this.createChatCompletion({
                messages,
                temperature: 0.7,
                stream,
            });

            return response.choices[0]?.message?.content || '';
        } catch (error) {
            console.error('AI聊天请求失败:', error);
            throw error;
        }
    }


    /**
     * 获取当前配置
     * @returns AI配置
     */
    getConfig(): AIServiceConfig {
        return { ...this.config };
    }

    /**
     * 更新配置
     * @param config 新的配置
     */
    updateConfig(config: Partial<AIServiceConfig>): void {
        this.config = {
            ...this.config,
            ...config
        };
    }

    /**
     * 从数据库加载AI配置
     */
    async loadConfigFromDB(): Promise<void> {
        try {
            const settings = await aiApi.load();
            if (settings) {
                this.config = {
                    apiKey: settings.api_key,
                    baseUrl: settings.base_url,
                    model: settings.model
                };
            } else {
                console.error('从数据库加载AI配置失败:', settings);
            }
        } catch (error) {
            console.error('从数据库加载AI配置失败:', error);
        }
    }

    /**
     * 保存AI配置到数据库
     * @param config AI配置
     */
    async saveConfigToDB(config: AIServiceConfig): Promise<void> {
        try {
            await aiApi.save({
                api_key: config.apiKey,
                base_url: config.baseUrl,
                model: config.model
            });

            // 更新本地配置
            this.updateConfig(config);
        } catch (error) {
            console.error('保存AI配置到数据库失败:', error);
            throw error;
        }
    }

    /**
     * 测试AI连接
     * @returns 测试结果
     */
    async testConnection(): Promise<{ success: boolean; message: string; responseTime?: number }> {
        const startTime = Date.now();

        try {
            // 验证配置
            if (!this.config.apiKey) {
                return {
                    success: false,
                    message: 'API Key未配置'
                };
            }

            if (!this.config.baseUrl) {
                return {
                    success: false,
                    message: 'Base URL未配置'
                };
            }

            // 发送测试请求
            await this.createChatCompletion({
                messages: [
                    {
                        role: 'user',
                        content: 'Hello, this is a test message. Please respond with "OK".'
                    }
                ],
                max_tokens: 10,
                temperature: 0.1
            });

            const responseTime = Date.now() - startTime;

            return {
                success: true,
                message: `连接成功！响应时间: ${responseTime}ms`,
                responseTime
            };
        } catch (error) {
            const responseTime = Date.now() - startTime;
            return {
                success: false,
                message: `连接失败: ${error instanceof Error ? error.message : String(error)}`,
                responseTime
            };
        }
    }

    async extTask(input: string): Promise<TodoVo | null> {
        let prompt = `现在是${new GDate().toChineseString()}; 要提取的文本是：${input}`
        let response = await this.chat(prompt, false, ExtractionTaskPrompt)
        let res = null
        if (response) {
            let todoStr = response.replace('```json', '').replace('```', '').trim();
            res = JSON.parse(todoStr) as TodoVo;

        }
        return res
    }

    async dailyMotivationalQuote(): Promise<string> {
        let prompt = `请生成`
        let response = await this.chat(prompt, false, dailyMotivationalQuotePrompt)
        return response
    }
}

// 创建默认AI服务实例
const defaultAIService = new AIService();

export default defaultAIService;