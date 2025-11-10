import { createApp } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'
import './style.css'
import App from './App.vue'

import Home from '@components/Home.vue'
import BlogList from '@components/BlogList.vue'
import BlogPost from '@components/BlogPost.vue'

const routes = [
  { path: '/', name: 'Home', component: Home },
  { path: '/blog', name: 'BlogList', component: BlogList },
  { path: '/blog/:slug', name: 'BlogPost', component: BlogPost },
]

const router = createRouter({
  history: createWebHistory(),
  routes,

  // ✅ Custom scroll behavior
  scrollBehavior(to, from, savedPosition) {
    // Always reset scroll when going to Home
    if (to.path === '/') {
      return { top: 0 }
    }

    // Restore saved position when using back/forward
    if (savedPosition) {
      return savedPosition
    }

    // If navigating to an anchor (#something)
    if (to.hash) {
      return {
        el: to.hash,
        behavior: 'smooth',
      }
    }

    // Default: scroll to top
    return { top: 0 }
  },
})

createApp(App).use(router).mount('#app')
