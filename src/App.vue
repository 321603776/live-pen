<script setup lang="ts">
import UI from "./components/UI.vue";
import { onBeforeUnmount, onMounted, ref, watch } from 'vue';
import SignaturePad from 'signature_pad';
import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import { register, unregister } from '@tauri-apps/api/globalShortcut';
import { config } from './config';
import { window as tauriWindow } from '@tauri-apps/api';
import { useMagicKeys } from '@vueuse/core'
import { LogicalPosition } from '@tauri-apps/api/window';

let signaturePad: SignaturePad | null = null;
const bg = ref("")

// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
const keys = useMagicKeys();
const hotescape = keys.escape;
const hotundo = keys[config.undo]

const canvas = ref<HTMLCanvasElement | null>(null);
onMounted(() => {
  signaturePad = new SignaturePad(canvas.value!);
  window.addEventListener("resize", resizeCanvas);
  resizeCanvas();
})

onBeforeUnmount(() => {
  window.removeEventListener("resize", resizeCanvas);
})

watch(hotescape, () => {
  if (hotescape.value) {
    tauriWindow.getCurrent().hide();
    bg.value = ""
  }
})

watch(hotundo, () => {
  if (hotundo.value) {
    undo();
  }
})

onMounted(() => {
  register(config.shortcut, handleActive)
})

onBeforeUnmount(() => {
  unregister(config.shortcut)
})

async function handleActive() {
  const monitor = await tauriWindow.currentMonitor();
  const currentWindow = tauriWindow.getCurrent();

  if (monitor) tauriWindow.appWindow.setSize(monitor.size);
  currentWindow.setPosition(new LogicalPosition(0, 0));
  // currentWindow.setAlwaysOnTop(true);
  currentWindow.show();

  if (signaturePad) signaturePad.clear()
  bg.value = convertFileSrc((await invoke("update_screenshot", {}) as string)) + "?" + Date.now();
}

function resizeCanvas() {
  if (canvas.value && signaturePad) {
    const ratio = Math.max(window.devicePixelRatio || 1, 1);
    canvas.value.width = canvas.value.offsetWidth * ratio;
    canvas.value.height = canvas.value.offsetHeight * ratio;
    canvas.value.getContext("2d")?.scale(ratio, ratio);
    signaturePad.clear(); // otherwise isEmpty() might return incorrect value
  }
}

async function undo() {
  if (signaturePad) {
    const data = signaturePad.toData();

    if (data && data.length) {
      data.pop();
      signaturePad.fromData(data);
    }
  }
}
</script>

<template>
  <img v-show="bg" :src="bg" style="width: 100%; height: 100%;" />
  <canvas ref="canvas" style="width: 100%; height: 100%; position: absolute; top: 0; left: 0;"></canvas>
  <UI />
</template>

<style scoped></style>
