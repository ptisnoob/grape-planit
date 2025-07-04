import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'
import Layout from '@/layout/index.vue'

// 定义路由
const routes: RouteRecordRaw[] = [
    {
        path: '/',
        component: Layout,
        children: [
            {
                path: '',
                name: 'Home',
                component: () => import('@/views/Home.vue')
            },
            {
                path: 'list',
                name: 'List',
                component: () => import('@/views/list.vue')
            },
            {
                path: 'calendar',
                name: 'Calendar',
                component: () => import('@/views/Calendar.vue')
            },
            {
                path: 'about',
                name: 'About',
                component: () => import('@/views/About.vue')
            },
            {
                path: 'add',
                name: 'Add',
                component: () => import('@/views/AddTodo.vue')
            }
        ]
    },
    {
        path: '/settings',
        name: 'Settings',
        component: () => import('@/views/Settings/index.vue')
    }
]

// 创建路由实例
const router = createRouter({
    history: createWebHistory(),
    routes
})

export default router