<template>
    <div class="h-screen w-screen flex justify-center items-center bg-gray-100">
        <div class="card w-11/12 h-5/6 shadow-xl bg-primary-content/10">
            <div class="card-body break-words overflow-hidden">
                <div class="card-title">Data</div>
                <div class="h-full overflow-y-scroll">
                    <code v-for="(v,i) in text" :key="i">
                        {{ v }}
                    </code>
                </div>
                <div class="card-actions justify-end">
                    <button class="btn btn-success" @click="text.length = 0">Clear</button>
                </div>
            </div>
        </div>
        <button
            class="btn btn-circle shadow-xl bg-red-600 text-white fixed bottom-3.5 right-3.5 hover:bg-red-800"
            @click="onCloseClick"
        >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24"
                 stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
            </svg>
        </button>
    </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from "vue";
import {listen, UnlistenFn} from '@tauri-apps/api/event'
import {start_hall_mode, stop} from "../invokes.ts";
import {useRouter} from "vue-router";

const router = useRouter()

let unListenFn: UnlistenFn
const text = ref<string[]>([])
const startEventListener = async () => {
    unListenFn = await listen('hall', (event: any) => {
        text.value += event.payload
    });
}

const onCloseClick = async () => {
    await stop()
    unListenFn()
    await router.replace('/')
}


onMounted(async () => {
    setInterval(() => {
        text.value.push("hello")
    }, 1)
    // const query = router.currentRoute.value.query as SensorMode
    // await startEventListener()
    // await start_hall_mode({payload: query})
})
</script>

<style scoped>

</style>