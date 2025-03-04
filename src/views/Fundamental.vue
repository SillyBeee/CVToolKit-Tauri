<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { message } from 'ant-design-vue';
import { UploadOutlined } from '@ant-design/icons-vue';
import type { UploadChangeParam, UploadFile } from 'ant-design-vue';

const value1 = ref(0);
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
    const base64Data = await invoke('process_threshold', { fileData: uint8Array });
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


</script>

<template>
  <h1>ciallo</h1>
  <a-slider class="slider threshold" v-model:value="value1"  />
  <a-upload
      v-model:file-list="fileList"
      name="file"
      :customRequest="customRequest"
      @change="handleChange"
  >
    <a-button>
      <upload-outlined></upload-outlined>
      Click to Upload
    </a-button>
  </a-upload>
  <img :src="processedImage" alt="processed image" />
</template>

<style scoped>
h1 {
  color: red;
  position: fixed;
  top: 50%;
  left: 50%;
}

.slider {
  position: fixed;
  top: 40%;
  left: 50%;
  width: 200px;
  height: 20px;

  cursor: grab;
}
</style>