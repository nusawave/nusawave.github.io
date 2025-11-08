<template>
  <div>
    <Header />
    <Hero />
    <About />
    <Footer />
  </div>
</template>

<script setup>  
import { onMounted } from 'vue';
import Header from '@components/Header.vue';
import AOS from 'aos';
import 'aos/dist/aos.css';
import Hero from '@components/Hero.vue';
import About from '@components/About.vue';
import Footer from '@components/Footer.vue';

onMounted(()=>{
  AOS.init();
})
</script>