import { ref, computed } from 'vue'

export type ThemeType = 'light' | 'dark' | 'auto'

const currentTheme = ref<ThemeType>('light')

export const useTheme = () => {
  const setTheme = (theme: ThemeType) => {
    currentTheme.value = theme
    document.documentElement.setAttribute('data-theme', theme)
    localStorage.setItem('theme', theme)
  }

  const toggleTheme = () => {
    const newTheme = currentTheme.value === 'light' ? 'dark' : 'light'
    setTheme(newTheme)
  }

  const initTheme = () => {
    const savedTheme = localStorage.getItem('theme') as ThemeType
    if (savedTheme) {
      setTheme(savedTheme)
    } else if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
      setTheme('dark')
    }
  }

  const isDark = computed(() => currentTheme.value === 'dark')

  return {
    currentTheme,
    setTheme,
    toggleTheme,
    initTheme,
    isDark
  }
}