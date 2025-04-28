<template>
  <div class="min-h-screen bg-gray-900 text-white p-8 flex flex-col pb-32 overflow-x-hidden">
    <!-- Download Progress Modal -->
    <div v-if="downloading" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-gray-900 p-6 rounded-2xl shadow-2xl w-96 text-center">
        <h2 class="text-xl font-bold mb-4">Downloading Mods...</h2>
        
        <div class="w-full bg-gray-700 rounded-full h-4 mb-4 overflow-hidden">
          <div
            class="bg-fuchsia-500 h-full transition-all duration-300"
            :style="{ width: downloadProgress + '%' }"
          />
        </div>

        <p class="text-sm text-gray-300">{{ downloadProgress }}%</p>
      </div>
    </div>

    <div v-if="alertMessage" class="fixed top-4 right-4 bg-fuchsia-700 text-white py-2 px-4 rounded-lg shadow-lg z-50 animate-fade-in">
      {{ alertMessage }}
    </div>
    <!-- Top Title -->
    <div class="text-center text-3xl font-bold mb-8">
      R.E.P.O
    </div>
    <!-- Search Bar -->
    <div class="mb-6 w-full flex justify-center">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Search mods or tags..."
        class="w-full max-w-md px-4 py-2 rounded-lg border-2 border-fuchsia-400 bg-gray-800 text-white focus:outline-none focus:ring-2 focus:ring-fuchsia-500"
      >
    </div>

    <div class="flex flex-1 gap-12">
      <!-- Mods Grid -->
      <div class="grid grid-cols-2 gap-6 w-full">
        <button
        v-for="mod in filteredMods"
          :key="mod.name"
          class="relative border-2 px-6 py-4 rounded-xl text-white transition duration-300"
          :class="{
            'border-fuchsia-500': !selectedMods.includes(mod.name),
            'border-fuchsia-300 bg-fuchsia-700': selectedMods.includes(mod.name),
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
                class="text-xs bg-fuchsia-600 px-2 py-0.5 rounded-full"
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
      <div class="flex flex-col justify-start items-center w-55">
        <button
          class="bg-fuchsia-600 hover:bg-fuchsia-700 text-white font-bold py-2 px-6 rounded-lg transition duration-300"
          :disabled="downloading"
          @click="downloadSelectedMods"
        >
          {{ downloading ? 'Downloading...' : 'Download Selected Mods' }}
        </button>
        <div class="text-white mt-8 text-xl max-w-40">
          Selected Mods: {{ selectedMods.length > 0 ? selectedMods.join(', ') : 'None' }}
        </div>
      </div>
    </div>

    <!-- Select Directory pinned at bottom -->
    <div class="fixed bottom-0 left-0 w-full bg-gray-800 py-4 flex items-center justify-center gap-4">
      <button
        class="px-6 py-2 border-2 border-fuchsia-400 rounded-lg hover:bg-fuchsia-400 hover:text-black transition"
        @click="selectDirectory"
      >
        Select Directory
      </button>
      <div class="text-lg font-semibold max-w-80 truncate">{{ selectedDirectory }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { open } from '@tauri-apps/plugin-dialog';
import { load } from '@tauri-apps/plugin-store';
import { download } from '@tauri-apps/plugin-upload';
import { exists, mkdir } from '@tauri-apps/plugin-fs';
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
  { name: 'REPOConfig', url: 'https://thunderstore.io/package/download/nickklmao/REPOConfig/1.2.2/', tags: [], dependencies: ['BepInEx', "MenuLib"] },

  { name: 'MoreHead', url: 'https://thunderstore.io/package/download/YMC_MHZ/MoreHead/1.3.11/', tags: [], dependencies: ['MenuLib'] },
  { name: 'MoreHeadUtilities', url: 'https://thunderstore.io/package/download/Maygik/MoreHeadUtilities/1.0.6/', tags: [], dependencies: ['MoreHead'] },
  { name: 'MoreHeadPlus', url: 'https://thunderstore.io/package/download/RESET/MoreHeadPlus/1.9.6/', tags: [], dependencies: ['MoreHead', "MoreHeadUtilities"] },
  { name: 'MoreShopItems', url: 'https://thunderstore.io/package/download/GalaxyMods/MoreShopItems/1.3.3/', tags: [], dependencies: ['REPOLib'] },
  { name: 'Mimic', url: 'https://thunderstore.io/package/download/eth9n/Mimic/1.1.6/', tags: [], dependencies: ['REPOLib', 'REPOConfig'] },
  { name: 'MoreUpgrades', url: 'https://thunderstore.io/package/download/BULLETBOT/MoreUpgrades/1.4.8/', tags: [], dependencies: ['REPOLib'] },
  { name: 'LateJoin', url: 'https://thunderstore.io/package/download/Rebateman/LateJoin/0.1.2/', tags: [], dependencies: [] },
  { name: 'MorePlayers', url: 'https://thunderstore.io/package/download/zelofi/MorePlayers/1.0.4/', tags: [], dependencies: [] },
  { name: 'DeadTTS', url: 'https://thunderstore.io/package/download/flipf17/DeadTTS/1.0.10/', tags: [], dependencies: [] },
  { name: 'ExtractionPointConfirmButton', url: 'https://thunderstore.io/package/download/Zehs/ExtractionPointConfirmButton/1.1.0/', tags: [], dependencies: ['REPOLib'] },
  { name: 'Wesleys Enemies', url: 'https://thunderstore.io/package/download/Magic_Wesley/Wesleys_Enemies/1.2.2/', tags: [], dependencies: ['REPOLib'] },
  { name: 'CustomColors', url: 'https://thunderstore.io/package/download/x753_REPO/CustomColors/1.1.1/', tags: [], dependencies: [] },
  { name: 'Unique Potions', url: 'https://thunderstore.io/package/download/Yuckers/Unique_Potions/0.6.3/', tags: [], dependencies: ['REPOLib', 'REPO Alchemy'] },
  { name: 'REPO Alchemy', url: 'https://thunderstore.io/package/download/Lunoid/REPO_Alchemy/1.1.0/', tags: ['Lib'], dependencies: ['REPOLib', 'CustomDiscoverStateLib'] },
  { name: 'CustomDiscoverStateLib', url: 'https://thunderstore.io/package/download/Kistras/CustomDiscoverStateLib/1.0.1/', tags: ['Lib'], dependencies: [] },
  { name: 'REPORoles', url: 'https://thunderstore.io/package/download/BobisMods/REPORoles/2.0.3/', tags: [], dependencies: ['REPOConfig', 'REPOLib', 'MenuLib'] },
  { name: 'Faster Load', url: 'https://thunderstore.io/package/download/DiFFoZ/BepInEx_Faster_Load_AssetBundles_Patcher/1.0.1/', tags: [], dependencies: [] },
  { name: 'MinecraftStrongholdLevel', url: 'https://thunderstore.io/package/download/AriIcedT/MinecraftStrongholdLevel/1.6.1/', tags: [], dependencies: ['REPOLib'] },
  { name: 'MoreReviveHP', url: 'https://thunderstore.io/package/download/Tidaleus/MoreReviveHP/1.0.1/', tags: [], dependencies: [] },
  { name: 'Wesleys Valuables', url: 'https://thunderstore.io/package/download/Magic_Wesley/Wesleys_Valuables/1.0.3/', tags: [], dependencies: ['REPOLib'] },
  { name: 'DamageShow', url: 'https://thunderstore.io/package/download/XiaohaiMod/XH_DamageShow_EnemyHealthBar/1.0.2/', tags: [], dependencies: [] },
  { name: 'FovUpdate', url: 'https://thunderstore.io/package/download/darmuh/FovUpdate/0.2.9/', tags: [], dependencies: [] },
  { name: 'CustomGrabColor', url: 'https://thunderstore.io/package/download/Enchanted_Games/CustomGrabColor/2.1.0/', tags: [], dependencies: ['MenuLib'] },
  { name: 'UltimateRevive', url: 'https://thunderstore.io/package/download/Godji/UltimateRevive/1.0.1/', tags: [], dependencies: [] },
  { name: 'PostLevelSummary', url: 'https://thunderstore.io/package/download/Hattorius/PostLevelSummary/2.0.0/', tags: [], dependencies: [] },
  { name: 'MapVote', url: 'https://thunderstore.io/package/download/Patrick/MapVote/1.1.0/', tags: [], dependencies: ['REPOLib', 'MenuLib'] },
  { name: 'Hospital Level', url: 'https://thunderstore.io/package/download/Rangerbb275/Hospital_Level/2.0.0/', tags: ['Levels'], dependencies: ['REPOLib'] },
  { name: 'ItemResistUpgrade', url: 'https://thunderstore.io/package/download/TopSandwich/ItemResistUpgrade/1.0.4/', tags: [], dependencies: ['REPOLib', 'MoreUpgrades'] },
  { name: 'CartSpeedSync', url: 'https://thunderstore.io/package/download/discjenny/CartSpeedSync/2.0.0/', tags: [], dependencies: [] },
  { name: 'DynamicLighting', url: 'https://thunderstore.io/package/download/DirtyGames/DynamicLighting/1.1.0/', tags: [], dependencies: [] },
  { name: 'No Save Delete', url: 'https://thunderstore.io/package/download/PxntxrezStudio/No_Save_Delete/1.2.4/', tags: [], dependencies: [] },
  { name: 'Dead Map Access', url: 'https://thunderstore.io/package/download/SaturnKai/Dead_Map_Access/1.0.4/', tags: [], dependencies: [] },
  { name: 'StinkyPaintingReplacement', url: 'https://thunderstore.io/package/download/LandoZor/StinkyPaintingReplacement/1.1.0/', tags: ['Client'], dependencies: [] },
  { name: 'ChatClipboard', url: 'https://thunderstore.io/package/download/ManancialGD/ChatClipboard/1.0.0/', tags: ['Client'], dependencies: [] },
  { name: 'ShoppingListHUD', url: 'https://thunderstore.io/package/download/khalliv/ShoppingListHUD/1.0.2/', tags: [], dependencies: [] },
  { name: 'GambleInShop', url: 'https://thunderstore.io/package/download/khalliv/GambleInShop/1.0.0/', tags: [], dependencies: ['REPOLib'] },
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
  let fakeProgress = 0;
  let intervalId: ReturnType<typeof setInterval> | null = null;

  try {
    // Start fake smooth progress
    intervalId = setInterval(() => {
      if (downloadProgress.value < 90) {
        fakeProgress += Math.random() * 3; // Increase slowly
        downloadProgress.value = Math.min(90, Math.floor(fakeProgress)); // <--- Use Math.floor
      }
    }, 200);
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
      if (modName === 'BepInEx') targetExtractFolder = `${selectedDirectory.value}`;
      else targetExtractFolder = `${selectedDirectory.value}/BepInEx/plugins/`;

      await extract(tempFilePath, tempExtractFolder, targetExtractFolder);
      console.log(`Extracted ${mod.name} to ${targetExtractFolder}`);
    }
    // Finish progress
    downloadProgress.value = 100;
    await new Promise(resolve => setTimeout(resolve, 500)); // short delay to show 100%

    showAlert('Download complete!')
  } catch (error) {
    console.error(error)
    showAlert('Failed to download mods!');
  } finally {
    if (intervalId) clearInterval(intervalId);
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

const alertMessage = ref<string | null>(null);

function showAlert(message: string, duration = 3000) {
  alertMessage.value = message;
  setTimeout(() => {
    alertMessage.value = null;
  }, duration);
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
@keyframes fade-in {
  from { opacity: 0; transform: translateY(-10px); }
  to { opacity: 1; transform: translateY(0); }
}
.animate-fade-in {
  animation: fade-in 0.3s ease-out;
}
</style>
