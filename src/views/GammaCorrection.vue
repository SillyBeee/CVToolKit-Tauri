<script setup lang="ts">
import { ref, watch } from "vue";
import { useRouter } from 'vue-router';
import { message } from 'ant-design-vue';
import { invoke } from "@tauri-apps/api/core";
import { UploadOutlined } from '@ant-design/icons-vue';
import type { UploadChangeParam, UploadFile } from 'ant-design-vue';

const selectedProcessingMethod = ref('gamma');
const router = useRouter();
const fileList = ref<UploadFile[]>([]);
const processedImage = ref<string | null>(null);
const value_gamma = ref(127);
//文件上传消息函数
const handleChange = (info: UploadChangeParam) => {
    if (info.file.status !== 'uploading') {
        console.log(info.file, info.fileList);
    }
    if (info.file.status === 'done') {
        message.success(`${info.file.name} 文件上传成功.`);
    } else if (info.file.status === 'error') {
        message.error(`${info.file.name} 文件上传失败.`);
    }
};

watch(value_gamma, async (newValue) => {
    if (fileList.value.length > 0) {
        const file = fileList.value[0];
        const arrayBuffer = await file.originFileObj.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);

        try {
            const base64Data = await invoke('process_threshold', { fileData: uint8Array, thresholdValue: newValue });
            const imageUrl = `data:image/png;base64,${base64Data}`;
            processedImage.value = imageUrl;
            // message.success(`文件处理成功，阈值: ${newValue}`);
        } catch (error) {
            message.error(`文件处理失败: ${error}`);
        }
    }
});



//图像上传及调用后端函数
const customRequest = async (options: any) => {
    const { file } = options;

    // 将文件读取为二进制数据
    const arrayBuffer = await file.arrayBuffer();
    const uint8Array = new Uint8Array(arrayBuffer);

    try {
        // 调用 Tauri 后端的 process_file 命令
        const base64Data = await invoke('process_threshold', { fileData: uint8Array, thresholdValue: value_gamma.value });
        // 构造图像的 data URL
        const imageUrl = `data:image/png;base64,${base64Data}`;

        // 更新 processedImage 以显示图像
        processedImage.value = imageUrl;

        // 更新 fileList 中的文件状态
        options.onSuccess('done');
        message.success(`${file.name} 文件处理成功`);
    } catch (error) {
        message.error(`${file.name} 文件处理失败: ${error}`);
        options.onError(error);
    }
};
const handleProcessingChange = (value: string) => {
    console.log(`选择的处理方法: ${value}`);

    // 根据不同的处理方法设置默认参数
    if (value === 'threshold') {
        selectedProcessingMethod.value = 'threshold';
        router.push('/Fundamental');
        message.success('跳转至二值化');
    }
    else if (value === 'gamma') {
        selectedProcessingMethod.value = 'gamma';
        router.push('/GammaCorrection');
        message.success('跳转至伽马校正');
    } else if (value === 'gaussian') {
        selectedProcessingMethod.value = 'gaussian';
    } else if (value === 'canny') {
        selectedProcessingMethod.value = 'canny';
    }
};
</script>

<template>
    <!-- 选择器 -->
    <a-select v-model:value="selectedProcessingMethod" style="width: 200px" class="processing-selector"
        placeholder="选择处理方式" @change="handleProcessingChange">
        <a-select-option value="threshold">二值化阈值处理</a-select-option>
        <a-select-option value="gamma">伽马校正</a-select-option>
        <a-select-option value="gaussian">高斯模糊</a-select-option>
        <a-select-option value="canny">Canny边缘检测</a-select-option>
        <a-select-option value="sobel">Sobel边缘检测</a-select-option>
    </a-select>
    <!-- 滑动条 -->
    <a-slider class="slider threshold" v-model:value="value_gamma" :min="0" :max="255"
        :marks="{ 0: '0', 127: '127', 255: '255' }" />
    <!-- 上传组件 -->
    <a-upload class="upload" v-model:file-list="fileList" name="file" :customRequest="customRequest"
        @change="handleChange">
        <a-button class="button">
            <upload-outlined></upload-outlined>
            点击上传文件
        </a-button>
    </a-upload>
    <img class="img" :src="processedImage" v-if="processedImage" alt="processedImage" />

</template>

<style scoped>
.upload {
    color: #1d1c1c;
    position: fixed;
    top: 70%;
    left: 47%;
}

.slider {
    position: fixed;
    top: 80%;
    left: 45%;
    width: 200px;
    height: 20px;

    cursor: grab;
}

.img {
    position: fixed;
    top: 17%;
    left: 39%;
    width: 300px;
    height: auto;
    border: 1px solid #000;
    border-radius: 5px;
    cursor: grab;
}

.processing-selector {
    position: fixed;
    top: 3%;
    left: 25%;
    width: 200px;
    height: 20px;
    cursor: grab;
}
</style>