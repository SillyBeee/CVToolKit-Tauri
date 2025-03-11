import { createRouter, createWebHistory } from 'vue-router';
import Fundamental from '../views/Fundamental.vue';
import JudgePara from '../views/JudgePara.vue';
import GammaCorrection from '../views/GammaCorrection.vue';
const routes = [
    {
        path: '/Fundamental',
        name: 'Fundamental',
        component: Fundamental ,
        meta: { keepAlive: true}
    },
    {
        path: '/JudgePara',
        name: 'JudgePara',
        component: JudgePara
    },
    {
        path: '/GammaCorrection',
        name: 'GammaCorrection',
        component: GammaCorrection
    }
];

const router = createRouter({
    history: createWebHistory(),
    routes
});

export default router;
