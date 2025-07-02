<template>
  <div class="empty-container">
    <div class="empty-content">
      <div class="empty-icon">
        📝
      </div>
      <h3 class="empty-title"><slot></slot></h3>
    </div>
  </div>
</template>

<script setup lang="ts">
// Empty component for showing no data state
</script>

<style scoped>
.empty-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  /* padding: 20px 10px; */
}

.empty-content {
  text-align: center;
  max-width: 300px;
}

.empty-icon {
  font-size: 64px;
  opacity: 0.6;
  animation: float 3s ease-in-out infinite;
}

.empty-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 12px 0;
}

.empty-description {
  font-size: 14px;
  color: var(--text-secondary);
  line-height: 1.5;
  margin: 0;
  opacity: 0.8;
}

@keyframes float {
  0%, 100% {
    transform: translateY(0px);
  }
  50% {
    transform: translateY(-10px);
  }
}
</style>