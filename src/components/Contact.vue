<template>
<!-- Contact Form -->
<section class="relative overflow-hidden  bg-gradient-to-b from-white via-blue-50/80 to-transparent py-16 sm:py-24 scroll-mt-[0vh] flex items-center justify-center" id="contact">
  <div class="lg:w-3/5 rounded-3xl shadow-2xl p-8 h-full flex flex-col justify-center bg-white backdrop-blur-sm mb-0" data-aos="fade-up" data-aos-delay="300">
    <h2 class="text-2xl font-bold text-gray-800 mb-6 text-center">Send Us a Message</h2>
    <form class="space-y-4">
      <fieldset class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div>
          <label for="name" class="block text-gray-700 font-medium mb-2">Full Name</label>
          <input
            type="text"
            id="name"
            name="name"
            class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-900 focus:border-transparent"
            placeholder="Your name"
            required>
        </div>
        <div>
          <label for="phone" class="block text-gray-700 font-medium mb-2">Phone Number</label>
          <input
            type="tel"
            id="phone"
            name="phone"
            class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-900 focus:border-transparent"
            placeholder="Your phone number"
            required>
        </div>
      </fieldset>
      <fieldset class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div>
          <label for="email" class="block text-gray-700 font-medium mb-2">Email Address</label>
          <input
            type="email"
            id="email"
            name="email"
            class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-900 focus:border-transparent"
            placeholder="Your email address"
            required>
        </div>
        <div>
          <label for="company" class="block text-gray-700 font-medium mb-2">Company / Affiliation</label>
          <input
            type="text"
            id="company"
            name="company"
            class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-900 focus:border-transparent"
            placeholder="Your company / affiliation"
            required>
        </div>
        </fieldset>
        <div>
          <label for="service" class="block text-gray-700 font-medium mb-2">
          Service Interested In
          </label>
          <select
            id="service"
            name="service"
            class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            required
            >
            <option value="">Select a service</option>
            <option v-for="s in services" :key="s.value" :value="s.value">
                {{ s.label }}
            </option>
            </select>
        </div>
        <div>
        <label for="message" class="block text-gray-700 font-medium mb-2">
            Your Message
        </label>

        <textarea
            id="message"
            name="message"
            rows="5"
            placeholder="Tell us about your concerns or inquiries..."
            class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            required
        ></textarea>
        </div>
        <div class="flex items-center justify-center">
        <button
            type="submit"
            class=" bg-blue-900 text-white px-6 py-3 rounded-full shadow-lg hover:bg-blue-800 transition w-full md:w-auto"
            aria-label="Submit contact form">
            Send Message
        </button>
        </div>
    </form>
  </div>
</section>
</template>
<script setup>
import { ref } from 'vue';
const services = ref([
    { label: 'Real-Time Sea State Monitoring', value: 'real-time-sea-state-monitoring' },
    { label: 'Predictive Analytics for Offshore Operations', value: 'predictive-analytics-offshore-operations' },
    { label: 'Custom Data Solutions', value: 'custom-data-solutions' },
    { label: 'Nusawave Academy', value: 'nusawave-academy' },
    { label: 'Other Inquiries', value: 'other-inquiries' },
]);
</script>