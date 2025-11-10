<template>
  <div class="flex flex-col min-h-screen bg-white">
    <Header />

    <main class="flex-grow">
      <!-- ✅ Transition wrapper -->
      <transition
        name="fade-slide"
        mode="out-in"
        appear
      >
        <router-view />
      </transition>
    </main>

    <Footer />
  </div>
</template>

<script setup>
import Header from '@components/Header.vue'
import Footer from '@components/Footer.vue'
</script>

<style scoped>
/* Global smooth page transition */

.fade-slide-enter-active,
.fade-slide-leave-active {
  transition: all 0.7s ease;
}

.fade-slide-enter-from {
  opacity: 0;
  transform: translateY(20px);
}

.fade-slide-enter-to {
  opacity: 1;
  transform: translateY(0);
}

.fade-slide-leave-from {
  opacity: 1;
  transform: translateY(0);
}

.fade-slide-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}
</style>