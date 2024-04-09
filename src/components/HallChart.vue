<template>
    <div class="relative h-screen w-screen">
        <VChart ref="chart"
                class="h-full w-full"
                autoresize manual-update
        />
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
import {useRouter} from "vue-router";
import {listen, UnlistenFn} from '@tauri-apps/api/event'
import {onMounted, ref} from "vue";
import {startHallEffectMode} from "../invokes.ts";
import {stop} from "../invokes.ts";
import VChart from 'vue-echarts';
import * as echarts from 'echarts/core';
import {
    GridComponent,
    DatasetComponent,
    MarkLineComponent,
    MarkPointComponent,
    TitleComponent,
} from 'echarts/components';
import {LineChart} from 'echarts/charts';
import {UniversalTransition} from 'echarts/features';
import {CanvasRenderer} from 'echarts/renderers';
import {sensorChartInit} from "./config.ts";

echarts.use([
    TitleComponent,
    GridComponent,
    MarkPointComponent,
    MarkLineComponent,
    LineChart,
    CanvasRenderer,
    DatasetComponent,
    UniversalTransition,
])


const chart = ref<InstanceType<typeof VChart> | null>(null)
const router = useRouter()


const x = Array.from({length: 500}, (_, i) => i)
const renderData = (data: number[][]) => {
    data.push(x)
    chart.value?.setOption({
        dataset: {
            source: data
        }
    })
}


let unListenFn: UnlistenFn

const onCloseClick = async () => {
    await stop()
    unListenFn()
    await router.replace('/')
}

const startEventListener = async () => {
    unListenFn = await listen('hall', (event: any) => {
        renderData(event.payload)
    });
}


onMounted(async () => {
    const query = router.currentRoute.value.query as SensorMode
    const gradient = ['x','y','z']
    const titles = Array.from({length: 3}, (_, i) => `G${gradient[i]} DS#${query.sensor}`)
    chart.value?.setOption(sensorChartInit(titles, 3, 1))
    await startHallEffectMode({payload: query})
    await startEventListener()
})


</script>

<style scoped>

</style>