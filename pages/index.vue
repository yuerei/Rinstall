<template>
  <div class="min-h-screen bg-gray-900 flex flex-col md:flex-row p-8 gap-8 text-white">
    <!-- Left Sidebar -->
    <div class="flex flex-col gap-6 w-full md:w-64 xl:w-1/5">
      <button
          v-for="(game, index) in games"
          :key="index"
          :class="selectedGame === index ? 'border-fuchsia-400 bg-gray-800 transform scale-105' : 'border-gray-700 bg-gray-800'"
          class="p-6 text-center rounded-lg cursor-pointer transition border-2 w-full focus:outline-none focus:ring-2 focus:ring-fuchsia-400"
          :aria-pressed="selectedGame === index"
          @click="selectGame(index)"
        >
        {{ game.name }}
    </button>
    </div>

    <!-- Right Content -->
    <div class="flex-1 border-2 border-gray-700 rounded-lg relative p-8 bg-gray-800 overflow-hidden">
      <Transition name="fade-slide" mode="out-in">
        <div :key="selectedGame" class="h-full flex flex-col justify-between">
          <h2 class="text-2xl font-bold mb-4">{{ games[selectedGame].description }}</h2>

          <button
            class="self-end mt-auto px-6 py-2 border-2 border-fuchsia-400 rounded-lg hover:bg-fuchsia-400 hover:text-black transition"
            @click="goToGame(games[selectedGame].slug)"
          >
            GO
          </button>
        </div>
      </Transition>
    </div>
    
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

onMounted(() => {
  const isLoggedIn = localStorage.getItem('isLoggedIn')
  if (isLoggedIn !== 'true') {
    router.push('/login')
  }
})

const games = ref([
  { name: 'R.E.P.O', slug: "repo", description: 'Sigma Sigma Boy' },
  { name: 'GAME 2', slug: "#", description: 'GAME 2 DESC' },
  { name: 'GAME 3', slug: "#", description: 'GAME 3 DESC' },
  { name: 'GAME 4', slug: "#", description: 'GAME 4 DESC' },
])

const selectedGame = ref(0)

const selectGame = (index: number) => {
  selectedGame.value = index
}

const goToGame = (slug?: string) => {
  if (!slug) return
  router.push(`/game/${slug}`)
}
</script>