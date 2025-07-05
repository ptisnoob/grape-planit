<template>
  <div class="weather-settings">
    <!-- 天气功能开关 -->
    <div class="settings-section">
      <div class="section-header">
        <h3 class="section-title">🌤️ 天气功能</h3>
        <div class="toggle-switch">
          <input 
            type="checkbox" 
            id="weather-enabled" 
            v-model="currentSettings.enabled" 
            @change="saveSettings"
            class="toggle-input"
          >
          <label for="weather-enabled" class="toggle-label">
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>
      <p class="section-description">开启后将显示天气背景和天气信息</p>
    </div>

    <!-- API配置 -->
    <div class="settings-section" v-if="currentSettings.enabled">
      <h3 class="section-title">🗝️ API配置</h3>
      <div class="input-group">
        <label class="input-label">高德地图API Key</label>
        <input 
          type="password" 
          v-model="currentSettings.api_key" 
          @blur="saveSettings"
          placeholder="请输入高德地图API Key"
          class="api-key-input"
        >
      </div>
      <div class="config-tip">
        <span class="tip-text">💡 请在高德开放平台申请Web服务API Key</span>
      </div>
    </div>

    <!-- 位置信息 -->
    <div class="settings-section" v-if="currentSettings.enabled">
      <div class="section-header">
        <h3 class="section-title">📍 位置信息</h3>
        <button 
          @click="getCurrentLocation" 
          :disabled="!currentSettings.api_key || isGettingLocation"
          class="get-location-btn"
        >
          {{ isGettingLocation ? '获取中...' : '获取位置' }}
        </button>
      </div>
      
      <div v-if="currentSettings.location_name" class="location-info">
        <div class="location-summary">
          <div class="location-main">
            <span class="location-name">{{ getLocationDisplay }}</span>
            <span class="location-coords" v-if="currentSettings.latitude && currentSettings.longitude">
              {{ currentSettings.latitude.toFixed(3) }}, {{ currentSettings.longitude.toFixed(3) }}
            </span>
          </div>
        </div>
      </div>
      
      <div v-else class="no-location">
        <span class="no-location-text">暂未获取位置信息</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { WeatherSettings } from '@/common/weather'

// 当前设置状态
const currentSettings = ref<WeatherSettings>({
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

// 获取位置状态
const isGettingLocation = ref(false)

// 获取紧凑的位置显示
const getLocationDisplay = ref('')

// 更新位置显示
const updateLocationDisplay = () => {
  const parts = []
  if (currentSettings.value.city) parts.push(currentSettings.value.city)
  if (currentSettings.value.district) parts.push(currentSettings.value.district)
  getLocationDisplay.value = parts.join(' · ') || currentSettings.value.location_name || '未知位置'
}

// 保存设置到数据库
const saveSettings = async () => {
  try {
    await invoke('save_weather_settings_to_db', { settings: currentSettings.value })
    
    // 同步天气开关状态到主窗口
    const script = `
      if (window.weatherStore) {
        window.weatherStore.updateSettings(${JSON.stringify(currentSettings.value)});
      }
    `
    await invoke('eval_script_in_main_window', { script })
    
    console.log('✅ [前端] 天气设置已保存:', currentSettings.value)
  } catch (error) {
    console.error('❌ [前端] 保存天气设置失败:', error)
  }
}

// 获取当前位置
const getCurrentLocation = async () => {
  if (!currentSettings.value.api_key) {
    alert('请先配置高德地图API Key')
    return
  }
  
  isGettingLocation.value = true
  
  try {
    // 获取浏览器地理位置
    const position = await new Promise<GeolocationPosition>((resolve, reject) => {
      navigator.geolocation.getCurrentPosition(resolve, reject, {
        enableHighAccuracy: true,
        timeout: 10000,
        maximumAge: 60000
      })
    })
    
    const { latitude, longitude } = position.coords
    
    // 调用高德逆地理编码API
    const response = await fetch(
      `https://restapi.amap.com/v3/geocode/regeo?key=${currentSettings.value.api_key}&location=${longitude},${latitude}&extensions=all&batch=false&roadlevel=0`
    )
    
    if (!response.ok) {
      throw new Error('网络请求失败')
    }
    
    const data = await response.json()
    
    if (data.status === '1' && data.regeocode) {
      const regeocode = data.regeocode
      const addressComponent = regeocode.addressComponent
      
      // 更新位置信息
      currentSettings.value.latitude = latitude
      currentSettings.value.longitude = longitude
      currentSettings.value.location_name = regeocode.formatted_address || ''
      currentSettings.value.adcode = addressComponent.adcode || null
      currentSettings.value.province = addressComponent.province || null
      currentSettings.value.city = addressComponent.city || addressComponent.province || null
      currentSettings.value.district = addressComponent.district || null
      
      // 更新位置显示
      updateLocationDisplay()
      
      // 保存设置
      await saveSettings()
      
      console.log('✅ [前端] 位置信息获取成功')
    } else {
      throw new Error(data.info || '获取位置信息失败')
    }
  } catch (error) {
    console.error('❌ [前端] 获取位置失败:', error)
    if (error instanceof GeolocationPositionError) {
      switch (error.code) {
        case error.PERMISSION_DENIED:
          alert('位置访问被拒绝，请在浏览器设置中允许位置访问')
          break
        case error.POSITION_UNAVAILABLE:
          alert('位置信息不可用')
          break
        case error.TIMEOUT:
          alert('获取位置超时，请重试')
          break
      }
    } else {
      alert('获取位置失败: ' + (error as Error).message)
    }
  } finally {
    isGettingLocation.value = false
  }
}

// 加载设置
const loadSettings = async () => {
  try {
    const settings = await invoke<WeatherSettings>('load_weather_settings_from_db')
    currentSettings.value = settings
    updateLocationDisplay()
    console.log('✅ [前端] 天气设置加载成功:', settings)
  } catch (error) {
    console.error('❌ [前端] 加载天气设置失败:', error)
  }
}

// 组件挂载时加载设置
onMounted(async () => {
  await loadSettings()
})
</script>

<style scoped>
.weather-settings {
  padding: 0;
}

.settings-section {
  margin-bottom: 20px;
  padding: 16px;
  background: var(--bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
  display: flex;
  align-items: center;
  gap: 6px;
}

.section-description {
  font-size: 13px;
  color: var(--text-secondary);
  margin: 0;
  opacity: 0.8;
}

/* 开关样式 */
.toggle-switch {
  position: relative;
  display: inline-block;
}

.toggle-input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-label {
  display: block;
  width: 44px;
  height: 24px;
  background: var(--text-disabled);
  border-radius: 12px;
  cursor: pointer;
  transition: background 0.3s ease;
  position: relative;
}

.toggle-slider {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 20px;
  height: 20px;
  background: white;
  border-radius: 50%;
  transition: transform 0.3s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.toggle-input:checked + .toggle-label {
  background: var(--accent-color);
}

.toggle-input:checked + .toggle-label .toggle-slider {
  transform: translateX(20px);
}

.input-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-bottom: 8px;
}

.input-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
}

.api-key-input {
  padding: 10px 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 13px;
  transition: all 0.2s ease;
}

.api-key-input:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px var(--accent-color-alpha);
}

.config-tip {
  margin-top: 6px;
}

.tip-text {
  font-size: 12px;
  color: var(--text-secondary);
  opacity: 0.7;
}

.get-location-btn {
  padding: 8px 16px;
  background: var(--accent-color);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.get-location-btn:hover:not(:disabled) {
  background: var(--accent-color-hover);
  transform: translateY(-1px);
}

.get-location-btn:disabled {
  background: var(--text-disabled);
  cursor: not-allowed;
  transform: none;
}

.location-info {
  margin-top: 12px;
}

.location-summary {
  padding: 12px;
  background: var(--bg-primary);
  border-radius: 6px;
  border: 1px solid var(--border-color);
}

.location-main {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.location-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.location-coords {
  font-size: 12px;
  color: var(--text-secondary);
  opacity: 0.7;
  font-family: monospace;
}

.no-location {
  padding: 16px;
  text-align: center;
  background: var(--bg-primary);
  border-radius: 6px;
  border: 1px solid var(--border-color);
  margin-top: 12px;
}

.no-location-text {
  font-size: 13px;
  color: var(--text-disabled);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .settings-section {
    padding: 12px;
    margin-bottom: 16px;
  }
  
  .section-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }
}
</style>