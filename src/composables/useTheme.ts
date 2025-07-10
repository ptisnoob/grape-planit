import { ref, computed } from 'vue'
import { databaseApi, windowApi } from '@/api/services'

export type ThemeType = 'light' | 'dark' | 'auto'

const currentTheme = ref<ThemeType>('auto')

const getSystemTheme = (): 'light' | 'dark' => {
  const isDarkMode = window.matchMedia('(prefers-color-scheme: dark)').matches;
  return isDarkMode ? 'dark' : 'light'
}

const applyTheme = (theme: ThemeType) => {
  let actualTheme: 'light' | 'dark'

  if (theme === 'auto') {
    actualTheme = getSystemTheme()
  } else {
    actualTheme = theme
  }

  document.documentElement.setAttribute('data-theme', actualTheme)
}

export const useTheme = () => {
  const setTheme = (theme: ThemeType) => {
    currentTheme.value = theme
    applyTheme(theme)
    // 不再使用localStorage，主题设置由GeneralSettings组件保存到数据库
  }

  const toggleTheme = async () => {
    const themes: ThemeType[] = ['light', 'dark', 'auto']
    const currentIndex = themes.indexOf(currentTheme.value)
    const nextIndex = (currentIndex + 1) % themes.length
    const newTheme = themes[nextIndex]
    
    // 应用主题到当前窗口
    setTheme(newTheme)
    
    // 应用主题到主窗口
    await applyThemeToMainWindow(newTheme)
    
    // 保存到数据库
    await saveThemeToDatabase(newTheme)
  }

  const initTheme = async () => {
    try {
      // 从数据库读取主题设置
      const settings = await databaseApi.window.load()
      if (settings && settings.theme && ['light', 'dark', 'auto'].includes(settings.theme)) {
        setTheme(settings.theme as ThemeType)
      } else {
        setTheme('auto')
      }
    } catch (error) {
      console.error('Failed to load theme from database:', error)
      setTheme('auto')
    }

    // 监听系统主题变化
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
      if (currentTheme.value === 'auto') {
        applyTheme('auto')
      }
    })
  }

  const isDark = computed(() => {
    if (currentTheme.value === 'auto') {
      return getSystemTheme() === 'dark'
    }
    return currentTheme.value === 'dark'
  })

  const getThemeIcon = computed(() => {
    switch (currentTheme.value) {
      case 'light': return 'sun-fill'
      case 'dark': return 'moon-fill'
      case 'auto': return 'contrast-alt'
      default: return 'sun'
    }
  })

  // 应用主题到主窗口
  const applyThemeToMainWindow = async (theme: string) => {
    try {
      // 如果是auto模式，需要获取实际的系统主题
      let actualTheme = theme
      if (theme === 'auto') {
        actualTheme = getSystemTheme()
      }

      // 通过JavaScript在主窗口中设置主题
      const script = `document.documentElement.setAttribute('data-theme', '${actualTheme}')`
      await windowApi.evalScript(script)
    } catch (error) {
      console.error('应用主题到主窗口失败:', error)
    }
  }

  // 保存主题设置到数据库
  const saveThemeToDatabase = async (theme: ThemeType) => {
    try {
      // 先加载当前设置
      const currentSettings = await databaseApi.window.load()
      if (currentSettings) {
        // 更新主题设置
        currentSettings.theme = theme
        // 保存到数据库
        await databaseApi.window.save(currentSettings)
        console.log('✅ [useTheme] 主题设置已保存到数据库:', theme)
      }
    } catch (error) {
      console.error('❌ [useTheme] 保存主题设置失败:', error)
    }
  }

  return {
    currentTheme,
    setTheme,
    toggleTheme,
    initTheme,
    isDark,
    getThemeIcon
  }
}