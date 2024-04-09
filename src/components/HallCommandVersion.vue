<template>
    <div class="relative h-screen w-screen flex flex-col p-6 justify-center items-center"
         style="background-color:oklch(0.196166 0.063518 257.65198);">
        <div class="flex flex-row w-10/12 justify-evenly items-center">
            <select class="select select-primary w-3/4" v-model="selected">>
                <option disabled value="">Command?</option>
                <option v-for="(v,i) in commands" :key="i">{{ v }}</option>
            </select>
            <button class="btn btn-primary w-36" @click="send">Send</button>
            <button class="btn btn-warning w-18" @click="messages.length = 0">Clear</button>
        </div>
        <div class="h-full w-5/6 mt-3.5 text-gray-200 overflow-y-scroll">
            <pre class="whitespace-pre-wrap break-all"><code v-for="(v, i) in messages" :key="i">{{ v }}</code></pre>
        </div>
        <button
            class="btn border-none btn-circle shadow-xl bg-red-600 text-white fixed bottom-3.5 right-3.5 hover:bg-red-800"
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
import {sendCommand, startHallMode, stop} from "../invokes.ts";
import {useRouter} from "vue-router";

const router = useRouter()

let unListenFn: UnlistenFn
const messages = ref<string[]>([])

const commands = [
    'CMD_InfoSystem',
    'CMD_ID',
    'CMD_UID',
    'CMD_SID',
    'CMD_HID',
    'CMD_ECG',
    'CMD_ECG_PWR_OFF',
    'CMD_ECG3_PWR_ON',
    'CMD_IMU',
    'CMD_IMU_PWR_OFF',
    'CMD_IMU_PWR_ON',
    'CMD_HALL',
    'CMD_HALL_PWR_OFF',
    'CMD_HALL_PWR_ON',
    'CMD_HALL_B0_PWR_OFF',
    'CMD_HALL_B0_PWR_ON',
    'CMD_HALL_GRD_PWR_OFF',
    'CMD_HALL_GRD_PWR_ON'
]

const selected = ref('')

const send = async () => {
    selected.value.length !== 0 && await sendCommand({payload: selected.value})
}
const startEventListener = async () => {
    unListenFn = await listen('hall', (event: any) => {
        messages.value.push(event.payload)
    });
}

const onCloseClick = async () => {
    await stop()
    unListenFn()
    await router.replace('/')
}


onMounted(async () => {
    const query = router.currentRoute.value.query as SensorMode
    await startEventListener()
    await startHallMode({payload: query})
})
</script>

<style scoped>

</style>