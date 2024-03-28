<template>
    <div class="hero h-screen" style="background-image: url(/bg.png);">
        <div class="hero-overlay bg-opacity-35"></div>
        <div class="hero-content text-center">
            <div class="max-w-md">
                <div class="skeleton w-80 h-12 m-3.5" v-if="loading"></div>
                <div v-else>
                    <select class="select select-primary select-md m-3.5" v-model="selected">
                        <option disabled value="">Select a DS sensor!</option>
                        <option v-for="(v,i) in sensors" :key="i"> {{ v }}</option>
                    </select>
                    <button class="btn bg-lime-300 hover:bg-lime-500 border-none shadow-xl" @click="scan">Refresh
                    </button>
                </div>
                <button
                    class="btn m-3 btn-primary shadow-xl disabled:text-gray-300"
                    :disabled="selected.length === 0"
                    @click="router.replace({path: '/chart', query: {sensor: selected, mode: 'normal'}})"
                >
                    Normal
                </button>
                <button
                    class="btn m-3 btn-secondary shadow-xl disabled:text-gray-300"
                    :disabled="selected.length === 0"
                    @click="router.replace({path: '/hall', query: {sensor: selected, mode: 'hall'}})"
                >
                    Hall Effect
                </button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from "vue";
import {scanSensors} from "../invokes.ts";
import {useRouter} from "vue-router";

const router = useRouter()
const loading = ref(true)
const sensors = ref<string[]>([])
const selected = ref('')
const scan = async () => {
    selected.value = ''
    loading.value = true
    sensors.value = await scanSensors()
    loading.value = false
}

onMounted(() => {
    scan()
})
</script>

<style scoped></style>