<script setup lang="ts">
import { ref , watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { message } from 'ant-design-vue';
import { UploadOutlined } from '@ant-design/icons-vue';
import type { UploadChangeParam, UploadFile } from 'ant-design-vue';


const value1 = ref(127);
const fileList = ref<UploadFile[]>([]);
const processedImage = ref<string | null>(null);

const handleChange = (info: UploadChangeParam) => {
  if (info.file.status !== 'uploading') {
    console.log(info.file, info.fileList);
  }
  if (info.file.status === 'done') {
    message.success(`${info.file.name} file uploaded successfully`);
  } else if (info.file.status === 'error') {
    message.error(`${info.file.name} file upload failed.`);
  }
};

const customRequest = async (options: any) => {
  const { file } = options;

  // 将文件读取为二进制数据
  const arrayBuffer = await file.arrayBuffer();
  const uint8Array = new Uint8Array(arrayBuffer);

  try {
    // 调用 Tauri 后端的 process_file 命令
    const base64Data = await invoke('process_threshold', { fileData: uint8Array , thresholdValue: value1.value });
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

watch(value1, async (newValue) => {
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

</script>

<template>

  <a-slider class="slider threshold" 
  v-model:value="value1"
  :min="0"
  :max="255" 
  :marks="{ 0: '0', 127: '127', 255: '255' }" />

  <a-upload
      class="upload"
      v-model:file-list="fileList"
      name="file"
      :customRequest="customRequest"
      @change="handleChange"
  >
    <a-button class="button">
      <upload-outlined></upload-outlined>
      Click to Upload
    </a-button>
  </a-upload>
  <img class="img" :src="processedImage"  v-if="processedImage" alt="processedImage" />

</template>

<style scoped>
.upload{
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
.img{
  position: fixed;
  top: 17%;
  left: 39%;
  width: 300px;
  height: auto;
  border: 1px solid #000;
  border-radius: 5px;
  cursor: grab;
}
</style>