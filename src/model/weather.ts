// 天气相关类型定义

// 天气设置接口
export interface WeatherSettings {
  enabled: boolean
  api_key: string
  location_name: string
  latitude: number | null
  longitude: number | null
  adcode: string | null
  province: string | null
  city: string | null
  district: string | null
}

// 天气信息接口
export interface WeatherInfo {
  province: string
  city: string
  adcode: string
  weather: string
  temperature: string
  winddirection: string
  windpower: string
  humidity: string
  reporttime: string
  temperature_float: number
  humidity_float: number
}

// 天气类型枚举
export enum WeatherType {
  SUNNY = 'sunny',
  CLOUDY = 'cloudy',
  RAINY = 'rainy',
  SNOWY = 'snowy',
  FOGGY = 'foggy',
  STORMY = 'stormy',
  UNKNOWN = 'unknown'
}