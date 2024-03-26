<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref, watch } from 'vue';
import SignaturePad from 'signature_pad';
import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import { register } from '@tauri-apps/api/globalShortcut';
import { config } from './config';
import { window as tauriWindow } from '@tauri-apps/api';
import { useMagicKeys } from '@vueuse/core'

let signaturePad: SignaturePad | null = null;
let bg: string = "";

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
  }
})

watch(hotundo, () => {
  if (hotundo.value) {
    undo();
  }
})

register(config.shortcut, async () => {
  tauriWindow.getCurrent().show();
  if (signaturePad) signaturePad.clear()
  bg = convertFileSrc((await invoke("update_screenshot", {}) as string)) + "?" + Date.now();
  if (signaturePad) signaturePad.fromDataURL(bg)
})

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

    if (data) {
      data.pop();
      await signaturePad.fromDataURL(bg);
      signaturePad.fromData(data);
    }
  }
}
</script>

<template>
  <canvas ref="canvas" style="width: 100%; height: 100%;"></canvas>
</template>

<style scoped></style>
