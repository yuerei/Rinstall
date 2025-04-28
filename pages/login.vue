<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-900">
    <Transition name="fade-scale" appear>
      <div v-if="showLogin" class="bg-gray-800 p-8 rounded-2xl shadow-2xl w-full max-w-md">
        <h1 class="text-3xl font-bold mb-6 text-center text-white">Login</h1>

        <form class="space-y-4" @submit.prevent="handleLogin">
          <div>
            <label class="block text-sm font-medium text-gray-300">Username</label>
            <!-- eslint-disable-next-line vue/html-self-closing -->
            <input
              v-model="username"
              type="text"
              required
              class="mt-1 w-full px-4 py-2 border border-gray-700 rounded-lg bg-gray-700 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-300">Password</label>
            <!-- eslint-disable-next-line vue/html-self-closing -->
            <input
              v-model="password"
              type="password"
              required
              class="mt-1 w-full px-4 py-2 border border-gray-700 rounded-lg bg-gray-700 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <button
            type="submit"
            class="w-full bg-fuchsia-500 hover:bg-fuchsia-600 text-white py-2 rounded-lg transition"
          >
            Login
          </button>
        </form>

        <p v-if="error" class="text-red-400 text-center mt-4">{{ error }}</p>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'

const username = ref('')
const password = ref('')
const error = ref('')
const showLogin = ref(false)
const router = useRouter()

onMounted(() => {
  showLogin.value = true // trigger the transition on mount
})

const handleLogin = async () => {
  error.value = ''

  if (username.value === 'admin' && password.value === 'password') {
    await router.push('/')
  } else {
    error.value = 'Invalid username or password'
  }
}
</script>

<style scoped>
.fade-scale-enter-active,
.fade-scale-leave-active {
  transition: all 0.5s ease;
}
.fade-scale-enter-from {
  opacity: 0;
  transform: scale(0.95);
}
.fade-scale-enter-to {
  opacity: 1;
  transform: scale(1);
}
.fade-scale-leave-from {
  opacity: 1;
  transform: scale(1);
}
.fade-scale-leave-to {
  opacity: 0;
  transform: scale(0.95);
}
</style>
