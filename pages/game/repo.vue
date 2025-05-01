<template>
  <div class="min-h-screen bg-gray-900 text-white p-8 flex flex-col pb-32 overflow-y-clip overflow-x-hidden animate-fade-in">
    <!-- Back Button -->
    <button
      class="absolute top-4 left-4 w-10 h-10 rounded-full bg-gray-700 hover:bg-gray-600 text-white flex items-center justify-center shadow transition-transform hover:scale-110"
      @click="goBack"
    >
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
        <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 19.5 3 12m0 0 7.5-7.5M3 12h18" />
      </svg>
    </button>
    <!-- Download Modal -->
    <div v-if="downloading" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 animate-fade-in">
      <div class="bg-gray-900 p-6 rounded-2xl shadow-2xl w-96 text-center animate-scale-in">
        <h2 class="text-xl font-bold mb-4">Downloading Mods...</h2>

        <div class="w-full bg-gray-700 rounded-full h-4 mb-4 overflow-hidden">
          <div
            class="bg-fuchsia-500 h-full transition-all duration-300 ease-in-out"
            :style="{ width: downloadProgress + '%' }"
          />
        </div>

        <p class="text-sm text-gray-300">{{ downloadProgress }}%</p>
      </div>
    </div>
    <!-- Alert -->
    <div v-if="alertMessage" class="fixed top-4 right-4 bg-fuchsia-700 text-white py-2 px-4 rounded-lg shadow-lg z-50 animate-fade-in">
      {{ alertMessage }}
    </div>
    <!-- Title -->
    <div class="text-center text-3xl font-bold mb-8 animate-fade-slide-down">
      R.E.P.O
    </div>  
    <!-- Search Bar -->
    <div class="mb-6 w-full flex justify-center animate-fade-in">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Search mods or tags..."
        class="w-full max-w-md px-4 py-2 rounded-lg border-2 border-fuchsia-400 bg-gray-800 text-white focus:outline-none focus:ring-2 focus:ring-fuchsia-500 transition duration-300 transform focus:scale-105"
      >
    </div>

    <div v-if="modsLoading" class="text-center text-gray-400 py-4">
      Loading mods...
    </div>
    <div v-else class="flex flex-1 gap-6 animate-fade-in">
      <!-- Mods Grid -->
      <div class="grid grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-6 w-full">
        <button
          v-for="mod in filteredMods"
          :key="mod.name"
          class="max-h-24 relative border-2 px-6 py-4 rounded-xl text-white transition-all duration-300 transform hover:scale-105"
          :class="{
            'border-fuchsia-600': !selectedMods.includes(mod.name),
            'border-fuchsia-300 bg-fuchsia-700 scale-105': selectedMods.includes(mod.name),
          }"
          @click="toggleMod(mod.name)"
        >
          <div class="flex flex-col items-start">
            <span>{{ mod.name }}</span>
            <div class="flex gap-2 mt-2 flex-wrap min-h-[24px]">
              <template v-if="computedTags(mod.name).length > 0">
                <span
                  v-for="tag in computedTags(mod.name)"
                  :key="tag"
                  class="text-xs bg-fuchsia-600 px-2 py-0.5 rounded-full animate-fade-in"
                  :class="{
                    'bg-fuchsia-600': tag !== 'Dependency',
                    'text-xs bg-fuchsia-900 text-white px-2 py-1 rounded-md border border-fuchsia-500': (tag === 'Dependency' || tag === 'Required' || tag === 'Lib'),
                    'text-xs bg-red-900 text-white px-2 py-1 rounded-md border border-red-500': tag === 'Heavy',
                  }"
                >
                  {{ tag }}
                </span>
              </template>
              <template v-else>
                <span class="invisible text-xs bg-fuchsia-600 px-2 py-0.5 rounded-full">
                  Placeholder
                </span>
              </template>
            </div>
          </div>
        </button>
      </div>
      <!-- Download Section -->
      <div class="flex flex-col justify-start items-center w-55 animate-fade-in">
        <button
          class="min-w-52 bg-fuchsia-600 hover:bg-fuchsia-700 text-white font-bold py-2 px-6 rounded-lg transition duration-300 transform hover:scale-110"
          :disabled="downloading"
          @click="downloadSelectedMods"
        >
          {{ downloading ? 'Downloading...' : 'Download' }}
        </button>
        <!-- Import/Export Buttons -->
        <button
          class="min-w-52 my-4 px-6 py-2 border-2 border-fuchsia-400 rounded-lg hover:bg-fuchsia-400 hover:text-black transition transform hover:scale-105"
          @click="exportSelectedMods"
        >
          Export Selected
        </button>
        <button
          class="min-w-52 px-6 py-2 border-2 border-fuchsia-400 rounded-lg hover:bg-fuchsia-400 hover:text-black transition transform hover:scale-105"
          @click="importSelectedMods"
        >
          Import Selected
        </button>
        <div class="max-w-52 bg-gray-800 p-4 mt-8 rounded-2xl text-white text-center shadow-md w-64 max-h-48 overflow-y-auto animate-fade-in">
          <div class="text-lg font-bold mb-2 text-fuchsia-400">
            Selected Mods: {{ selectedMods.length }}
          </div>
          <div class="text-sm break-words">
            {{ selectedMods.length > 0 ? selectedMods.join(', ') : 'None' }}
          </div>
        </div>
      </div>
    </div>
    <!-- Directory Selector -->
    <div class="fixed bottom-0 left-0 w-full bg-gray-800 py-4 flex items-center justify-center gap-4 animate-fade-up">
      <button
        class="px-6 py-2 border-2 border-fuchsia-400 rounded-lg hover:bg-fuchsia-400 hover:text-black transition-all transform hover:scale-105"
        @click="selectDirectory"
      >
        Select Folder
      </button>
      <div class="text-lg font-semibold max-w-80 truncate">{{ selectedDirectory }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { save, open } from '@tauri-apps/plugin-dialog';
import { load } from '@tauri-apps/plugin-store';
import { download } from '@tauri-apps/plugin-upload';
import { readTextFile, writeTextFile, exists, mkdir } from '@tauri-apps/plugin-fs';
import { invoke } from '@tauri-apps/api/core';

interface Mod {
  name: string
  url: string
  tags?: string[]
  dependencies?: string[]
}

const mods = ref<Mod[]>([]);
const modsLoading = ref(false);

const selectedDirectory = ref('Loading...')
const selectedMods = ref<string[]>([])
const downloading = ref(false);
const downloadProgress = ref(0);
const searchQuery = ref('');

const filteredMods = computed(() => {
  if (!searchQuery.value.trim()) {
    return mods.value;
  }
  const query = searchQuery.value.toLowerCase();
  return mods.value.filter(mod => {
    const nameMatch = mod.name.toLowerCase().includes(query);
    const tagMatch = (mod.tags || []).some(tag => tag.toLowerCase().includes(query));
    return nameMatch || tagMatch;
  });
});

const store = await load('.config.json', { autoSave: false });

async function selectDirectory() {
  const selected = await open({
    directory: true,
    multiple: false,
  });

  if (typeof selected === 'string') {
    selectedDirectory.value = selected;
    await store.set('repo', { dir: selected });
    await store.save();
  }
}

onMounted(async () => {
  const savedDir = await store.get<{ dir: string }>('repo');
  selectedDirectory.value = savedDir?.dir || 'Please select a directory';

  // Fetch mods dynamically
  modsLoading.value = true;
  await new Promise((resolve) => setTimeout(resolve, 800))
  try {
    const response = await fetch('https://yuerei.github.io/api/game/repo.json');
    if (!response.ok) throw new Error('Failed to fetch mods');
    const data = await response.json();

    mods.value = data;
    for (const mod of mods.value) {
      if (mod.tags?.includes('Required')) {
        if (!selectedMods.value.includes(mod.name)) {
          selectedMods.value.push(mod.name);
        }
      }
    }
  } catch (error) {
    console.error('Error loading mods:', error);
    showAlert('Failed to load mods!');
  } finally {
    modsLoading.value = false;
  }
});

function toggleMod(modName: string) {
  const mod = mods.value.find(m => m.name === modName);
  if (!mod) return;

  const isSelected = selectedMods.value.includes(modName);

  if (isSelected) {
    if (mod.tags?.includes('Required') || computedTags(modName).includes('Dependency')) {
      return;
    }
    selectedMods.value = selectedMods.value.filter(m => m !== modName);

    for (const dep of mod.dependencies || []) {
      const stillNeeded = mods.value.some(m =>
        selectedMods.value.includes(m.name) && m.dependencies?.includes(dep)
      );
      if (!stillNeeded) {
        toggleMod(dep);
      }
    }
  } else {
    selectModWithDependencies(modName);
  }
}

function selectModWithDependencies(modName: string) {
  if (selectedMods.value.includes(modName)) return;

  selectedMods.value.push(modName);

  const mod = mods.value.find(m => m.name === modName);
  if (!mod) return;

  for (const dep of mod.dependencies || []) {
    selectModWithDependencies(dep);
  }
}

function computedTags(modName: string): string[] {
  const mod = mods.value.find(m => m.name === modName);
  if (!mod) return [];

  const tags = [...(mod.tags || [])];
  // Check if this mod is needed because of dependency
  const isDependency = selectedMods.value.some(selectedModName => {
    const selectedMod = mods.value.find(m => m.name === selectedModName);
    return selectedMod?.dependencies?.includes(modName);
  });

  if (isDependency && !tags.includes('Dependency')) {
    tags.push('Dependency');
  }
  return tags;
}

async function downloadSelectedMods() {
  if (selectedDirectory.value === 'Loading...' || selectedDirectory.value === 'Please select a directory') {
    selectedDirectory.value = 'Please select a directory'
    showAlert('No directory selected!');
    return;
  }
  if (selectedMods.value.length === 0) {
    showAlert('Please select at least one mod!')
    return
  }

  downloading.value = true;
  downloadProgress.value = 0;

  try {
    const totalMods = selectedMods.value.length;
    let completedMods = 0;

    for (const modName of selectedMods.value) {
      const mod = mods.value.find(m => m.name === modName)
      if (!mod) continue

      const fileDir = `${selectedDirectory.value}\\_temp`
      const targetDir = `${selectedDirectory.value}\\_temp_target`
      if (!(await exists(fileDir))) await mkdir(fileDir)
      if (!(await exists(targetDir))) await mkdir(targetDir)
      
      const tempFilePath = `${selectedDirectory.value}/_temp/${mod.name}.zip`
      console.log('Downloading', mod.url, 'to', tempFilePath)

      await download(mod.url, tempFilePath, ({ progress, total }) => {
        if (total > 0) {
          const realProgress = Math.floor((progress / total) * 100); // <--- Use Math.floor
          if (realProgress > downloadProgress.value) {
            downloadProgress.value = realProgress;
          }
        }
      });

      const tempExtractFolder = `${targetDir}/${mod.name}`;
      let targetExtractFolder;
      if (modName === 'BepInEx' || modName==='Faster Load') targetExtractFolder = `${selectedDirectory.value}`;
      else targetExtractFolder = `${selectedDirectory.value}/BepInEx/plugins/`;

      await extract(tempFilePath, tempExtractFolder, targetExtractFolder);
      console.log(`Extracted ${mod.name} to ${targetExtractFolder}`);

      completedMods++;
      downloadProgress.value = Math.floor((completedMods / totalMods) * 100);
    }
    downloadProgress.value = 100;
    await new Promise(resolve => setTimeout(resolve, 500));

    showAlert('Download complete!')
  } catch (error) {
    console.error(error)
    showAlert('Failed to download mods!');
  } finally {
    downloading.value = false;
    downloadProgress.value = 0;
  }
}

async function extract(zipPath: string, tempPath: string, outputPath: string) {
  try {
    await invoke('unzip_file', { zipPath, tempPath, outputPath })
    console.log('Extraction complete!')
  } catch (error) {
    console.error('Extraction error:', error)
  }
}

async function exportSelectedMods() {
  if (selectedMods.value.length === 0) {
    showAlert('No mods selected to export!');
    return;
  }

  const filePath = await save({
    filters: [{ name: 'JSON', extensions: ['json'] }],
    defaultPath: 'selected_mods.json'
  });

  if (!filePath) return;

  const data = JSON.stringify(selectedMods.value, null, 2);
  await writeTextFile(filePath, data);
  showAlert('Mods exported successfully!');
}

async function importSelectedMods() {
  const selected = await open({
    filters: [{ name: 'JSON', extensions: ['json'] }],
    multiple: false
  });

  if (!selected || typeof selected !== 'string') {
    showAlert('No files selected!');
    return;
  }

  const fileContent = await readTextFile(selected);
  try {
    const importedMods = JSON.parse(fileContent);
    if (Array.isArray(importedMods)) {
      selectedMods.value = importedMods.filter((modName: string) => 
        mods.value.some(m => m.name === modName)
      );
      showAlert('Mods imported successfully!');
    } else {
      showAlert('Invalid file format!');
    }
  } catch (e) {
    showAlert('Failed to import: Invalid JSON.');
    throw new Error(`${e}`);
  }
}

const alertMessage = ref<string | null>(null);
function showAlert(message: string) {
  alertMessage.value = message;
  setTimeout(() => alertMessage.value = '', 3000);
}
const goBack = () => {
  window.history.back();
}
</script>