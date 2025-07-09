<template>
    <WeatherBackground :show-weather-info="modeStore.currentMode === 'current'" container-class="default-box">
        <div class="default-box animate__animated animate__fadeIn">
            <!-- 时间显示区域 -->
            <div class="time-container animate__animated animate__fadeInUp animate__delay-1s">
                <h1 class="time-display" @click="toggleTimeDisplay">{{ displayTime }}</h1>
            </div>

            <!-- 日期信息 -->
            <div class="date-info animate__animated animate__fadeInUp animate__delay-1s"
                v-if="modeStore.currentMode === 'current'">
                <p class="date-text">{{ currentDate }} {{ currentWeekday }}</p>
                <p class="holiday-text">距下个假期：{{ nextHoliday.name }} (还有<span
                        class="holiday-days animate__animated animate__pulse animate__infinite">{{ nextHoliday.days
                        }}</span>天)</p>
            </div>

            <!-- 倒计时信息 -->
            <div class="countdown-info animate__animated animate__fadeInUp animate__delay-1s"
                v-else>
                <p class="countdown-target">{{ countdownTarget }}</p>
                <div class="countdown-actions" v-if="modeStore.currentMode === 'workEnd'">
                    <button class="edit-btn" @click="openWorkEndSettings" title="设置下班时间">
                        ✏️
                    </button>
                </div>

            </div>

            <h2 class="motivation-text animate__animated animate__fadeInUp animate__delay-2s"
                :class="{ 'generating': isGeneratingMotivation }">
                <span v-if="isGeneratingMotivation" class="loading-dots">生成中</span>
                <span v-else>{{ motivationText }}</span>
            </h2>
            <!-- 下班时间设置弹窗 -->
            <WorkEndSettings :visible="showWorkEndSettings" :work-end-time="workEndTime" @close="closeWorkEndSettings"
                @saved="handleWorkEndSaved" />


        </div>
    </WeatherBackground>
</template>

<script lang="ts" setup>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { listen } from '@tauri-apps/api/event'
import WorkEndSettings from './WorkEndSettings.vue'

import WeatherBackground from './WeatherBackground.vue'
import { CountdownConfig, CountdownData } from '@/model/countdown'
import { useModeStore } from '@/store/mode'

import { useDatabase } from '@/composables/useDatabase'
import { useTime } from '@/composables/useTime'
import { databaseApi } from '@/api/services'
import { holidayApi } from '@/api/holiday'
import type { Holiday } from '@/model/holiday'
import { motivationApi } from '@/api/motivation'
import defaultAIService from '@/common/ai'


// 使用时间相关的 composable
const {
    currentTime,
    currentDate,
    currentWeekday,
    startTimer,
    stopTimer,
    updateTimeForDefaultDisplay,
    formatCountdownToHMS,
} = useTime()

const nextHoliday = ref({
    name: '--',
    days: '--'
})
const motivationText = ref('请你再坚持一下，终会拨开乌云见明月！')
const isGeneratingMotivation = ref(false)
const showSeconds = ref(true) // 控制是否显示秒数
const countdownData = ref<CountdownData | null>(null)
const config = ref<CountdownConfig | null>(null)

const modeStore = useModeStore()
const { loadConfigFromDb, updateCountdownConfig: updateConfigInDb } = useDatabase()

const showWorkEndSettings = ref(false)

// 倒计时设置
const workEndTime = ref('')
const beforeTime = ref(60)

// 事件监听器
let unlistenCountdown: (() => void) | null = null

// 计算属性
const displayTime = computed(() => {
    if (modeStore.currentMode === 'current') {
        return showSeconds.value ? currentTime.value : currentTime.value.slice(0, 5)
    } else if (countdownData.value) {
        // 如果倒计时已结束（finished状态）
        if (countdownData.value.status === 'finished') {
            if (countdownData.value.mode === 'workEnd') {
                return '下班'
            }
        }

        // 如果倒计时正在运行且有时间戳
        if (countdownData.value.timestamp > 0) {
            const totalSeconds = countdownData.value.timestamp

            if (showSeconds.value) {
                // 显示秒数时，使用完整的时:分:秒格式
                return formatCountdownToHMS(totalSeconds)
            } else {
                // 不显示秒数时，使用时:分格式
                const hours = Math.floor(totalSeconds / 3600)
                const minutes = Math.floor((totalSeconds % 3600) / 60)
                return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}`
            }
        }

        // 其他情况（如未设置目标时间）
        return '--:--'
    }
    return currentTime.value
})


const countdownTarget = computed(() => {
    if (countdownData.value) {
        return countdownData.value.target_info
    }
    return ''
})




const toggleTimeDisplay = async () => {
    showSeconds.value = !showSeconds.value
    if (config.value) {
        config.value.showSeconds = showSeconds.value
        await updateConfig()
    }
}



// 与Rust后端通信的函数
const loadConfig = async () => {
    try {
        // 优先从数据库加载配置
        const rustConfig = await loadConfigFromDb() as CountdownConfig

        config.value = rustConfig
        showSeconds.value = rustConfig.showSeconds
        workEndTime.value = rustConfig.workEndTime

        // 更新beforeTime为配置中的值（转换为秒）
        beforeTime.value = (rustConfig.finalCountdownMinutes || 1) * 60
    } catch (error) {
        console.error('Failed to load config from database:', error)
    }
}

const updateConfig = async () => {
    if (!config.value) return

    try {
        // 使用updateConfigInDb统一更新（它内部会调用update_countdown_config）
        await updateConfigInDb(config.value)
    } catch (error) {
        console.error('Failed to update config:', error)
    }
}

// 新的事件处理方法
const closeWorkEndSettings = () => {
    showWorkEndSettings.value = false
}

const openWorkEndSettings = () => {
    showWorkEndSettings.value = true
}

const handleWorkEndSaved = async () => {
    // 重新加载配置以更新倒计时
    await loadConfig()
}


// 设置倒计时事件监听
const setupCountdownListener = async () => {
    try {
        console.log('🎧 [DefaultTime] 开始设置倒计时事件监听器')
        unlistenCountdown = await listen('countdown-update', (event) => {
            const newData = event.payload as CountdownData
            
            // 只有在下班倒计时模式下才处理下班倒计时数据
            if (modeStore.currentMode === 'workEnd' && newData.mode === 'workEnd') {
                console.log('✅ [DefaultTime] 更新下班倒计时数据')
                countdownData.value = newData
            }
        })
    } catch (error) {
        console.error('Failed to setup countdown listener:', error)
    }
}

// 使用 composable 中的 updateTimeForDefaultDisplay 方法替代

// 生成励志文案
const generateMotivationText = async (forceGenerate: boolean = false) => {
    if (isGeneratingMotivation.value) {
        return
    }
    
    try {
        isGeneratingMotivation.value = true
        
        // 如果不是强制生成，先尝试从缓存获取
        if (!forceGenerate) {
            const cached = await motivationApi.getTodayCache()
            if (cached && cached.content) {
                motivationText.value = cached.content
                return
            }
        }
        
        // 生成新的励志文案
        const newMotivation = await defaultAIService.dailyMotivationalQuote()
        if (newMotivation && newMotivation.trim()) {
            motivationText.value = newMotivation.trim()
            // 保存到缓存
            await motivationApi.saveTodayCache(motivationText.value)
        }
    } catch (error) {
        console.error('生成励志文案失败:', error)
        // 如果生成失败，保持默认文案
        if (motivationText.value === '请你再坚持一下，终会拨开乌云见明月！') {
            // 如果还是默认文案，可以尝试使用一些备用文案
            const backupMotivations = [
                '每一个不曾起舞的日子，都是对生命的辜负。',
                '星光不问赶路人，时光不负有心人。',
                '愿你历尽千帆，归来仍是少年。',
                '山高路远，但总有人为你点亮明灯。',
                '纵然前路未卜，也要勇敢地走下去。'
            ]
            const randomIndex = Math.floor(Math.random() * backupMotivations.length)
            motivationText.value = backupMotivations[randomIndex]
        }
    } finally {
        isGeneratingMotivation.value = false
    }
}



// 从数据库获取节假日数据并计算下个节假日
const updateNextHoliday = async () => {
    try {
        const currentYear = new Date().getFullYear()
        const nextYear = currentYear + 1
        
        // 获取当前年份和下一年的节假日数据
        const [currentYearData, nextYearData] = await Promise.all([
            holidayApi.getByYear(currentYear),
            holidayApi.getByYear(nextYear)
        ])
        
        // 提取节假日数组
        const currentYearHolidays = currentYearData?.days || []
        const nextYearHolidays = nextYearData?.days || []
        
        // 合并两年的节假日数据
        const allHolidays: Holiday[] = [...currentYearHolidays, ...nextYearHolidays]
        
        if (allHolidays.length === 0) {
            // 如果没有节假日数据，显示 --
            nextHoliday.value = {
                name: '--',
                days: '--'
            }
            return
        }
        
        // 找到下一个节假日
        const now = new Date()
        const today = new Date(now.getFullYear(), now.getMonth(), now.getDate())
        
        // 过滤出未来的节假日并按日期排序
        const futureHolidays = allHolidays
            .filter(holiday => {
                const holidayDate = new Date(holiday.date)
                return holidayDate >= today
            })
            .sort((a, b) => new Date(a.date).getTime() - new Date(b.date).getTime())
        
        if (futureHolidays.length > 0) {
            const nextHolidayData = futureHolidays[0]
            const holidayDate = new Date(nextHolidayData.date)
            const diffTime = holidayDate.getTime() - today.getTime()
            const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24))
            
            nextHoliday.value = {
                name: nextHolidayData.name,
                days: diffDays.toString()
            }
        } else {
            // 如果没有未来的节假日，显示 --
            nextHoliday.value = {
                name: '--',
                days: '--'
            }
        }
    } catch (error) {
        console.error('获取节假日数据失败:', error)
        // 出错时显示 --
        nextHoliday.value = {
            name: '--',
            days: '--'
        }
    }
}







onMounted(async () => {
    await loadConfig();
    await setupCountdownListener();

    // 启动倒计时服务
    try {
        await databaseApi.countdown.startTimer();
    } catch (error) {
        console.error('Failed to start countdown timer:', error);
    }

    // 启动时间更新定时器
    startTimer(updateTimeForDefaultDisplay);
    await updateNextHoliday();
    
    // 生成励志文案（优先从缓存获取）
    await generateMotivationText(false);
    
    // 清理过期的励志文案缓存
    try {
        await motivationApi.cleanupCache();
    } catch (error) {
        console.error('清理励志文案缓存失败:', error);
    }
});

onUnmounted(() => {
    stopTimer();
    if (unlistenCountdown) {
        unlistenCountdown();
    }
});
</script>

<style lang="scss" scoped>
.default-box {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    // padding: 30px 0;
    text-align: center;
    overflow: hidden;
    position: relative;
}



// 时间容器
.time-container {
    display: flex;
    flex-direction: column;
    align-items: center;
}


// 倒计时信息
.countdown-info {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 15px;
    margin: 10px 0;
}

.countdown-target {
    font-size: 16px;
    color: var(--text-secondary);
    margin: 0;
}

.countdown-actions {
    display: flex;
    gap: 10px;
}

.action-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 16px;
    border: 1px solid var(--border-color);
    border-radius: 6px;
    background: var(--bg-primary);
    color: var(--text-primary);
    font-size: 14px;
    cursor: pointer;
    transition: all var(--transition-normal);

    &:hover {
        background: var(--bg-hover);
        border-color: var(--accent-color);
    }

    i {
        font-size: 14px;
    }
}

.edit-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border: none;
    border-radius: 50%;
    background: transparent;
    color: white;
    font-size: 14px;
    cursor: pointer;
    transition: all var(--transition-normal);
    // margin-left: 10px;

    &:hover {
        background: var(--accent-color-hover);
        transform: scale(1.1);
    }
}





.time-display {
    font-size: 60px;
    font-weight: 600;
    letter-spacing: 15px;
    color: var(--text-primary);
    font-variant-numeric: tabular-nums;
    font-family: 'Courier New', 'Monaco', 'Menlo', monospace;
    width: 100%;
    min-width: 400px;
    display: inline-block;
    user-select: none;
    transition: all var(--transition-normal);
    cursor: default;

    &:hover {
        transform: scale(1.05);
        text-shadow: 0 0 20px var(--accent-color);
    }
}

.date-info {
    display: flex;
    flex-direction: column;
    gap: 5px;
    align-items: center;
}

.date-text,
.holiday-text {
    color: var(--text-secondary);
    margin: 0;
}

.holiday-days {
    color: var(--accent-color);
    font-weight: 600;
    font-size: 1.1em;
    text-shadow: 0 0 8px var(--accent-color);
}

.motivation-text {
    font-size: 18px;
    font-weight: 400;
    line-height: 1.6;
    color: var(--text-primary);
    max-width: 500px;
    transition: all 0.3s ease;
    border-radius: 8px;
    padding: 12px 16px;
    position: relative;
}



.motivation-text.generating {
    color: #999;
    cursor: wait;
    background-color: rgba(153, 153, 153, 0.1);
}

.loading-dots {
    position: relative;
}

.loading-dots::after {
    content: '';
    position: absolute;
    right: -20px;
    top: 50%;
    transform: translateY(-50%);
    width: 4px;
    height: 4px;
    border-radius: 50%;
    background-color: #999;
    animation: loading-dots 1.5s infinite;
}

@keyframes loading-dots {
    0%, 20% {
        box-shadow: 0 0 0 0 #999, 8px 0 0 0 transparent, 16px 0 0 0 transparent;
    }
    40% {
        box-shadow: 0 0 0 0 transparent, 8px 0 0 0 #999, 16px 0 0 0 transparent;
    }
    60%, 100% {
        box-shadow: 0 0 0 0 transparent, 8px 0 0 0 transparent, 16px 0 0 0 #999;
    }
}

/* 动画延迟调整 */
.animate__delay-1s {
    animation-delay: 0.3s;
}

.animate__delay-2s {
    animation-delay: 0.6s;
}

.animate__delay-3s {
    animation-delay: 0.9s;
}

/* 悬停效果 */
.motivation-text {
    transition: all var(--transition-normal);
    cursor: default;

    &:hover {
        color: var(--accent-color);
    }
}

/* 最后倒计时效果 */
.final-countdown-container {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 99;
    background: rgba(0, 0, 0, 0.9);
    overflow: hidden;
    cursor: pointer;
}

.final-countdown-number {
    font-size: 220px;
    font-weight: bold;
    color: #ffffff;
    text-align: center;
    text-shadow: 0 0 30px rgba(255, 255, 255, 0.8);
    user-select: none;
}

@media screen and (max-width: 1024px) {
    .final-countdown-number {
        font-size: 150px;
    }
}

@media screen and (max-width: 768px) {
    .final-countdown-number {
        font-size: 120px;
    }
}

/* 结束状态样式 */
.end-state-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 20px;
    z-index: 10;
}

.end-message {
    font-size: 24px;
    font-weight: 600;
    color: var(--text-color);
    text-align: center;
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    cursor: pointer;
    transition: all 0.3s ease;
    padding: 12px 20px;
    border-radius: 8px;
    user-select: none;
}

.end-message:hover {
    color: var(--accent-color);
    transform: scale(1.05);
    text-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
}

.end-message:active {
    transform: scale(1.02);
    color: var(--accent-color-hover, var(--accent-color));
}



/* 响应式调整 */
@media (max-width: 768px) {
    .end-message {
        font-size: 20px;
        padding: 10px 16px;
    }
    
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