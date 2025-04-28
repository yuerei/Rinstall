<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-900">
    <Transition name="fade-scale" appear>
      <div v-if="showLogin" class="bg-gray-800 p-8 rounded-2xl shadow-2xl w-full max-w-md">
        <h1 class="text-3xl font-bold mb-6 text-center text-white">Login</h1>

        <form class="space-y-4" @submit.prevent="handleLogin">
          <div>
            <label class="block text-sm font-medium text-gray-300">Username</label>
            <input
              v-model="username"
              type="text"
              required
              aria-label="Username"
              placeholder="Enter your username"
              class="mt-1 w-full px-4 py-2 border border-gray-700 rounded-lg bg-gray-700 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-300">Password</label>
            <input
              v-model="password"
              type="password"
              required
              aria-label="Password"
              placeholder="Enter your password"
              class="mt-1 w-full px-4 py-2 border border-gray-700 rounded-lg bg-gray-700 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
          </div>

          <button
            :disabled="loading"
            aria-label="Submit login form"
            type="submit"
            class="w-full bg-fuchsia-500 hover:bg-fuchsia-600 text-white py-2 rounded-lg transition disabled:opacity-50 flex items-center justify-center space-x-2"
          >
            <svg
              v-if="loading"
              class="animate-spin h-5 w-5 text-white"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
            >
              <circle
                class="opacity-25"
                cx="12"
                cy="12"
                r="10"
                stroke="currentColor"
                stroke-width="4"
              />
              <path
                class="opacity-75"
                fill="currentColor"
                d="M4 12a8 8 0 018-8v4l3-3-3-3v4a8 8 0 00-8 8h4z"
              />
            </svg>
            <span>{{ loading ? 'Logging in...' : 'Login' }}</span>
          </button>
        </form>

        <p v-if="error" class="text-red-400 text-center mt-4">
          {{ errorMessages[error] }}
        </p>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'

const username = ref('')
const password = ref('')
type LoginError = 'INVALID_CREDENTIALS' | 'NETWORK_ERROR' | ''
const error = ref<LoginError>('')
const showLogin = ref(false)
const loading = ref(false)
const router = useRouter()

onMounted(() => {
  const isLoggedIn = localStorage.getItem('isLoggedIn')
  if (isLoggedIn === 'true') {
    router.push('/')
  } else {
    showLogin.value = true
  }
})

function validateCredentials(username: string, password: string): boolean {
  return username === 'admin' && password === 'password'
}

const handleLogin = async () => {
  error.value = ''
  loading.value = true

  // await new Promise((resolve) => setTimeout(resolve, 2000))

  try {
    if (validateCredentials(username.value, password.value)) {
      localStorage.setItem('isLoggedIn', 'true')
      await router.push('/')
    } else {
      error.value = 'INVALID_CREDENTIALS'
    }
  } finally {
    loading.value = false
  }
}

const errorMessages = {
  INVALID_CREDENTIALS: 'Invalid username or password.',
  NETWORK_ERROR: 'Network error. Please try again.',
}
</script>