<template>
  <div class="home">
    <div class="hero-section animate__animated animate__fadeInDown">
      <h1 class="hero-title">欢迎使用 Grape PlanIt</h1>
      <p class="hero-subtitle animate__animated animate__fadeInUp animate__delay-1s">
        这是一个基于 Vue 3 + TypeScript + Tauri 的桌面应用
      </p>
    </div>
    
    <nav class="main-nav animate__animated animate__fadeInLeft animate__delay-2s">
      <router-link to="/" class="nav-link">🏠 首页</router-link>
      <router-link to="/about" class="nav-link">📖 关于</router-link>
    </nav>

    <!-- 演示基础样式 -->
    <section class="demo-section card animate__animated animate__fadeInUp animate__delay-3s">
      <h2>📝 表单演示</h2>
      <form class="demo-form">
        <div class="form-group">
          <input type="text" placeholder="请输入文本" class="input" />
        </div>
        <div class="form-group">
          <input type="email" placeholder="请输入邮箱" class="input" />
        </div>
        <button type="submit" class="btn animate__animated animate__pulse animate__infinite">✨ 提交</button>
      </form>
    </section>

    <section class="demo-section card animate__animated animate__fadeInUp animate__delay-4s">
      <h2>📋 列表演示</h2>
      <ul class="demo-list">
        <li class="list-item animate__animated animate__fadeInRight animate__delay-5s">🎯 列表项 1</li>
        <li class="list-item animate__animated animate__fadeInRight animate__delay-6s">🚀 列表项 2</li>
        <li class="list-item animate__animated animate__fadeInRight animate__delay-7s">⭐ 列表项 3</li>
      </ul>
    </section>

    <section class="demo-section card animate__animated animate__fadeInUp animate__delay-8s">
      <h2>🔗 链接演示</h2>
      <p class="link-demo">
        这是一个 <a href="#" class="demo-link">普通链接</a>，
        这是一个 <a href="#" class="demo-link external">外部链接</a>
      </p>
    </section>

    <!-- 新增功能卡片 -->
    <section class="features-grid animate__animated animate__fadeIn animate__delay-9s">
      <div class="feature-card card animate__animated animate__bounceIn animate__delay-10s">
        <div class="feature-icon">🎨</div>
        <h3>主题切换</h3>
        <p>支持浅色/深色/自动主题</p>
      </div>
      <div class="feature-card card animate__animated animate__bounceIn animate__delay-11s">
        <div class="feature-icon">✨</div>
        <h3>动画效果</h3>
        <p>丰富的Animate.css动画</p>
      </div>
      <div class="feature-card card animate__animated animate__bounceIn animate__delay-12s">
        <div class="feature-icon">🖥️</div>
        <h3>桌面应用</h3>
        <p>基于Tauri的原生体验</p>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
// 组件逻辑
</script>

<style scoped>
.home {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
  min-height: 100vh;
}

/* 英雄区域 */
.hero-section {
  text-align: center;
  margin-bottom: 40px;
  padding: 40px 20px;
}

.hero-title {
  font-size: 3rem;
  font-weight: 700;
  background: linear-gradient(135deg, var(--accent-color), #ff6b6b);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  margin-bottom: 16px;
  text-shadow: 0 4px 8px var(--shadow);
}

.hero-subtitle {
  font-size: 1.2rem;
  color: var(--text-secondary);
  margin-bottom: 0;
}

/* 导航 */
.main-nav {
  display: flex;
  justify-content: center;
  gap: 20px;
  margin: 40px 0;
}

.nav-link {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  text-decoration: none;
  border-radius: 25px;
  font-weight: 500;
  transition: all var(--transition-normal);
  border: 1px solid var(--border-color);
  backdrop-filter: blur(10px);
}

.nav-link:hover {
  background: var(--accent-color);
  color: white;
  transform: translateY(-2px) scale(1.05);
  box-shadow: 0 8px 20px var(--shadow);
}

.nav-link.router-link-active {
  background: var(--accent-color);
  color: white;
  box-shadow: 0 4px 12px rgba(0, 123, 255, 0.3);
}

/* 演示区域 */
.demo-section {
  margin: 30px 0;
  animation-duration: 0.8s;
}

.demo-section h2 {
  margin-top: 0;
  margin-bottom: 20px;
  color: var(--text-primary);
  font-size: 1.5rem;
  display: flex;
  align-items: center;
  gap: 8px;
}

/* 表单样式 */
.demo-form {
  display: flex;
  flex-direction: column;
  gap: 16px;
  max-width: 400px;
}

.form-group {
  display: flex;
  flex-direction: column;
}

/* 列表样式 */
.demo-list {
  list-style: none;
  padding: 0;
}

.list-item {
  padding: 12px 16px;
  margin: 8px 0;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  color: var(--text-primary);
  transition: all var(--transition-normal);
  cursor: pointer;
}

.list-item:hover {
  background: var(--accent-color);
  color: white;
  transform: translateX(10px);
  box-shadow: 0 4px 12px var(--shadow);
}

/* 链接样式 */
.link-demo {
  font-size: 1.1rem;
  line-height: 1.6;
}

.demo-link {
  color: var(--accent-color);
  text-decoration: none;
  font-weight: 500;
  transition: all var(--transition-fast);
  position: relative;
}

.demo-link:hover {
  color: var(--text-primary);
  text-shadow: 0 0 8px var(--accent-color);
}

.demo-link.external::after {
  content: " ↗";
  font-size: 12px;
  opacity: 0.7;
}

/* 功能卡片网格 */
.features-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 20px;
  margin-top: 40px;
}

.feature-card {
  text-align: center;
  padding: 30px 20px;
  transition: all var(--transition-normal);
  cursor: pointer;
}

.feature-card:hover {
  transform: translateY(-8px) scale(1.02);
  box-shadow: 0 12px 24px var(--shadow);
}

.feature-icon {
  font-size: 3rem;
  margin-bottom: 16px;
  display: block;
}

.feature-card h3 {
  color: var(--text-primary);
  margin: 16px 0 8px;
  font-size: 1.2rem;
}

.feature-card p {
  color: var(--text-secondary);
  margin: 0;
  font-size: 0.9rem;
}

/* 动画延迟调整 */
.animate__delay-1s { animation-delay: 0.2s; }
.animate__delay-2s { animation-delay: 0.4s; }
.animate__delay-3s { animation-delay: 0.6s; }
.animate__delay-4s { animation-delay: 0.8s; }
.animate__delay-5s { animation-delay: 1.0s; }
.animate__delay-6s { animation-delay: 1.2s; }
.animate__delay-7s { animation-delay: 1.4s; }
.animate__delay-8s { animation-delay: 1.6s; }
.animate__delay-9s { animation-delay: 1.8s; }
.animate__delay-10s { animation-delay: 2.0s; }
.animate__delay-11s { animation-delay: 2.2s; }
.animate__delay-12s { animation-delay: 2.4s; }

/* 响应式设计 */
@media (max-width: 768px) {
  .hero-title {
    font-size: 2rem;
  }
  
  .hero-subtitle {
    font-size: 1rem;
  }
  
  .main-nav {
    flex-direction: column;
    align-items: center;
  }
  
  .features-grid {
    grid-template-columns: 1fr;
  }
  
  .demo-form {
    max-width: 100%;
  }
}

@media (max-width: 480px) {
  .home {
    padding: 16px;
  }
  
  .hero-section {
    padding: 20px 16px;
  }
  
  .nav-link {
    padding: 10px 20px;
    font-size: 14px;
  }
}
</style>