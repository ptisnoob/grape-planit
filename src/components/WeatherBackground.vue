<template>
    <div class="weather-background" :class="props.containerClass" :style="weatherBackgroundStyle">
        <!-- 天气动画背景 -->
        <div class="weather-animation" v-if="weatherInfo">
            <!-- 云朵动画 -->
            <div class="clouds" v-if="shouldShowClouds">
                <div class="cloud cloud-1">☁️</div>
                <div class="cloud cloud-2">☁️</div>
                <div class="cloud cloud-3">☁️</div>
            </div>

            <!-- 雨滴动画 -->
            <div class="rain" v-if="shouldShowRain && !shouldShowSnow">
                <div class="raindrop" v-for="n in 20" :key="n" :style="getRaindropStyle(n)">💧</div>
            </div>

            <!-- 雪花动画 -->
            <div class="snow" v-if="shouldShowSnow">
                <div class="snowflake" v-for="n in 15" :key="n" :style="getSnowflakeStyle(n)">❄️</div>
            </div>

            <!-- 晴天指示器 -->
            <div class="weather-indicator" v-if="shouldShowSunshine">
                <div class="sun-circle">
                    <div class="sun-rays">
                        <div class="ray" v-for="n in 4" :key="n"></div>
                    </div>
                </div>
            </div>
        </div>

        <!-- 天气信息显示（可选） -->
        <div class="weather-info animate__animated animate__fadeInUp animate__delay-1s"
            v-if="props.showWeatherInfo && weatherInfo">
            <div class="weather-display">
                <span class="weather-icon">{{ getWeatherIcon(weatherInfo.weather) }}</span>
                <span class="weather-temp">{{ weatherInfo.temperature }}°C</span>
                <span class="weather-desc">{{ weatherInfo.weather }}</span>
                <span class="weather-location">
                    <i class="bi bi-geo-alt-fill"></i>
                    {{ weatherInfo.city }}
                </span>
            </div>
        </div>

        <!-- 插槽用于放置其他内容 -->
        <slot></slot>
    </div>
</template>

<script lang="ts" setup>
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { weatherService, type WeatherInfo, type WeatherSettings } from '@/common/weather'
import { invoke } from '@tauri-apps/api/core'

// Props
interface Props {
    showWeatherInfo?: boolean // 是否显示天气信息文字
    containerClass?: string   // 容器额外的CSS类名
}

const props = withDefaults(defineProps<Props>(), {
    showWeatherInfo: true,
    containerClass: ''
})

// 本地状态
const weatherBackgroundStyle = ref<string>('')
const weatherInfo = ref<WeatherInfo | null>(null)
const weatherLoading = ref(false)
const weatherSettings = ref<WeatherSettings>({
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

// 计算属性
const isWeatherEnabled = computed(() => weatherSettings.value.enabled)
const isConfigured = computed(() => {
  return weatherSettings.value.enabled && 
         weatherSettings.value.api_key.trim() !== '' && 
         weatherSettings.value.adcode !== null
})

// 天气动画控制
const shouldShowClouds = computed(() => {
    if (!isWeatherEnabled.value || !weatherInfo.value) return false
    const weather = weatherInfo.value.weather.toLowerCase()
    return weather.includes('云') || weather.includes('阴')
})

const shouldShowRain = computed(() => {
    if (!isWeatherEnabled.value || !weatherInfo.value) return false
    const weather = weatherInfo.value.weather.toLowerCase()
    return weather.includes('雨') || weather.includes('雷') || weather.includes('雪')
})

const shouldShowSnow = computed(() => {
    if (!isWeatherEnabled.value || !weatherInfo.value) return false
    const weather = weatherInfo.value.weather.toLowerCase()
    return weather.includes('雪')
})

const shouldShowSunshine = computed(() => {
    if (!isWeatherEnabled.value || !weatherInfo.value) return false
    const weather = weatherInfo.value.weather.toLowerCase()
    return weather.includes('晴') || weather.includes('阳')
})

// 天气相关方法
const loadWeatherInfo = async () => {
    if (!isConfigured.value) {
        console.log('🌤️ [WeatherBackground] 天气服务未配置或已禁用，跳过加载')
        weatherInfo.value = null
        return
    }

    try {
        weatherLoading.value = true
        const weather = await weatherService.getCurrentWeather(weatherSettings.value)
        if (weather) {
            weatherInfo.value = weather
            console.log('✅ [WeatherBackground] 天气信息加载成功:', weather)
        } else {
            weatherInfo.value = null
        }
    } catch (error) {
        console.error('❌ [WeatherBackground] 加载天气信息失败:', error)
        weatherInfo.value = null
    } finally {
        weatherLoading.value = false
    }
}

const resetWeatherBackground = () => {
    weatherBackgroundStyle.value = ''

    // 移除天气背景样式
    const styleId = 'weather-background-style'
    const styleElement = document.getElementById(styleId)
    if (styleElement) {
        styleElement.remove()
    }
}

const updateWeatherBackground = (weather: WeatherInfo) => {
    const weatherType = weatherService.getWeatherType(weather.weather)
    const { gradient, filter } = weatherService.getWeatherBackground(weatherType)

    // 使用伪元素方式，不影响主背景
    weatherBackgroundStyle.value = `
    position: relative;
    filter: ${filter};
    transition: filter 1s ease;
  `

    // 动态添加天气背景样式
    const styleId = 'weather-background-style'
    let styleElement = document.getElementById(styleId)
    if (!styleElement) {
        styleElement = document.createElement('style')
        styleElement.id = styleId
        document.head.appendChild(styleElement)
    }

    styleElement.textContent = `
    .weather-background::before {
      content: '';
      position: absolute;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background: ${gradient};
      opacity: 0.3;
      z-index: -1;
      transition: opacity 1s ease;
      pointer-events: none;
    }
  `
}

const getWeatherIcon = (weather: string) => {
    // 先将天气文本转换为WeatherType类型
    const weatherType = weatherService.getWeatherType(weather)
    return weatherService.getWeatherIcon(weatherType)
}

// 天气动画样式生成方法
const getRaindropStyle = (index: number) => {
    const left = Math.random() * 100
    const animationDelay = Math.random() * 2
    const animationDuration = 1 + Math.random() * 2
    return {
        left: `${left}%`,
        animationDelay: `${animationDelay}s`,
        animationDuration: `${animationDuration}s`
    }
}

const getSnowflakeStyle = (index: number) => {
    const left = Math.random() * 100
    const animationDelay = Math.random() * 3
    const animationDuration = 3 + Math.random() * 4
    const fontSize = 12 + Math.random() * 8
    return {
        left: `${left}%`,
        animationDelay: `${animationDelay}s`,
        animationDuration: `${animationDuration}s`,
        fontSize: `${fontSize}px`
    }
}

// 监听天气信息变化，更新背景
watch(weatherInfo, (newWeather) => {
    if (newWeather && isWeatherEnabled.value) {
        updateWeatherBackground(newWeather)
    } else {
        resetWeatherBackground()
    }
}, { immediate: true })

// 监听天气功能开关变化
watch(isWeatherEnabled, (enabled) => {
    if (!enabled) {
        resetWeatherBackground()
    } else if (weatherInfo.value) {
        updateWeatherBackground(weatherInfo.value)
    }
})

// 暴露方法给父组件
defineExpose({
    loadWeatherInfo,
    weatherInfo
})

// 加载天气设置
const loadWeatherSettings = async () => {
    try {
        const settings = await invoke<WeatherSettings>('load_weather_settings_from_db')
        weatherSettings.value = settings
        console.log('✅ [WeatherBackground] 天气设置加载成功:', settings)
        
        // 如果配置完整，加载天气信息
        if (isConfigured.value) {
            await loadWeatherInfo()
        }
    } catch (error) {
        console.error('❌ [WeatherBackground] 加载天气设置失败:', error)
    }
}

// 更新天气设置（供设置窗口调用）
const updateWeatherSettings = (newSettings: WeatherSettings) => {
    weatherSettings.value = newSettings
    console.log('🔄 [WeatherBackground] 天气设置已更新:', newSettings)
    
    // 如果天气功能被禁用，清除天气信息
    if (!newSettings.enabled) {
        weatherInfo.value = null
    } else if (isConfigured.value) {
        // 如果天气功能启用且配置完整，重新加载天气信息
        loadWeatherInfo()
    }
}

let weatherUpdateInterval: number | null = null

onMounted(async () => {
    // 加载天气设置
    await loadWeatherSettings()
    
    // 设置全局weatherStore供设置窗口调用
    ;(window as any).weatherStore = {
        updateSettings: updateWeatherSettings
    }

    // 设置天气信息定时更新（每30分钟更新一次）
    weatherUpdateInterval = window.setInterval(() => {
        if (isWeatherEnabled.value) {
            loadWeatherInfo()
        }
    }, 30 * 60 * 1000)
})

onUnmounted(() => {
    // 清理天气背景样式
    resetWeatherBackground()
    
    // 清理定时器
    if (weatherUpdateInterval) {
        clearInterval(weatherUpdateInterval)
    }
})
</script>

<style lang="scss" scoped>
.weather-background {
    width: 100%;
    height: 100%;
    position: relative;
    overflow: hidden;
}

/* 天气动画样式 */
.weather-animation {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
    z-index: 1;
    overflow: hidden;
}

/* 云朵动画 */
.clouds {
    position: absolute;
    width: 100%;
    height: 100%;
}

.cloud {
    position: absolute;
    font-size: 50px;
    opacity: 0.4;
    animation: cloudFloat 25s infinite linear;
    filter: blur(0.5px);
}

.cloud-1 {
    top: 8%;
    animation-delay: 0s;
    transform: scale(1.2);
}

.cloud-2 {
    top: 18%;
    animation-delay: -8s;
    transform: scale(0.9);
}

.cloud-3 {
    top: 12%;
    animation-delay: -16s;
    transform: scale(1.1);
}

@keyframes cloudFloat {
    0% {
        transform: translateX(-120px) translateY(0px);
    }

    25% {
        transform: translateX(25vw) translateY(-5px);
    }

    50% {
        transform: translateX(50vw) translateY(3px);
    }

    75% {
        transform: translateX(75vw) translateY(-2px);
    }

    100% {
        transform: translateX(calc(100vw + 120px)) translateY(0px);
    }
}

/* 雨滴动画 */
.rain {
    position: absolute;
    width: 100%;
    height: 100%;
}

.raindrop {
    position: absolute;
    top: -20px;
    font-size: 14px;
    opacity: 0.3;
    animation: fall linear infinite;
    filter: grayscale(30%) brightness(0.8);
}

@keyframes fall {
    0% {
        transform: translateY(-20px);
        opacity: 0.4;
    }

    100% {
        transform: translateY(100vh);
        opacity: 0;
    }
}

/* 雪花动画 */
.snow {
    position: absolute;
    width: 100%;
    height: 100%;
}

.snowflake {
    position: absolute;
    top: -20px;
    opacity: 0.4;
    animation: snowfall linear infinite;
    filter: grayscale(20%) brightness(0.9);
}

@keyframes snowfall {
    0% {
        transform: translateY(-20px) rotate(0deg);
        opacity: 0.5;
    }

    100% {
        transform: translateY(100vh) rotate(360deg);
        opacity: 0;
    }
}

/* 晴天指示器 */
.weather-indicator {
    position: absolute;
    top: 40px;
    right: 40px;
    width: 60px;
    height: 60px;
}

.sun-circle {
    width: 100%;
    height: 100%;
    background: #FFD700;
    border-radius: 50%;
    box-shadow: 0 0 15px rgba(255, 215, 0, 0.4);
    position: relative;
}

.sun-rays {
    position: absolute;
    width: 100%;
    height: 100%;
    animation: rotate 20s linear infinite;
}

.ray {
    position: absolute;
    width: 80px;
    height: 3px;
    background: linear-gradient(90deg, rgba(255, 215, 0, 0.6) 0%, rgba(255, 215, 0, 0) 100%);
    left: 50%;
    top: 50%;
    transform-origin: left center;
}

.ray:nth-child(1) {
    transform: rotate(0deg) translateX(30px);
}

.ray:nth-child(2) {
    transform: rotate(90deg) translateX(30px);
}

.ray:nth-child(3) {
    transform: rotate(180deg) translateX(30px);
}

.ray:nth-child(4) {
    transform: rotate(270deg) translateX(30px);
}

@keyframes rotate {
    from {
        transform: rotate(0deg);
    }

    to {
        transform: rotate(360deg);
    }
}

/* 天气信息样式 */
.weather-info {
    position: absolute;
    top: 20px;
    right: 20px;
    z-index: 5;
}

.weather-display {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 4px;
    padding: 8px 12px;
    color: var(--text-primary);
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
    transition: all var(--transition-normal);
}

.weather-icon {
    font-size: 32px;
    line-height: 1;
    margin-bottom: 4px;
}

.weather-temp {
    font-size: 24px;
    font-weight: 600;
    color: var(--text-primary);
    line-height: 1;
}

.weather-desc {
    font-size: 14px;
    font-weight: 400;
    color: var(--text-secondary);
    opacity: 0.9;
}

.weather-location {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 12px;
    color: var(--text-secondary);
    opacity: 0.8;

    i {
        font-size: 10px;
    }
}

/* 动画延迟调整 */
.animate__delay-1s {
    animation-delay: 0.3s;
}

/* 响应式调整 */
@media (max-width: 768px) {
    .weather-info {
        top: 15px;
        right: 15px;
    }

    .weather-display {
        padding: 6px 10px;
        font-size: 12px;
    }

    .weather-icon {
        font-size: 14px;
    }

    .weather-desc {
        font-size: 11px;
    }
}
</style>