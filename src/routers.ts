import {createRouter, createWebHashHistory} from 'vue-router'

const Greet = () => import('./components/Greet.vue')
const Chart = () => import('./components/Chart.vue')
const Hall = () => import('./components/HallChart.vue')

export default createRouter({
    history: createWebHashHistory(),
    routes: [
        {
            path: '/',
            component: Greet,
        },
        {
            path: '/chart',
            component: Chart
        },
        {
            path: '/hall',
            component: Hall
        }
    ]
})