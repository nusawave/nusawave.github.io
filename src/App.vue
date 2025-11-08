<template>
  <div>
    <Header />
    <Hero />
  </div>
</template>

<script setup>
import { onMounted } from 'vue';
import Header from '@components/Header.vue';
import AOS from 'aos';
import 'aos/dist/aos.css';
import Hero from '@components/Hero.vue';

onMounted(()=>{
  AOS.init();
})
</script>