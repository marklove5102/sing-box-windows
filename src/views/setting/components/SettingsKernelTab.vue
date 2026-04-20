<template>
  <div class="settings-tab-panel">
    <div class="settings-grid">
      <div class="settings-section full-width">
        <div class="section-header">
          <n-icon size="20"><SettingsOutline /></n-icon>
          <h3>{{ props.t('setting.kernel.title') }} · {{ props.t('setting.kernel.advancedTag') }}</h3>
        </div>
        <div class="section-card">
          <div class="alert-box info">
            <n-icon size="18"><InformationCircleOutline /></n-icon>
            <span>{{ props.t('setting.kernel.embeddedHint') }}</span>
          </div>

          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-label">{{ props.t('setting.kernel.version') }}</div>
              <div class="setting-desc">{{ props.t('setting.kernel.description') }}</div>
            </div>
            <div class="setting-action">
              <n-tag v-if="props.kernelStore.hasVersionInfo()" type="success" round :bordered="false">
                {{ props.formatVersion(props.kernelStore.getVersionString()) }}
              </n-tag>
              <n-tag v-else type="error" round :bordered="false">
                {{ props.t('setting.notInstalled') }}
              </n-tag>
            </div>
          </div>

          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-label">{{ props.t('setting.kernel.selectVersion') }}</div>
              <div class="setting-desc">{{ props.t('setting.kernel.selectVersionDesc') }}</div>
            </div>
            <n-select
              :value="props.selectedKernelVersion"
              :options="props.kernelVersionOptions"
              :loading="props.kernelStore.isLoading"
              :disabled="props.downloading"
              size="small"
              class="setting-input"
              placeholder="Latest"
              @update:value="props.onSelectedKernelVersionChange"
            />
          </div>

          <div
            v-if="props.hasNewVersion || !props.kernelStore.hasVersionInfo()"
            class="alert-box warning"
          >
            <n-icon size="18"><WarningOutline /></n-icon>
            <span>
              {{
                props.hasNewVersion
                  ? props.t('setting.update.newVersionFound', {
                      version: props.kernelLatestVersion || props.t('setting.newVersionFound'),
                    })
                  : props.t('setting.kernel.installPrompt')
              }}
            </span>
          </div>

          <div v-if="props.downloading" class="download-box">
            <n-progress
              type="line"
              :percentage="props.downloadProgress"
              :processing="props.downloadProgress < 100"
              indicator-placement="inside"
            />
            <div class="download-text">{{ props.downloadMessage }}</div>
          </div>

          <div class="actions-row">
            <n-button
              type="default"
              :loading="props.loading"
              :disabled="props.downloading"
              block
              secondary
              @click="props.downloadTheKernel"
            >
              <template #icon>
                <n-icon><DownloadOutline /></n-icon>
              </template>
              {{
                props.hasNewVersion
                  ? props.t('setting.kernel.update')
                  : props.kernelStore.hasVersionInfo()
                    ? props.t('setting.kernel.redownload')
                    : props.t('setting.kernel.download')
              }}
            </n-button>
            <div class="sub-actions">
              <n-button size="small" ghost :disabled="props.downloading" @click="props.showManualDownloadModal">
                {{ props.t('setting.kernel.manualDownload') }}
              </n-button>
              <n-button size="small" ghost :disabled="props.downloading" @click="props.checkManualInstall">
                {{ props.t('setting.kernel.checkInstall') }}
              </n-button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  DownloadOutline,
  InformationCircleOutline,
  SettingsOutline,
  WarningOutline,
} from '@vicons/ionicons5'
import type { useKernelStore } from '@/stores'

type KernelStoreLike = ReturnType<typeof useKernelStore>

interface Option {
  label: string
  value: string | undefined
}

const props = defineProps<{
  t: (key: string, params?: Record<string, string | number>) => string
  kernelStore: KernelStoreLike
  selectedKernelVersion?: string
  kernelVersionOptions: Option[]
  hasNewVersion: boolean
  kernelLatestVersion: string
  downloading: boolean
  loading: boolean
  downloadProgress: number
  downloadMessage: string
  onSelectedKernelVersionChange: (value: string | undefined) => void
  downloadTheKernel: () => void | Promise<void>
  showManualDownloadModal: () => void
  checkManualInstall: () => void | Promise<void>
  formatVersion: (value: string) => string
}>()
</script>
