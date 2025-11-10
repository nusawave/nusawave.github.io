<template>
  <article class="max-w-4xl mx-auto py-16 px-6 prose prose-blue pt-40" >
    <h1 class="text-4xl font-bold text-blue-900 mb-6">{{ post.meta.title }}</h1>
    <p class="text-gray-500 mb-4">{{ post.meta.date }}</p>

    <img
      v-if="post.meta.image"
      :src="post.meta.image"
      :alt="post.meta.title"
      class="rounded-xl mb-6 w-full h-64 object-cover"
    />

    <div v-html="post.html"></div>

    <RouterLink
      to="/blog"
      class="inline-block mt-10 text-blue-700 font-semibold hover:underline"
    >
      ← Back to Blog
    </RouterLink>
  </article>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { marked } from 'marked'

const route = useRoute()
const post = ref({ meta: {}, html: '' })

// Load all markdown files
const markdownFiles = import.meta.glob('../blog/*.md', { eager: true })

onMounted(() => {
  const slug = route.params.slug
  const file = markdownFiles[`../blog/${slug}.md`]

  if (file) {
    post.value.meta = file.frontmatter || {}
    post.value.html = marked.parse(file.default || '')
  } else {
    post.value.html = '<p>Post not found.</p>'
  }
})
</script>
