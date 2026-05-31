import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      redirect: () => {
        const auth = useAuthStore()
        if (!auth.isAuthenticated) return '/login'
        return auth.isAdmin ? '/admin/dashboard' : '/app/reservoirs'
      },
    },
    {
      path: '/login',
      component: () => import('../views/LoginView.vue'),
      meta: { public: true },
    },
    {
      path: '/admin',
      component: () => import('../layouts/AppLayout.vue'),
      meta: { requiresAuth: true, requiresAdmin: true },
      children: [
        { path: '', redirect: '/admin/dashboard' },
        { path: 'dashboard',  component: () => import('../views/admin/DashboardView.vue') },
        { path: 'users',      component: () => import('../views/admin/UsersView.vue') },
        { path: 'users/:id',  component: () => import('../views/admin/UserDetailView.vue') },
        { path: 'audit-logs', component: () => import('../views/admin/AuditLogsView.vue') },
      ],
    },
    {
      path: '/app',
      component: () => import('../layouts/AppLayout.vue'),
      meta: { requiresAuth: true },
      children: [
        { path: '', redirect: '/app/reservoirs' },
        { path: 'reservoirs',     component: () => import('../views/app/ReservoirsView.vue') },
        { path: 'reservoirs/:id', component: () => import('../views/app/ReservoirDetailView.vue') },
        { path: 'devices',        component: () => import('../views/app/DevicesView.vue') },
        { path: 'alerts',         component: () => import('../views/app/AlertsView.vue') },
        { path: 'profile',        component: () => import('../views/app/ProfileView.vue') },
      ],
    },
  ],
})

router.beforeEach(async to => {
  const auth = useAuthStore()

  if (auth.token && !auth.user) {
    await auth.fetchMe()
  }

  if (to.meta.public) return true
  if (!auth.isAuthenticated) return '/login'
  if (to.meta.requiresAdmin && !auth.isAdmin) return '/login'

  return true
})

export default router
