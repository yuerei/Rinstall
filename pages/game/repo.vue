<template>
  <div class="min-h-screen bg-gray-900 text-white p-8 flex flex-col pb-32 overflow-x-hidden animate-fade-in">
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

    <div class="flex flex-1 gap-6 animate-fade-in">
      <!-- Mods Grid -->
      <div class="grid grid-cols-2 gap-6 w-full">
        <button
          v-for="mod in filteredMods"
          :key="mod.name"
          class="relative border-2 px-6 py-4 rounded-xl text-white transition duration-300 transform hover:scale-105"
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
        class="px-6 py-2 border-2 border-fuchsia-400 rounded-lg hover:bg-fuchsia-400 hover:text-black transition transform hover:scale-105"
        @click="selectDirectory"
      >
        Select
      </button>
      <div class="text-lg font-semibold max-w-80 truncate">{{ selectedDirectory }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { save, open } from '@tauri-apps/plugin-dialog';
import { load } from '@tauri-apps/plugin-store';
import { download } from '@tauri-apps/plugin-upload';
import { readTextFile, writeTextFile, exists, mkdir } from '@tauri-apps/plugin-fs';
import { invoke } from '@tauri-apps/api/core';

const selectedDirectory = ref('Loading...')
const selectedMods = ref<string[]>([])
const downloading = ref(false);
const downloadProgress = ref(0);
const searchQuery = ref('');

const filteredMods = computed(() => {
  if (!searchQuery.value.trim()) {
    return mods;
  }
  const query = searchQuery.value.toLowerCase();
  return mods.filter(mod => {
    const nameMatch = mod.name.toLowerCase().includes(query);
    const tagMatch = (mod.tags || []).some(tag => tag.toLowerCase().includes(query));
    return nameMatch || tagMatch;
  });
});

interface Mod {
  name: string
  url: string
  tags?: string[]
  dependencies?: string[]
}
const mods: Mod[] = [
  { name: 'BepInEx', url: 'https://drive.google.com/uc?export=download&id=1tDHVSSMJ0fvU1aQw_O4edPpNDQ7sGtmf', tags: ['Required'], dependencies: [] },
  { name: 'REPOLib', url: 'https://thunderstore.io/package/download/Zehs/REPOLib/2.0.1/', tags: ['Lib'], dependencies: ['BepInEx'] },
  { name: 'MenuLib', url: 'https://thunderstore.io/package/download/nickklmao/MenuLib/2.4.1/', tags: ['Lib'], dependencies: ['BepInEx'] },
  { name: 'REPOConfig', url: 'https://thunderstore.io/package/download/nickklmao/REPOConfig/1.2.2/', tags: ['Lib'], dependencies: ['BepInEx', "MenuLib"] },
  { name: 'MoreHead', url: 'https://thunderstore.io/package/download/YMC_MHZ/MoreHead/1.3.11/', tags: ['Fun'], dependencies: ['MenuLib'] },
  { name: 'MoreHeadUtilities', url: 'https://thunderstore.io/package/download/Maygik/MoreHeadUtilities/1.0.6/', tags: ['Lib'], dependencies: ['MoreHead'] },
  { name: 'MoreHeadPlus', url: 'https://thunderstore.io/package/download/RESET/MoreHeadPlus/1.9.6/', tags: ['Fun'], dependencies: ['MoreHead', "MoreHeadUtilities"] },
  { name: 'MoreShopItems', url: 'https://thunderstore.io/package/download/GalaxyMods/MoreShopItems/1.3.3/', tags: ['QOL', 'Upgrades'], dependencies: ['REPOLib'] },
  { name: 'Mimic', url: 'https://thunderstore.io/package/download/eth9n/Mimic/1.1.6/', tags: ['Enemies'], dependencies: ['REPOLib', 'REPOConfig'] },
  { name: 'MoreUpgrades', url: 'https://thunderstore.io/package/download/BULLETBOT/MoreUpgrades/1.4.8/', tags: ['QOL', 'Upgrades'], dependencies: ['REPOLib'] },
  { name: 'LateJoin', url: 'https://thunderstore.io/package/download/Rebateman/LateJoin/0.1.2/', tags: ['QOL'], dependencies: [] },
  { name: 'MorePlayers', url: 'https://thunderstore.io/package/download/zelofi/MorePlayers/1.0.4/', tags: ['QOL'], dependencies: [] },
  { name: 'DeadTTS', url: 'https://thunderstore.io/package/download/flipf17/DeadTTS/1.0.10/', tags: ['QOL'], dependencies: [] },
  { name: 'ExtractionPointConfirm', url: 'https://thunderstore.io/package/download/Zehs/ExtractionPointConfirmButton/1.1.0/', tags: ['QOL'], dependencies: ['REPOLib'] },
  { name: 'Wesleys Enemies', url: 'https://thunderstore.io/package/download/Magic_Wesley/Wesleys_Enemies/1.2.2/', tags: ['Enemies'], dependencies: ['REPOLib'] },
  { name: 'CustomColors', url: 'https://thunderstore.io/package/download/x753_REPO/CustomColors/1.1.1/', tags: ['Fun'], dependencies: [] },
  { name: 'Unique Potions', url: 'https://thunderstore.io/package/download/Yuckers/Unique_Potions/0.6.3/', tags: ['Items'], dependencies: ['REPOLib', 'REPO Alchemy'] },
  { name: 'REPO Alchemy', url: 'https://thunderstore.io/package/download/Lunoid/REPO_Alchemy/1.1.0/', tags: ['Lib'], dependencies: ['REPOLib', 'CustomDiscoverStateLib'] },
  { name: 'CustomDiscoverStateLib', url: 'https://thunderstore.io/package/download/Kistras/CustomDiscoverStateLib/1.0.1/', tags: ['Lib'], dependencies: [] },
  { name: 'REPORoles', url: 'https://thunderstore.io/package/download/BobisMods/REPORoles/2.0.3/', tags: ['Fun'], dependencies: ['REPOConfig', 'REPOLib', 'MenuLib'] },
  { name: 'Faster Load', url: 'https://thunderstore.io/package/download/DiFFoZ/BepInEx_Faster_Load_AssetBundles_Patcher/1.0.1/', tags: ['QOL'], dependencies: [] },
  { name: 'MinecraftStrongholdLevel', url: 'https://thunderstore.io/package/download/AriIcedT/MinecraftStrongholdLevel/1.6.1/', tags: ['Levels'], dependencies: ['REPOLib'] },
  { name: 'MoreReviveHP', url: 'https://thunderstore.io/package/download/Tidaleus/MoreReviveHP/1.0.1/', tags: ['QOL'], dependencies: [] },
  { name: 'Wesleys Valuables', url: 'https://thunderstore.io/package/download/Magic_Wesley/Wesleys_Valuables/1.0.3/', tags: ['Items'], dependencies: ['REPOLib'] },
  { name: 'DamageShow', url: 'https://thunderstore.io/package/download/XiaohaiMod/XH_DamageShow_EnemyHealthBar/1.0.2/', tags: ['Cheats?'], dependencies: [] },
  { name: 'FovUpdate', url: 'https://thunderstore.io/package/download/darmuh/FovUpdate/0.2.9/', tags: ['Client', 'QOL'], dependencies: [] },
  { name: 'CustomGrabColor', url: 'https://thunderstore.io/package/download/Enchanted_Games/CustomGrabColor/2.1.0/', tags: ['QOL'], dependencies: ['MenuLib'] },
  { name: 'UltimateRevive', url: 'https://thunderstore.io/package/download/Godji/UltimateRevive/1.0.1/', tags: ['QOL', 'Cheats?'], dependencies: [] },
  { name: 'PostLevelSummary', url: 'https://thunderstore.io/package/download/Hattorius/PostLevelSummary/2.0.0/', tags: ['QOL', 'UI'], dependencies: [] },
  { name: 'MapVote', url: 'https://thunderstore.io/package/download/Patrick/MapVote/1.1.0/', tags: ['Cheats'], dependencies: ['REPOLib', 'MenuLib'] },
  { name: 'Hospital Level', url: 'https://thunderstore.io/package/download/Rangerbb275/Hospital_Level/2.0.0/', tags: ['Levels'], dependencies: ['REPOLib'] },
  { name: 'ItemResistUpgrade', url: 'https://thunderstore.io/package/download/TopSandwich/ItemResistUpgrade/1.0.4/', tags: ['Upgrades'], dependencies: ['REPOLib', 'MoreUpgrades'] },
  { name: 'CartSpeedSync', url: 'https://thunderstore.io/package/download/discjenny/CartSpeedSync/2.0.0/', tags: ['QOL'], dependencies: [] },
  { name: 'DynamicLighting', url: 'https://thunderstore.io/package/download/DirtyGames/DynamicLighting/1.1.0/', tags: ['QOL', 'Heavy'], dependencies: [] },
  { name: 'No Save Delete', url: 'https://thunderstore.io/package/download/PxntxrezStudio/No_Save_Delete/1.2.4/', tags: ['Cheats'], dependencies: [] },
  { name: 'Dead Map Access', url: 'https://thunderstore.io/package/download/SaturnKai/Dead_Map_Access/1.0.4/', tags: ['UI'], dependencies: [] },
  { name: 'StinkyPaintingReplacement', url: 'https://thunderstore.io/package/download/LandoZor/StinkyPaintingReplacement/1.1.0/', tags: ['Client', 'Fun'], dependencies: [] },
  { name: 'ChatClipboard', url: 'https://thunderstore.io/package/download/ManancialGD/ChatClipboard/1.0.0/', tags: ['Client', 'QOL'], dependencies: [] },
  { name: 'ShoppingListHUD', url: 'https://thunderstore.io/package/download/khalliv/ShoppingListHUD/1.0.2/', tags: ['UI'], dependencies: [] },
  { name: 'GambleInShop', url: 'https://thunderstore.io/package/download/khalliv/GambleInShop/1.0.0/', tags: ['Fun'], dependencies: ['REPOLib'] },
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

const goBack = () => {
  window.history.back();
}

function toggleMod(modName: string) {
  const mod = mods.find(m => m.name === modName);
  if (!mod) return;

  const isSelected = selectedMods.value.includes(modName);

  if (isSelected) {
    if (mod.tags?.includes('Required') || computedTags(modName).includes('Dependency')) {
      return;
    }
    selectedMods.value = selectedMods.value.filter(m => m !== modName);

    // Also remove dependencies if they are not needed anymore
    for (const dep of mod.dependencies || []) {
      const stillNeeded = mods.some(m =>
        selectedMods.value.includes(m.name) && m.dependencies?.includes(dep)
      );
      if (!stillNeeded) {
        toggleMod(dep); // Use toggleMod to properly remove recursively
      }
    }
  } else {
    selectModWithDependencies(modName);
  }
}

function selectModWithDependencies(modName: string) {
  if (selectedMods.value.includes(modName)) return;

  selectedMods.value.push(modName);

  const mod = mods.find(m => m.name === modName);
  if (!mod) return;

  for (const dep of mod.dependencies || []) {
    selectModWithDependencies(dep); // Recursively select dependencies
  }
}

function computedTags(modName: string): string[] {
  const mod = mods.find(m => m.name === modName);
  if (!mod) return [];

  const tags = [...(mod.tags || [])];

  // Check if this mod is needed because of dependency
  const isDependency = selectedMods.value.some(selectedModName => {
    const selectedMod = mods.find(m => m.name === selectedModName);
    return selectedMod?.dependencies?.includes(modName);
  });

  if (isDependency && !tags.includes('Dependency')) {
    tags.push('Dependency');
  }

  return tags;
}

async function downloadSelectedMods() {
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
      const mod = mods.find(m => m.name === modName)
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

  if (!selected || typeof selected !== 'string') return;

  const fileContent = await readTextFile(selected);
  try {
    const importedMods = JSON.parse(fileContent);
    if (Array.isArray(importedMods)) {
      selectedMods.value = importedMods.filter((modName: string) => 
        mods.some(m => m.name === modName)
      );
      showAlert('Mods imported successfully!');
    } else {
      showAlert('Invalid file format!');
    }
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  } catch (error) {
    showAlert('Failed to import: Invalid JSON.');
  }
}

const alertMessage = ref<string | null>(null);
function showAlert(message: string) {
  alertMessage.value = message;
  setTimeout(() => alertMessage.value = '', 3000);
}

// Auto-select Required mods
for (const mod of mods) {
  if (mod.tags?.includes('Required')) {
    if (!selectedMods.value.includes(mod.name)) {
      selectedMods.value.push(mod.name);
    }
  }
}
</script>

<style scoped>
/* Custom Scrollbar */
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  background: #1f2937; /* dark background (gray-800) */
  border-radius: 10px;
}

::-webkit-scrollbar-thumb {
  background-color: #d946ef; /* fuchsia-400 */
  border-radius: 10px;
  border: 2px solid #1f2937; /* gives padding effect */
}

::-webkit-scrollbar-thumb:hover {
  background: #c026d3; /* fuchsia-600 on hover */
}

@keyframes fade-in {
    0% {
        opacity: 0;
        transform: scale(0.95);
    }

    100% {
        opacity: 1;
        transform: scale(1);
    }
}

@keyframes fade-slide-down {
    0% {
        opacity: 0;
        transform: translateY(-10px);
    }

    100% {
        opacity: 1;
        transform: translateY(0);
    }
}

@keyframes fade-up {
    0% {
        opacity: 0;
        transform: translateY(10px);
    }

    100% {
        opacity: 1;
        transform: translateY(0);
    }
}

@keyframes scale-in {
    0% {
        transform: scale(0.8);
        opacity: 0;
    }

    100% {
        transform: scale(1);
        opacity: 1;
    }
}

.animate-fade-in {
    animation: fade-in 0.5s ease-out;
}

.animate-fade-slide-down {
    animation: fade-slide-down 0.5s ease-out;
}

.animate-fade-up {
    animation: fade-up 0.5s ease-out;
}

.animate-scale-in {
    animation: scale-in 0.4s ease-out;
}
</style>
