<template>
  <header class="bg-gradient-to-r from-blue-300 to-blue-100 shadow-md fixed w-full z-50">
    <div class="max-w-screen-xl mx-auto px-4 py-4 flex justify-between items-center">
      <!-- Logo -->
      <a href="/" class="flex items-center">
        <h1 class="text-2xl font-bold text-gray-900">Nusawave Intelligence</h1>
      </a>

      <!-- Desktop Navigation -->
      <nav class="hidden md:flex space-x-6 relative" aria-label="Primary navigation">
        <div
          v-for="(link, index) in navLinks"
          :key="index"
          class="relative group"
          @mouseenter="handleEnter(link.label)"
          @mouseleave="handleLeave"
        >
          <!-- Top-level link -->
          <button
            v-if="link.children"
            class="flex items-center gap-1 font-bold text-gray-700 hover:text-blue-900 transition-colors duration-200 
                   focus:outline-none focus:ring-2 focus:ring-blue-600 rounded cursor-pointer select-none"
          >
            {{ link.label }}
            <Icon
              icon="mdi:chevron-down"
              class="w-4 h-4 transition-transform duration-300 ease-in-out"
              :class="{ 'rotate-180 text-blue-800': activeDropdown === link.label }"
            />
          </button>

          <a
            v-else
            :href="link.href"
            class="text-gray-700 font-bold hover:text-blue-900 transition-colors duration-200 
                   focus:outline-none focus:ring-2 focus:ring-blue-600 rounded cursor-pointer select-none"
          >
            {{ link.label }}
          </a>

          <!-- Dropdown Menu -->
          <transition
            enter-active-class="transition-all ease-out duration-300"
            enter-from-class="opacity-0 -translate-y-2 scale-95"
            enter-to-class="opacity-100 translate-y-0 scale-100"
            leave-active-class="transition-all ease-in duration-200"
            leave-from-class="opacity-100 translate-y-0 scale-100"
            leave-to-class="opacity-0 -translate-y-2 scale-95"
          >
            <div
              v-if="activeDropdown === link.label && link.children"
              class="absolute left-0 mt-3 w-52 bg-white shadow-xl rounded-lg py-2 z-50 border border-gray-100"
            >
              <a
                v-for="(child, i) in link.children"
                :key="i"
                :href="child.href"
                class="block px-5 py-2 text-gray-700 hover:bg-blue-50 hover:text-blue-800 
                       transition-colors duration-150 cursor-pointer"
              >
                {{ child.label }}
              </a>
            </div>
          </transition>
        </div>
      </nav>

      <!-- Inquiry button (Desktop) -->
      <div class="hidden md:block">
        <a
          href="#inquiry"
          class="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 transition-colors duration-200 
                 focus:outline-none focus:ring-2 focus:ring-blue-600 focus:ring-offset-2 cursor-pointer"
        >
          Inquiry
        </a>
      </div>

      <!-- Mobile Menu Button -->
      <button
        class="md:hidden p-2 focus:outline-none focus:ring-2 focus:ring-blue-600 rounded"
        @click="toggleMenu"
        aria-label="Toggle menu"
        :aria-expanded="isMenuOpen.toString()"
        aria-controls="mobile-menu"
      >
        <Icon :icon="isMenuOpen ? 'mdi:close' : 'mdi:menu'" class="w-6 h-6" />
      </button>
    </div>

    <!-- Mobile Navigation -->
    <aside
      v-if="isMenuOpen"
      id="mobile-menu"
      class="md:hidden bg-white shadow-md py-4 px-4 absolute top-full left-0 right-0 z-40"
      aria-label="Mobile menu"
    >
      <nav aria-label="Mobile Navigation">
        <ul class="flex flex-col space-y-4">
          <li v-for="(link, index) in navLinks" :key="index">
            <div>
              <button
                v-if="link.children"
                @click="toggleSubmenu(link.label)"
                class="flex justify-between items-center w-full text-left text-gray-800 hover:text-blue-900 
                       transition-colors duration-200 cursor-pointer"
              >
                {{ link.label }}
                <Icon
                  :icon="openSubmenu === link.label ? 'mdi:chevron-up' : 'mdi:chevron-down'"
                  class="w-5 h-5 transition-transform duration-200"
                />
              </button>

              <a
                v-else
                :href="link.href"
                class="block text-gray-800 hover:text-blue-900 transition-colors duration-200 cursor-pointer"
                @click="isMenuOpen = false"
              >
                {{ link.label }}
              </a>

              <!-- Submenu mobile -->
              <transition
                enter-active-class="transition duration-200 ease-out"
                enter-from-class="opacity-0 -translate-y-2"
                enter-to-class="opacity-100 translate-y-0"
                leave-active-class="transition duration-150 ease-in"
                leave-from-class="opacity-100 translate-y-0"
                leave-to-class="opacity-0 -translate-y-2"
              >
                <ul v-if="openSubmenu === link.label" class="pl-4 mt-2 space-y-2 border-l border-blue-100">
                  <li v-for="(child, i) in link.children" :key="i">
                    <a
                      :href="child.href"
                      class="block text-gray-700 hover:text-blue-800 transition-colors duration-150 cursor-pointer"
                      @click="isMenuOpen = false"
                    >
                      {{ child.label }}
                    </a>
                  </li>
                </ul>
              </transition>
            </div>
          </li>

          <li>
            <a
              href="#inquiry"
              class="block bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 transition-colors duration-200 
                     focus:outline-none focus:ring-2 focus:ring-blue-600 cursor-pointer"
              @click="isMenuOpen = false"
            >
              Inquiry
            </a>
          </li>
        </ul>
      </nav>
    </aside>
  </header>
</template>

<script setup>
import { ref } from 'vue'
import { Icon } from '@iconify/vue'

const navLinks = ref([
  { href: '#home', label: 'Home' },
  { href: '#about', label: 'About' },
  {
    href: '#services',
    label: 'Services',
    children: [
      { href: '#academy', label: 'Nusawave Academy' },
      { href: '#data-intelligence', label: 'Data Intelligence' },
    ],
  },
  {
    href: '#outreach',
    label: 'Outreach',
    children: [
      { href: '#openlab', label: 'Nusawave Openlab' },
      { href: '#forecast', label: 'Nusawave Forecast' },
    ],
  },
  { href: '#blog', label: 'Blog' },
  // { href: '#contact', label: 'Contact' },
])

const isMenuOpen = ref(false)
const activeDropdown = ref(null)
const openSubmenu = ref(null)
let hoverTimeout = null

function toggleMenu() {
  isMenuOpen.value = !isMenuOpen.value
}

function toggleSubmenu(label) {
  openSubmenu.value = openSubmenu.value === label ? null : label
}

function handleEnter(label) {
  clearTimeout(hoverTimeout)
  activeDropdown.value = label
}

function handleLeave() {
  hoverTimeout = setTimeout(() => {
    activeDropdown.value = null
  }, 120) // sedikit delay agar lebih smooth saat keluar
}
</script>
