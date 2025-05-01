<template>
    <UApp>
      <NuxtPage />
    </UApp>
</template>

<script setup lang="ts">
document.addEventListener('contextmenu', event => event.preventDefault());
</script>

<style>
@import '@/assets/css/animations.css';

::-webkit-scrollbar {
  width: 7px;
}

::-webkit-scrollbar-thumb {
  background-color: #d946ef; /* fuchsia-400 */
  border-radius: 10px;
  border: 1px solid transparent; /* invisible padding effect */
  background-clip: padding-box; /* keeps thumb slim */
}

::-webkit-scrollbar-thumb:hover {
  background: #d946ef; /* fuchsia-600 on hover */
}

html, body {
  scrollbar-gutter: stable; /* Prevents layout shift when scrollbar appears */
  background-color: #111827; /* Tailwind's bg-gray-900 */
  margin: 0;
  padding: 0;
  height: 100%;
}
</style>
