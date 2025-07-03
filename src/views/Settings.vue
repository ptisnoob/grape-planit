<template>
  <div class="settings-container">
    <div class="settings-header">
      <h1>设置</h1>
    </div>
    <div class="settings-content">
      <p>设置内容将在这里显示...</p>
    </div>
  </div>
</template>

<script setup lang="ts">
// 设置页面逻辑将在这里添加
</script>

<style lang="scss" scoped>
.settings-container {
  width: 100%;
  height: 100vh;
  background: var(--bg-primary);
  color: var(--text-primary);
  padding: 20px;
  box-sizing: border-box;
}

.settings-header {
  margin-bottom: 30px;
  
  h1 {
    font-size: 24px;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }
}

.settings-content {
  p {
    font-size: 16px;
    color: var(--text-secondary);
    line-height: 1.6;
  }
}
</style>