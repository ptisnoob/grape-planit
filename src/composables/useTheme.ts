import { ref, computed } from 'vue'

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
    localStorage.setItem('theme', theme)
  }

  const toggleTheme = () => {
    const themes: ThemeType[] = ['light', 'dark', 'auto']
    const currentIndex = themes.indexOf(currentTheme.value)
    const nextIndex = (currentIndex + 1) % themes.length
    setTheme(themes[nextIndex])
  }

  const initTheme = () => {
    const savedTheme = localStorage.getItem('theme') as ThemeType
    if (savedTheme && ['light', 'dark', 'auto'].includes(savedTheme)) {
      setTheme(savedTheme)
    } else {
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
      case 'light': return 'sun'
      case 'dark': return 'moon'
      case 'auto': return 'A'
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