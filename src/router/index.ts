import { createRouter, createWebHistory } from 'vue-router';
import Fundamental from '../views/Fundamental.vue';
import JudgePara from '../views/JudgePara.vue';

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
    }
];

const router = createRouter({
    history: createWebHistory(),
    routes
});

export default router;
