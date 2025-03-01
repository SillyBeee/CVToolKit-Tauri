import { createRouter, createWebHistory } from 'vue-router';
import Fundamental from '../views/Fundamental.vue';
import JudgePara from '../views/JudgePara.vue';

const routes = [
    {
        path: '/',
        name: 'Fundamental',
        component: Fundamental
    },
    {
        path: '/JudgePara',
        name: 'JudgePara',
        component: JudgePara
    }
];

const router = createRouter({
    history: createWebHistory(),
    routes
});

export default router;
