import { createRouter, createWebHistory } from 'vue-router';
import Fundamental from '../views/Fundamental.vue';
import JudgePara from '../views/JudgePara.vue';
import GammaCorrection from '../views/GammaCorrection.vue';
import About from '../views/About.vue';
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
    },
    {
        path: '/About',
        name: 'About',
        component: About
    }
];

const router = createRouter({
    history: createWebHistory(),
    routes
});

export default router;
