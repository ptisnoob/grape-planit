<template>
    <div class="default-box animate__animated animate__fadeIn">

        <!-- 时间显示区域 -->
        <div class="time-container animate__animated animate__fadeInUp animate__delay-1s"
            v-if="!shouldShowFinalCountdown">
            <h1 class="time-display" @click="toggleTimeDisplay">{{ displayTime }}</h1>
        </div>

        <!-- 日期信息 -->
        <div class="date-info animate__animated animate__fadeInUp animate__delay-1s"
            v-if="modeStore.currentMode === 'current'">
            <p class="date-text">{{ currentDate }} {{ currentWeekday }}</p>
            <p class="holiday-text">下个节日：{{ nextHoliday.name }} (<span
                    class="holiday-days animate__animated animate__pulse animate__infinite">{{ nextHoliday.days
                    }}</span>天)</p>
        </div>

        <!-- 倒计时信息 -->
        <div class="countdown-info animate__animated animate__fadeInUp animate__delay-1s"
            v-else-if="!shouldShowFinalCountdown">
            <p class="countdown-target">{{ countdownTarget }}</p>
            <div class="countdown-actions" v-if="modeStore.currentMode === 'workEnd'">
                <button class="edit-btn" @click="openWorkEndSettings" title="设置下班时间">
                    ✏️
                </button>
            </div>
            <div class="countdown-actions" v-else-if="modeStore.currentMode === 'custom'">
                <button class="action-btn" @click="openSettings">
                    <i class="icon-settings"></i>
                    设置
                </button>
            </div>
        </div>

        <!-- 最后倒计时效果 -->
        <div class="final-countdown-container" v-if="shouldShowFinalCountdown" @click="handleGotIt">
            <div :key="finalCountdownNumber" class="final-countdown-number animate__animated animate__pulse">
                {{ finalCountdownNumber }}
            </div>
        </div>

        <h2 class="motivation-text animate__animated animate__fadeInUp animate__delay-2s"
            v-if="!shouldShowFinalCountdown">{{ motivationText }}</h2>
        <!-- 下班时间设置弹窗 -->
        <WorkEndSettings :visible="showWorkEndSettings" :work-end-time="workEndTime" @close="closeWorkEndSettings"
            @saved="handleWorkEndSaved" />

        <!-- 自定义倒计时设置弹窗 -->
        <CustomCountdownSettings :visible="showSettings" :custom-countdown="customCountdown" @close="closeSettings" />

    </div>
</template>

<script lang="ts" setup>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import WorkEndSettings from './WorkEndSettings.vue'
import CustomCountdownSettings from './CustomCountdownSettings.vue'
import { CountdownConfig, CountdownData } from '@/model/countdown'
import { useModeStore } from '@/store/mode'
import { useDatabase } from '@/composables/useDatabase'
import { useTime } from '@/composables/useTime'


// 使用时间相关的 composable
const {
    currentTime,
    currentDate,
    currentWeekday,
    startTimer,
    stopTimer,
    updateTimeForDefaultDisplay,
    formatCountdownToHMS,
    calculateNextHoliday: getNextHoliday,
} = useTime()

const nextHoliday = ref({
    name: '国庆节',
    days: 37
})
const motivationText = ref('请你再坚持一下，终会拨开乌云见明月！')
const showSeconds = ref(true) // 控制是否显示秒数
const countdownData = ref<CountdownData | null>(null)
const config = ref<CountdownConfig | null>(null)

const modeStore = useModeStore()
const { loadConfigFromDb, updateCountdownConfig: updateConfigInDb } = useDatabase()

const showSettings = ref(false)
const showWorkEndSettings = ref(false)

// 倒计时设置
const workEndTime = ref('')
const customCountdown = ref({
    name: '自定义事件',
    target: ''
})
const beforeTime = ref(60)

// 最后倒计时状态
const isInFinalCountdown = ref(false)
// 结束状态相关
const isInEndState = ref(false)
const endStateTimer = ref<NodeJS.Timeout | null>(null)

// 事件监听器
let unlistenCountdown: (() => void) | null = null

// 计算属性
const displayTime = computed(() => {
    if (modeStore.currentMode === 'current') {
        return showSeconds.value ? currentTime.value : currentTime.value.slice(0, 5)
    } else if (countdownData.value) {
        // 如果倒计时已结束（finished状态）
        if (countdownData.value.status === 'finished') {
            if (countdownData.value.mode === 'custom') {
                return '已结束'
            } else if (countdownData.value.mode === 'workEnd') {
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

// 判断是否进入最后倒计时阶段
const shouldShowFinalCountdown = computed(() => {
    if (modeStore.currentMode !== 'current' && countdownData.value) {
        // 如果状态是finished，显示最后倒计时效果
        if (countdownData.value.status === 'finished') {
            return true
        }
        // 如果状态是running且时间小于等于beforeTime，显示最后倒计时
        if (countdownData.value.status === 'running' && countdownData.value.timestamp > 0 && countdownData.value.timestamp <= beforeTime.value) {
            return true
        }
    }
    return false
})

// 最后倒计时显示的数字或文本
const finalCountdownNumber = computed(() => {
    if (shouldShowFinalCountdown.value && countdownData.value) {
        if (countdownData.value.status === 'finished') {
            // 下班倒计时结束显示"下班"，自定义倒计时结束显示0
            return countdownData.value.mode === 'workEnd' ? '下班' : 0
        }
        return Math.max(0, countdownData.value.timestamp)
    }
    return 0
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
        customCountdown.value = rustConfig.customCountdown
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

const openSettings = () => {
    showSettings.value = true
}

const closeSettings = () => {
    showSettings.value = false
}

// 处理"知道了"按钮点击
const handleGotIt = async () => {
    console.log('点击了知道了')
    // 如果既不在结束状态也不在最后倒计时阶段，则不处理
    if (!isInEndState.value && !shouldShowFinalCountdown.value) {
        return
    }

    // 清除结束状态定时器
    if (endStateTimer.value) {
        clearTimeout(endStateTimer.value)
        endStateTimer.value = null
    }

    // 退出结束状态
    isInEndState.value = false
    isInFinalCountdown.value = false

    // 如果是下班倒计时结束，切换到下一天的倒计时
    if (countdownData.value?.mode === 'workEnd' && countdownData.value?.status === 'finished') {
        try {
            // 调用后端重置下班倒计时到下一天
            await invoke('reset_work_end_countdown_to_next_day')
            console.log('✅ [前端] 下班倒计时已重置到下一天')
            
            // 重置前端状态
            countdownData.value = {
                mode: 'workEnd',
                timestamp: 0,
                target_info: '请设置下班时间',
                status: 'reset'
            }
        } catch (error) {
            console.error('❌ [前端] 重置下班倒计时失败:', error)
        }
    }

    // 如果是自定义倒计时结束，重置倒计时
    if (countdownData.value?.mode === 'custom' && countdownData.value?.status === 'finished') {
        try {
            await invoke('reset_custom_countdown')
            console.log('✅ [前端] 自定义倒计时已重置')
            
            // 重置前端状态
            countdownData.value = {
                mode: 'custom',
                timestamp: 0,
                target_info: '请设置目标时间',
                status: 'reset'
            }
        } catch (error) {
            console.error('❌ [前端] 重置自定义倒计时失败:', error)
        }
    }
}

// 开始结束状态保持定时器
const startEndStateTimer = () => {
    if (!config.value) return

    const keepMinutes = config.value.endStateKeepMinutes || 5
    const keepMilliseconds = keepMinutes * 60 * 1000

    console.log(`🕐 [前端] 开始结束状态保持定时器，将保持 ${keepMinutes} 分钟`)

    endStateTimer.value = setTimeout(() => {
        console.log('⏰ [前端] 结束状态保持时间到，自动退出结束状态')
        handleGotIt()
    }, keepMilliseconds)
}

// 设置倒计时事件监听
const setupCountdownListener = async () => {
    try {
        unlistenCountdown = await listen('countdown-update', (event) => {
            const newData = event.payload as CountdownData
            const wasInFinalCountdown = shouldShowFinalCountdown.value
            const oldData = countdownData.value
            
            // 如果当前处于结束状态，忽略后端的倒计时更新
            // 这样可以避免重置后立即被后端数据覆盖
            if (isInEndState.value) {
                console.log('🚫 [前端] 当前处于结束状态，忽略倒计时更新')
                return
            }
            
            countdownData.value = newData

            // 检查是否刚进入最后倒计时阶段
            if (!wasInFinalCountdown && shouldShowFinalCountdown.value) {
                isInFinalCountdown.value = true
                // 这里可以添加音效或其他效果
                console.log('进入最后倒计时阶段！')

                // 只有在当前模式为'current'时才自动切换到对应的倒计时模式
                // 这样可以避免干扰用户的手动模式选择
                if (newData.mode && modeStore.currentMode === 'current' && newData.mode !== 'current') {
                    console.log(`自动切换模式从 ${modeStore.currentMode} 到 ${newData.mode}`)
                    modeStore.switchMode(newData.mode)
                }
            }

            // 检查自定义倒计时是否从finished状态变为reset状态（重置）
            if (oldData && oldData.mode === 'custom' && oldData.status === 'finished' &&
                newData.mode === 'custom' && newData.status === 'reset') {
                console.log('自定义倒计时已重置')
                isInFinalCountdown.value = false
            }

            // 检查是否倒计时结束
            if (newData.status === 'finished' && oldData?.status !== 'finished') {
                console.log('倒计时结束！')
                // 进入结束状态
                isInEndState.value = true
                // 开始结束状态保持定时器
                startEndStateTimer()
                // 这里可以添加结束音效或其他效果
            }
        })
    } catch (error) {
        console.error('Failed to setup countdown listener:', error)
    }
}

// 使用 composable 中的 updateTimeForDefaultDisplay 方法替代

// 使用 composable 中的 calculateNextHoliday 方法
const updateNextHoliday = () => {
    nextHoliday.value = getNextHoliday()
}



onMounted(async () => {
    await loadConfig();
    await setupCountdownListener();

    // 启动倒计时服务
    try {
        await invoke('start_countdown_timer');
    } catch (error) {
        console.error('Failed to start countdown timer:', error);
    }

    // 启动时间更新定时器
    startTimer(updateTimeForDefaultDisplay);
    updateNextHoliday();
});

onUnmounted(() => {
    stopTimer();
    if (unlistenCountdown) {
        unlistenCountdown();
    }
    // 清理结束状态定时器
    if (endStateTimer.value) {
        clearTimeout(endStateTimer.value);
        endStateTimer.value = null;
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
    padding: 30px;
    text-align: center;
    overflow: hidden;
    position: relative;
}



// 时间容器
.time-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-bottom: 20px;
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
    margin: 15px 0 0 0;
    max-width: 500px;
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
}
</style>