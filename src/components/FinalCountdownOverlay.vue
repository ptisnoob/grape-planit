<template>
    <transition enter-active-class="animate__animated animate__fadeIn animate__faster"
        leave-active-class="animate__animated animate__fadeOut animate__faster">
        <div v-if="finalCountdownStore.isVisible" class="final-countdown-overlay" @click="handleClick">
            <!-- 退出按钮 -->
            <div class="exit-button" @click.stop="handleExit" title="退出倒计时">
                <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M18 6L6 18M6 6L18 18" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                        stroke-linejoin="round" />
                </svg>
            </div>

            <div class="final-countdown-container">
                <div :key="displayValue" class="final-countdown-number animate__animated animate__heartBeat">
                    {{ displayValue }}
                </div>
                <div class="final-countdown-hint">
                    {{ hintText }}
                </div>
                <div class="final-countdown-tips">
                    点击任意位置继续 • 右上角退出
                </div>
            </div>
        </div>
    </transition>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { CountdownData } from '@/model/countdown'
import { useFinalCountdownStore } from '@/store/finalCountdown'
import { databaseApi } from '@/api/services'
import { useDatabase } from '@/composables/useDatabase'

const finalCountdownStore = useFinalCountdownStore()
const { loadConfigFromDb } = useDatabase()

let unlistenCountdown: (() => void) | null = null

// 计算显示的值
const displayValue = computed(() => {
    const countdownData = finalCountdownStore.countdownData
    if (!countdownData) return 0

    if (countdownData.status === 'finished') {
        // 倒计时结束显示对应的文本
        return countdownData.mode === 'workEnd' ? '下班' : '结束'
    }

    // 显示剩余秒数
    return Math.max(0, countdownData.timestamp)
})

// 计算提示文本
const hintText = computed(() => {
    const countdownData = finalCountdownStore.countdownData
    if (!countdownData) return ''

    if (countdownData.status === 'finished') {
        return '点击继续'
    }

    return '下班倒计时'
})

// 设置倒计时事件监听
const setupCountdownListener = async () => {
    try {
        console.log('🎧 [FinalCountdownOverlay] 开始设置倒计时事件监听器')
        unlistenCountdown = await listen('countdown-update', async (event) => {
            const newData = event.payload as CountdownData

            // 如果当前处于最后倒计时状态，忽略后端的倒计时更新
            if (finalCountdownStore.isInEndState) {
                console.log('🚫 [FinalCountdownOverlay] 当前处于结束状态，忽略倒计时更新')
                return
            }

            // 只处理下班倒计时数据
            if (newData.mode === 'workEnd') {
                // console.log('✅ [FinalCountdownOverlay] 更新下班倒计时数据')

                // 获取配置中的beforeTime
                try {
                    const config = await loadConfigFromDb()
                    if (!config) return
                    const beforeTime = (config.finalCountdownMinutes || 1) * 60
                    finalCountdownStore.updateCountdownData(newData, beforeTime)
                } catch (error) {
                    console.error('Failed to load config:', error)
                    // 使用默认值
                    finalCountdownStore.updateCountdownData(newData, 60)
                }
            }
        })
    } catch (error) {
        console.error('Failed to setup countdown listener:', error)
    }
}

const handleClick = async () => {
    console.log('点击了最后倒计时overlay')

    const countdownData = finalCountdownStore.countdownData

    // 如果是下班倒计时结束，重置到下一天
    if (countdownData?.mode === 'workEnd' && countdownData?.status === 'finished') {
        try {
            await databaseApi.countdown.resetWorkEndToNextDay()
            console.log('✅ [FinalCountdownOverlay] 下班倒计时已重置到下一天')
        } catch (error) {
            console.error('❌ [FinalCountdownOverlay] 重置下班倒计时失败:', error)
        }
        
        // 倒计时自然结束时，重置手动退出标志
        finalCountdownStore.resetManualExit()
    }

    // 隐藏overlay
    finalCountdownStore.hideOverlay()
}

// 处理退出按钮点击
const handleExit = () => {
    console.log('用户主动退出最后倒计时')
    finalCountdownStore.hideOverlay(true)
}

onMounted(async () => {
    await setupCountdownListener()
})

onUnmounted(() => {
    if (unlistenCountdown) {
        unlistenCountdown()
    }
})
</script>

<style scoped>
.final-countdown-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: radial-gradient(ellipse at center,
            rgba(15, 15, 35, 0.98) 0%,
            rgba(25, 25, 45, 0.98) 30%,
            rgba(35, 35, 55, 0.98) 60%,
            rgba(10, 10, 25, 0.99) 100%);
    backdrop-filter: blur(25px) saturate(1.2);
    z-index: 9999;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    user-select: none;
    animation: backgroundShift 4s ease-in-out infinite;
}

.exit-button {
    position: absolute;
    top: 30px;
    right: 30px;
    width: 48px;
    height: 48px;
    background: rgba(255, 255, 255, 0.1);
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: rgba(255, 255, 255, 0.8);
    cursor: pointer;
    transition: all 0.3s ease;
    backdrop-filter: blur(10px);
}

.exit-button:hover {
    background: rgba(255, 255, 255, 0.2);
    border-color: rgba(255, 255, 255, 0.6);
    color: white;
    transform: scale(1.1);
}

.final-countdown-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 30px;
    text-align: center;
    padding: 40px;
    /* background: rgba(255, 255, 255, 0.05); */
    /* border-radius: 30px; */
    /* border: 1px solid rgba(255, 255, 255, 0.1); */
    /* backdrop-filter: blur(15px); */
    /* box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3); */
}

.final-countdown-number {
    font-size: 160px;
    font-weight: 900;
    color: #ffffff;
    text-shadow:
        0 0 20px rgba(255, 255, 255, 0.8),
        0 0 40px rgba(255, 255, 255, 0.6),
        0 0 60px rgba(255, 255, 255, 0.4),
        0 0 80px rgba(100, 200, 255, 0.3);
    font-family: 'Arial Black', 'Helvetica', sans-serif;
    line-height: 0.9;
    min-height: 140px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(45deg, #ffffff, #e0e0ff, #ffffff);
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    filter: drop-shadow(0 0 20px rgba(255, 255, 255, 0.5));
}

.final-countdown-hint {
    font-size: 22px;
    color: rgba(255, 255, 255, 0.9);
    font-weight: 600;
    letter-spacing: 3px;
    text-transform: uppercase;
    text-shadow: 0 2px 10px rgba(0, 0, 0, 0.5);
}

.final-countdown-tips {
    font-size: 14px;
    color: rgba(255, 255, 255, 0.6);
    font-weight: 400;
    letter-spacing: 1px;
    margin-top: 10px;
}

/* 响应式设计 */
@media (max-width: 600px) {
    .final-countdown-number {
        font-size: 100px;
        min-height: 90px;
    }

    .final-countdown-hint {
        font-size: 18px;
        letter-spacing: 2px;
    }

    .final-countdown-tips {
        font-size: 12px;
    }

    .exit-button {
        top: 20px;
        right: 20px;
        width: 40px;
        height: 40px;
    }

    .final-countdown-container {
        padding: 30px 20px;
        gap: 20px;
    }
}

/* 背景动画 */
@keyframes backgroundShift {

    0%,
    100% {
        background: radial-gradient(ellipse at center,
                rgba(15, 15, 35, 0.98) 0%,
                rgba(25, 25, 45, 0.98) 30%,
                rgba(35, 35, 55, 0.98) 60%,
                rgba(10, 10, 25, 0.99) 100%);
    }

    50% {
        background: radial-gradient(ellipse at center,
                rgba(20, 15, 40, 0.98) 0%,
                rgba(30, 25, 50, 0.98) 30%,
                rgba(40, 35, 60, 0.98) 60%,
                rgba(15, 10, 30, 0.99) 100%);
    }
}

/* 数字脉冲效果增强 */
.final-countdown-number.animate__heartBeat {
    animation: enhancedHeartBeat 1.5s ease-in-out infinite;
}

@keyframes enhancedHeartBeat {
    0% {
        transform: scale(1);
        text-shadow:
            0 0 20px rgba(255, 255, 255, 0.8),
            0 0 40px rgba(255, 255, 255, 0.6),
            0 0 60px rgba(255, 255, 255, 0.4),
            0 0 80px rgba(100, 200, 255, 0.3);
    }

    14% {
        transform: scale(1.05);
        text-shadow:
            0 0 25px rgba(255, 255, 255, 0.9),
            0 0 50px rgba(255, 255, 255, 0.7),
            0 0 75px rgba(255, 255, 255, 0.5),
            0 0 100px rgba(100, 200, 255, 0.4);
    }

    28% {
        transform: scale(1);
        text-shadow:
            0 0 20px rgba(255, 255, 255, 0.8),
            0 0 40px rgba(255, 255, 255, 0.6),
            0 0 60px rgba(255, 255, 255, 0.4),
            0 0 80px rgba(100, 200, 255, 0.3);
    }

    42% {
        transform: scale(1.05);
        text-shadow:
            0 0 25px rgba(255, 255, 255, 0.9),
            0 0 50px rgba(255, 255, 255, 0.7),
            0 0 75px rgba(255, 255, 255, 0.5),
            0 0 100px rgba(100, 200, 255, 0.4);
    }

    70% {
        transform: scale(1);
        text-shadow:
            0 0 20px rgba(255, 255, 255, 0.8),
            0 0 40px rgba(255, 255, 255, 0.6),
            0 0 60px rgba(255, 255, 255, 0.4),
            0 0 80px rgba(100, 200, 255, 0.3);
    }
}
</style>