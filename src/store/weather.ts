import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { weatherApi } from '@/api/services'
import { weatherService, type WeatherInfo, type WeatherSettings } from '@/common/weather'

export const useWeatherStore = defineStore('weather', () => {
  // 天气设置状态
  const settings = ref<WeatherSettings>({
    enabled: false,
    api_key: '',
    location_name: '',
    latitude: null,
    longitude: null,
    adcode: null,
    province: null,
    city: null,
    district: null
  })

  // 天气信息状态
  const weatherInfo = ref<WeatherInfo | null>(null)
  const isLoading = ref(false)
  const lastUpdateTime = ref<Date | null>(null)

  // 计算属性
  const isEnabled = computed(() => settings.value.enabled)
  const isConfigured = computed(() => {
    return settings.value.enabled && 
           settings.value.api_key.trim() !== '' && 
           settings.value.adcode !== null
  })

  // 获取紧凑的位置显示
  const locationDisplay = computed(() => {
    const parts = []
    if (settings.value.city) parts.push(settings.value.city)
    if (settings.value.district) parts.push(settings.value.district)
    return parts.join(' · ') || settings.value.location_name || '未知位置'
  })

  // 加载天气设置
  const loadSettings = async (): Promise<void> => {
    try {
      const loadedSettings = await weatherApi.load()
      settings.value = loadedSettings!
      console.log('✅ [WeatherStore] 天气设置加载成功:', loadedSettings)
    } catch (error) {
      console.error('❌ [WeatherStore] 加载天气设置失败:', error)
    }
  }

  // 保存天气设置
  const saveSettings = async (newSettings?: Partial<WeatherSettings>): Promise<void> => {
    try {
      if (newSettings) {
        settings.value = { ...settings.value, ...newSettings }
      }
      
      await weatherApi.save(settings.value)
      console.log('✅ [WeatherStore] 天气设置已保存:', settings.value)
      
      // 如果天气功能被禁用，清除天气信息
      if (!settings.value.enabled) {
        weatherInfo.value = null
      } else if (isConfigured.value) {
        // 如果天气功能启用且配置完整，重新加载天气信息
        await loadWeatherInfo()
      }
    } catch (error) {
      console.error('❌ [WeatherStore] 保存天气设置失败:', error)
      throw error
    }
  }

  // 切换天气功能开关
  const toggleEnabled = async (): Promise<void> => {
    await saveSettings({ enabled: !settings.value.enabled })
  }

  // 更新API Key
  const updateApiKey = async (apiKey: string): Promise<void> => {
    await saveSettings({ api_key: apiKey })
  }

  // 更新位置信息
  const updateLocation = async (locationData: Partial<WeatherSettings>): Promise<void> => {
    await saveSettings(locationData)
  }

  // 加载天气信息
  const loadWeatherInfo = async (): Promise<void> => {
    if (!isConfigured.value) {
      console.log('🌤️ [WeatherStore] 天气服务未配置或已禁用，跳过加载')
      weatherInfo.value = null
      return
    }

    try {
      isLoading.value = true
      const weather = await weatherService.getCurrentWeather()
      if (weather) {
        weatherInfo.value = weather
        lastUpdateTime.value = new Date()
        console.log('✅ [WeatherStore] 天气信息加载成功:', weather)
      } else {
        weatherInfo.value = null
      }
    } catch (error) {
      console.error('❌ [WeatherStore] 加载天气信息失败:', error)
      weatherInfo.value = null
    } finally {
      isLoading.value = false
    }
  }

  // 强制刷新天气信息
  const refreshWeatherInfo = async (): Promise<void> => {
    await loadWeatherInfo()
  }

  // 初始化store
  const initialize = async (): Promise<void> => {
    await loadSettings()
    if (isConfigured.value) {
      await loadWeatherInfo()
    }
  }

  return {
    // 状态
    settings,
    weatherInfo,
    isLoading,
    lastUpdateTime,
    
    // 计算属性
    isEnabled,
    isConfigured,
    locationDisplay,
    
    // 方法
    loadSettings,
    saveSettings,
    toggleEnabled,
    updateApiKey,
    updateLocation,
    loadWeatherInfo,
    refreshWeatherInfo,
    initialize
  }
})