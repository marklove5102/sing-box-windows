<template>
  <div class="settings-tab-panel">
    <div class="settings-grid">
      <div class="settings-section full-width">
        <div class="section-header">
          <n-icon size="20"><InformationCircleOutline /></n-icon>
          <h3>{{ props.t('setting.about.title') }}</h3>
        </div>
        <div class="section-card">
          <div class="about-list">
            <div class="about-item">
              <span class="label">{{ props.t('setting.appVersion') }}</span>
              <span class="value">{{ props.updateStore.appVersion }}</span>
            </div>
            <div class="about-item">
              <span class="label">{{ props.t('setting.kernel.version') }}</span>
              <span class="value">{{
                props.kernelStore.hasVersionInfo()
                  ? props.formatVersion(props.kernelStore.getVersionString())
                  : props.t('setting.notInstalled')
              }}</span>
            </div>
            <div class="about-item">
              <span class="label">{{ props.t('setting.about.system') }}</span>
              <span class="value">{{ props.platformInfo?.display_name || props.t('common.loading') }}</span>
            </div>
            <div class="about-item">
              <span class="label">{{ props.t('setting.about.license') }}</span>
              <span class="value">MIT License</span>
            </div>
            <div class="about-actions">
              <n-button
                text
                tag="a"
                href="https://github.com/xinggaoya/sing-box-windows"
                target="_blank"
              >
                <template #icon>
                  <n-icon><LogoGithub /></n-icon>
                </template>
                GitHub
              </n-button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { InformationCircleOutline, LogoGithub } from '@vicons/ionicons5'
import type { useKernelStore, useUpdateStore } from '@/stores'

type KernelStoreLike = ReturnType<typeof useKernelStore>
type UpdateStoreLike = ReturnType<typeof useUpdateStore>

const props = defineProps<{
  t: (key: string, params?: Record<string, string | number>) => string
  updateStore: UpdateStoreLike
  kernelStore: KernelStoreLike
  platformInfo: { os: string; arch: string; display_name: string } | null
  formatVersion: (value: string) => string
}>()
</script>
