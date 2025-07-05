import { invoke } from '@tauri-apps/api/core';
import { useTheme } from '@/composables/useTheme';
import { WeatherSettings, WeatherInfo, WeatherType } from '@/model/weather';

// 重新导出类型以保持向后兼容
export type { WeatherSettings, WeatherInfo, WeatherType } from '@/model/weather';

/**
 * 天气服务类
 * 提供高德地图天气API的交互功能
 */
export class WeatherService {
  private settings: WeatherSettings | null = null;

  constructor() {
    // 移除自动加载设置
  }

  /**
   * 设置天气配置
   */
  setSettings(settings: WeatherSettings): void {
    this.settings = settings;
  }

  /**
   * 加载天气设置
   */
  async loadSettings(): Promise<WeatherSettings | null> {
    try {
      this.settings = await invoke<WeatherSettings>('load_weather_settings_from_db');
      return this.settings;
    } catch (error) {
      console.error('加载天气设置失败:', error);
      this.settings = null;
      return null;
    }
  }

  /**
   * 检查是否已配置
   */
  checkConfiguration(settings?: WeatherSettings): boolean {
    const currentSettings = settings || this.settings;
    return !!(currentSettings?.enabled && currentSettings?.api_key && currentSettings?.adcode);
  }

  /**
   * 获取当前天气信息
   * @param settings 可选的天气设置，如果不提供则使用内部设置
   * @returns 天气信息
   */
  async getCurrentWeather(settings?: WeatherSettings): Promise<WeatherInfo | null> {
    const currentSettings = settings || this.settings;
    if (!currentSettings?.api_key || !currentSettings?.adcode) {
      console.warn('Weather API key or location not configured');
      return null;
    }

    try {
      const response = await fetch(
        `https://restapi.amap.com/v3/weather/weatherInfo?city=${currentSettings.adcode}&key=${currentSettings.api_key}&extensions=base`
      );

      if (!response.ok) {
        throw new Error(`Weather API request failed: ${response.status}`);
      }

      const data = await response.json();

      if (data.status !== '1' || !data.lives || data.lives.length === 0) {
        throw new Error('Invalid weather API response');
      }

      const weatherData = data.lives[0];

      return {
        province: weatherData.province,
        city: weatherData.city,
        adcode: weatherData.adcode,
        weather: weatherData.weather,
        temperature: weatherData.temperature,
        winddirection: weatherData.winddirection,
        windpower: weatherData.windpower,
        humidity: weatherData.humidity,
        reporttime: weatherData.reporttime,
        temperature_float: parseFloat(weatherData.temperature) || 0,
        humidity_float: parseFloat(weatherData.humidity) || 0
      };
    } catch (error) {
      console.error('Failed to fetch weather:', error);
      return null;
    }
  }

  /**
   * 根据天气描述判断天气类型
   * @param weather 天气描述
   * @returns 天气类型
   */
  getWeatherType(weather: string): WeatherType {
    const weatherLower = weather.toLowerCase();

    if (weatherLower.includes('晴') || weatherLower.includes('sunny')) {
      return WeatherType.SUNNY;
    } else if (weatherLower.includes('云') || weatherLower.includes('阴') || weatherLower.includes('cloudy')) {
      return WeatherType.CLOUDY;
    } else if (weatherLower.includes('雨') || weatherLower.includes('rain') || weatherLower.includes('shower')) {
      return WeatherType.RAINY;
    } else if (weatherLower.includes('雪') || weatherLower.includes('snow')) {
      return WeatherType.SNOWY;
    } else if (weatherLower.includes('雾') || weatherLower.includes('霾') || weatherLower.includes('fog')) {
      return WeatherType.FOGGY;
    } else if (weatherLower.includes('雷') || weatherLower.includes('storm') || weatherLower.includes('thunder')) {
      return WeatherType.STORMY;
    } else {
      return WeatherType.UNKNOWN;
    }
  }

  /**
   * 获取天气图标
   * @param weatherType 天气类型
   * @returns 天气图标
   */
  getWeatherIcon(weatherType: WeatherType): string {
    switch (weatherType) {
      case WeatherType.SUNNY:
        return '☀️';
      case WeatherType.CLOUDY:
        return '☁️';
      case WeatherType.RAINY:
        return '🌧️';
      case WeatherType.SNOWY:
        return '❄️';
      case WeatherType.FOGGY:
        return '🌫️';
      case WeatherType.STORMY:
        return '⛈️';
      default:
        return '🌤️';
    }
  }

  /**
   * 获取天气背景
   * @param weatherType 天气类型
   * @returns 背景样式对象，包含 gradient 和 filter
   */
  getWeatherBackground(weatherType: WeatherType): { gradient: string; filter: string } {
    const { isDark } = useTheme();
    const themeKey = isDark.value ? 'dark' : 'light';

    const weatherBackgrounds: Record<string, { light: { gradient: string; filter: string }; dark: { gradient: string; filter: string } }> = {
      [WeatherType.SUNNY]: {
        light: { gradient: 'radial-gradient(ellipse at 70% 30%, #FFD700 0%, #FFA500 40%, #FF8C00 80%, #f2dc8a 100%)', filter: 'saturate(1.2) brightness(1.1)' },
        dark: { gradient: 'radial-gradient(ellipse at 70% 30%, #4a4a4a 0%, #333333 40%, #1a1a1a 80%, #000000 100%)', filter: 'saturate(1.1) brightness(0.9)' }
      },
      [WeatherType.CLOUDY]: {
        light: { gradient: 'linear-gradient(to top, #B0C4DE 0%, #E6E9F0 100%)', filter: 'saturate(0.9) brightness(1.05)' },
        dark: { gradient: 'linear-gradient(to top, #4a4a4a 0%, #6b6b6b 100%)', filter: 'saturate(0.8) brightness(0.9)' }
      },
      [WeatherType.RAINY]: {
        light: { gradient: 'linear-gradient(to top, #4682B4 0%, #708090 100%)', filter: 'saturate(1.1) brightness(0.8)' },
        dark: { gradient: 'linear-gradient(to top, #1a1a1a 0%, #333333 100%)', filter: 'saturate(1) brightness(0.7)' }
      },
      [WeatherType.SNOWY]: {
        light: { gradient: 'linear-gradient(to top, #F0F8FF 0%, #E0FFFF 100%)', filter: 'saturate(0.8) brightness(1.1)' },
        dark: { gradient: 'linear-gradient(to top, #4a4a4a 0%, #6b6b6b 100%)', filter: 'saturate(0.7) brightness(0.95)' }
      },
      [WeatherType.FOGGY]: {
        light: { gradient: 'linear-gradient(to top, #D3D3D3 0%, #E6E6FA 100%)', filter: 'saturate(0.6) brightness(1.0)' },
        dark: { gradient: 'linear-gradient(to top, #333333 0%, #4a4a4a 100%)', filter: 'saturate(0.5) brightness(0.85)' }
      },
      [WeatherType.STORMY]: {
        light: { gradient: 'linear-gradient(to top, #2F4F4F 0%, #483D8B 100%)', filter: 'saturate(1.2) brightness(0.7)' },
        dark: { gradient: 'linear-gradient(to top, #1a1a1a 0%, #333333 100%)', filter: 'saturate(1.1) brightness(0.6)' }
      },
      [WeatherType.UNKNOWN]: {
        light: { gradient: 'linear-gradient(to top, #ADD8E6 0%, #F0F8FF 100%)', filter: 'saturate(1) brightness(1)' },
        dark: { gradient: 'linear-gradient(to top, #333333 0%, #4a4a4a 100%)', filter: 'saturate(0.9) brightness(0.9)' }
      }
    };

    const backgroundSet = weatherBackgrounds[weatherType] || weatherBackgrounds[WeatherType.UNKNOWN];
    return backgroundSet[themeKey];
  }
}

// 导出单例实例
export const weatherService = new WeatherService();

// 导出工具函数
export const getWeatherSettings = async (): Promise<WeatherSettings | null> => {
  try {
    return await invoke<WeatherSettings>('load_weather_settings_from_db');
  } catch (error) {
    console.error('获取天气设置失败:', error);
    return null;
  }
};

export const saveWeatherSettings = async (settings: WeatherSettings): Promise<boolean> => {
  try {
    await invoke('save_weather_settings_to_db', { settings });
    return true;
  } catch (error) {
    console.error('保存天气设置失败:', error);
    return false;
  }
};