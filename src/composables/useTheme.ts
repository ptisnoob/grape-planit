import { ref, computed } from 'vue'
import { databaseApi } from '@/api/services'

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

  const toggleTheme = () => {
    const themes: ThemeType[] = ['light', 'dark', 'auto']
    const currentIndex = themes.indexOf(currentTheme.value)
    const nextIndex = (currentIndex + 1) % themes.length
    setTheme(themes[nextIndex])
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

  return {
    currentTheme,
    setTheme,
    toggleTheme,
    initTheme,
    isDark,
    getThemeIcon
  }
}