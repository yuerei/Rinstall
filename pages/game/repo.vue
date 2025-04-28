<template>
  <div class="min-h-screen bg-gray-900 text-white p-8 flex flex-col">

    <!-- Top Title -->
    <div class="text-center text-3xl font-bold mb-8">
      R.E.P.O
    </div>

    <div class="flex flex-1 gap-12">

      <!-- Mods Grid -->
      <div class="grid grid-cols-2 gap-6 w-full">
        <button
          v-for="mod in mods"
          :key="mod.name"
          @click="toggleMod(mod.name)"
          class="border-2 px-6 py-4 rounded-xl text-white transition duration-300"
          :class="{
            'border-fuchsia-500': !selectedMods.includes(mod.name),
            'border-fuchsia-300 bg-fuchsia-700': selectedMods.includes(mod.name),
          }"
        >
        {{ mod.name }}
        </button>
      </div>


      <!-- Download Section -->
      <div class="flex flex-col justify-start items-center w-55">
        <button
          class="bg-fuchsia-600 hover:bg-fuchsia-700 text-white font-bold py-2 px-6 rounded-lg transition duration-300"
          :disabled="downloading"
          @click="downloadSelectedMods"
        >
          {{ downloading ? 'Downloading...' : 'Download Selected Mods' }}
        </button>
        <div class="text-white mt-8 text-xl">
          Selected Mods: {{ selectedMods.length > 0 ? selectedMods.join(', ') : 'None' }}
        </div>
      </div>
    </div>

    <!-- Bottom Select Directory -->
    <div class="flex items-center gap-4 mt-12">
      <button
        @click="selectDirectory"
        class="px-6 py-2 border-2 border-fuchsia-400 rounded-lg hover:bg-fuchsia-400 hover:text-black transition"
      >
        Select Dir
      </button>
      <div class="text-lg font-semibold">{{ selectedDirectory }}</div>
    </div>

  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { open } from '@tauri-apps/plugin-dialog';
import { load } from '@tauri-apps/plugin-store';
import { download } from '@tauri-apps/plugin-upload';
import { exists, mkdir } from '@tauri-apps/plugin-fs';

const selectedDirectory = ref('Loading...')
const selectedMods = ref<string[]>([])
const downloading = ref(false)

const mods = [
  { name: 'Mod 1', url: 'https://drive.google.com/uc?export=download&id=1tDHVSSMJ0fvU1aQw_O4edPpNDQ7sGtmf' },
  { name: 'Mod 2', url: 'https://cdn.modrinth.com/data/AANobbMI/versions/DA250htH/sodium-fabric-0.6.13%2Bmc1.21.5.jar' },
  { name: 'Mod 3', url: 'https://example.com/mod3.zip' },
  { name: 'Mod 4', url: 'https://example.com/mod4.zip' },
  { name: 'Mod 5', url: 'https://example.com/mod5.zip' },
  { name: 'Mod 6', url: 'https://example.com/mod6.zip' },
]

const store = await load('.config.json', { autoSave: false });
async function selectDirectory() {
  const selected = await open({
    directory: true,
    multiple: false,
  })

  if (typeof selected === 'string') {
    selectedDirectory.value = selected
    await store.set('repo', {dir: selected})
    await store.save()
  }
}
onMounted(async () => {
  const savedDir = await store.get<{dir: string}>('repo')
  selectedDirectory.value = savedDir?.dir || 'Please select a directory'
})

function toggleMod(modName: string) {
  if (selectedMods.value.includes(modName)) {
    selectedMods.value = selectedMods.value.filter(m => m !== modName)
  } else {
    selectedMods.value.push(modName)
  }
}

async function downloadSelectedMods() {
  if (selectedMods.value.length === 0) {
    alert('Please select at least one mod!')
    return
  }

  downloading.value = true

  try {
    for (const modName of selectedMods.value) {
      const mod = mods.find(m => m.name === modName)
      if (!mod) continue

      const fileDir = `${selectedDirectory.value}\\_temp`
      if (!(await exists(fileDir))) {
        await mkdir(fileDir)
      }
      const filePath = `${selectedDirectory.value}/_temp/${mod.name}.zip`
      console.log('Downloading', mod.url, 'to', filePath)

      await download(mod.url,
        filePath, ({ progress, total }) =>
        console.log(`Downloaded ${progress} of ${total} bytes`)
      )
    }
    alert('Download complete!')
  } catch (error) {
    console.error(error)
    alert('Failed to download mods!')
  } finally {
    downloading.value = false
  }
}
</script>
