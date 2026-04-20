<template>
  <div class="settings-tab-panel">
    <div class="settings-grid">
      <div class="settings-section">
        <div class="section-header">
          <n-icon size="20"><RefreshOutline /></n-icon>
          <h3>{{ props.t('setting.update.title') }}</h3>
        </div>
        <div class="section-card">
          <div class="update-status">
            <div class="version-info">
              <span>{{ props.t('setting.update.currentVersion') }}: {{ props.updateStore.appVersion }}</span>
              <n-tag
                v-if="props.updateStore.hasUpdate"
                type="warning"
                size="small"
                round
                :bordered="false"
              >
                {{ props.t('setting.update.hasUpdate') }}
              </n-tag>
              <n-tag v-else type="success" size="small" round :bordered="false">
                {{ props.t('setting.update.latest') }}
              </n-tag>
            </div>
            <n-button
              size="small"
              secondary
              :loading="props.checkingUpdate"
              :disabled="props.updateStore.isChecking"
              @click="props.handleCheckUpdate"
            >
              {{
                props.checkingUpdate
                  ? props.t('setting.update.checking')
                  : props.t('setting.update.checkNow')
              }}
            </n-button>
          </div>

          <div v-if="props.updateStore.hasUpdate" class="update-alert-card">
            <div class="update-meta">
              <div class="meta-box">
                <div class="meta-label">{{ props.t('setting.update.newVersion') }}</div>
                <div class="meta-value">v{{ props.updateStore.latestVersion }}</div>
              </div>
              <div class="meta-box">
                <div class="meta-label">{{ props.t('setting.update.currentVersion') }}</div>
                <div class="meta-value">v{{ props.updateStore.appVersion }}</div>
              </div>
            </div>

            <div v-if="props.updateStore.releaseNotes" class="update-notes-preview">
              <span class="meta-label">{{ props.t('setting.update.releaseNotes') }}</span>
              <div class="notes custom-scrollbar">
                {{ props.updateStore.releaseNotes }}
              </div>
            </div>

            <div v-if="!props.updateStore.supportsInAppUpdate" class="update-platform-hint">
              {{ props.t('setting.update.externalUpdateHint') }}
            </div>

            <div class="update-actions">
              <n-button
                type="primary"
                strong
                :loading="props.updateStore.supportsInAppUpdate && props.isUpdating"
                :disabled="
                  props.updateStore.supportsInAppUpdate
                    ? props.isUpdating
                    : !props.updateStore.canOpenReleasePage
                "
                @click="props.handleUpdateNow"
              >
                <template #icon>
                  <n-icon>
                    <OpenOutline v-if="!props.updateStore.supportsInAppUpdate" />
                    <DownloadOutline v-else />
                  </n-icon>
                </template>
                {{
                  !props.updateStore.supportsInAppUpdate
                    ? props.t('setting.update.openReleasePage')
                    : props.updateStatus === 'installing'
                      ? props.t('setting.update.installing')
                      : props.isUpdating
                        ? props.t('setting.update.downloading')
                        : props.t('setting.update.updateNow')
                }}
              </n-button>
              <n-button
                size="small"
                text
                :disabled="
                  props.checkingUpdate ||
                  (props.updateStore.supportsInAppUpdate && props.isUpdating)
                "
                @click="props.handleCheckUpdate"
              >
                {{ props.t('setting.update.checkAgain') }}
              </n-button>
            </div>

            <div v-if="props.updateStore.supportsInAppUpdate && props.showUpdateProgress" class="update-progress">
              <div class="progress-header">
                <span class="progress-text">{{
                  props.updateMessage || props.t('setting.update.downloading')
                }}</span>
                <span class="progress-value">{{ props.updateProgress.toFixed(0) }}%</span>
              </div>
              <n-progress
                type="line"
                :percentage="props.updateProgress"
                :processing="props.updateStatus === 'downloading'"
                :status="props.updateStatus === 'error' ? 'error' : 'default'"
                :show-indicator="false"
              />
            </div>

            <div
              v-else-if="props.updateStore.supportsInAppUpdate && props.updateStatus === 'error'"
              class="update-error"
            >
              {{ props.updateMessage || props.t('setting.update.updateFailed') }}
            </div>
          </div>

          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-label">{{ props.t('setting.update.autoCheck') }}</div>
            </div>
            <n-switch
              :value="props.updateStore.autoCheckUpdate"
              @update:value="props.onAutoCheckUpdateChange"
            />
          </div>
          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-label">{{ props.t('setting.update.channel') }}</div>
            </div>
            <n-select
              :value="props.updateStore.updateChannel"
              :options="props.updateChannelOptions"
              size="small"
              class="setting-input"
              @update:value="props.onUpdateChannelChange"
            />
          </div>
        </div>
      </div>

      <div class="settings-section">
        <div class="section-header">
          <n-icon size="20"><ArchiveOutline /></n-icon>
          <h3>{{ props.t('setting.backup.title') }}</h3>
        </div>
        <div class="section-card">
          <div class="setting-row align-start">
            <div class="setting-info">
              <div class="setting-label">{{ props.t('setting.backup.description') }}</div>
              <div class="setting-desc">{{ props.t('setting.backup.restoreHint') }}</div>
            </div>
            <div class="backup-actions">
              <n-button
                size="small"
                secondary
                :loading="props.backupExporting"
                :disabled="props.backupBusy"
                @click="props.handleExportBackup"
              >
                {{ props.t('setting.backup.exportAction') }}
              </n-button>
              <n-button
                size="small"
                secondary
                :loading="props.backupValidating"
                :disabled="props.backupBusy"
                @click="props.handleValidateBackup"
              >
                {{ props.t('setting.backup.validateAction') }}
              </n-button>
              <n-button
                size="small"
                type="warning"
                :loading="props.backupRestoring"
                :disabled="props.backupBusy"
                @click="props.handleRestoreBackup"
              >
                {{ props.t('setting.backup.restoreAction') }}
              </n-button>
            </div>
          </div>

          <div v-if="props.backupPreview" class="backup-preview">
            <div class="backup-preview-row">
              <span class="meta-label">{{ props.t('setting.backup.selectedFile') }}</span>
              <span class="backup-path">{{ props.backupPreview.file_path }}</span>
            </div>
            <div class="backup-preview-row">
              <span class="meta-label">{{ props.t('setting.backup.subscriptionCount') }}</span>
              <span class="meta-value">{{ props.backupPreview.subscriptions_count }}</span>
            </div>
            <div
              class="backup-preview-row"
              :class="{ warning: props.backupPreview.warnings.length > 0 }"
            >
              <span class="meta-label">{{ props.t('setting.backup.warningCount') }}</span>
              <span class="meta-value">{{ props.backupPreview.warnings.length }}</span>
            </div>
            <div v-if="props.backupPreview.warnings.length > 0" class="backup-warning-list">
              <div
                v-for="(warning, idx) in props.backupPreview.warnings"
                :key="idx"
                class="backup-warning-item"
              >
                {{ warning }}
              </div>
            </div>
          </div>
          <div v-else class="setting-desc">
            {{ props.t('setting.backup.noPreview') }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  ArchiveOutline,
  DownloadOutline,
  OpenOutline,
  RefreshOutline,
} from '@vicons/ionicons5'
import type { UpdateChannel } from '@/stores/app/UpdateStore'
import type { useUpdateStore } from '@/stores'
import type { BackupImportResult } from '@/services/system-service'

type UpdateStoreLike = ReturnType<typeof useUpdateStore>

interface Option<T extends string = string> {
  label: string
  value: T
}

const props = defineProps<{
  t: (key: string, params?: Record<string, string | number>) => string
  updateStore: UpdateStoreLike
  checkingUpdate: boolean
  updateStatus: string
  updateProgress: number
  updateMessage: string
  isUpdating: boolean
  showUpdateProgress: boolean
  updateChannelOptions: Option<UpdateChannel>[]
  backupExporting: boolean
  backupValidating: boolean
  backupRestoring: boolean
  backupBusy: boolean
  backupPreview: BackupImportResult | null
  handleUpdateNow: () => void | Promise<void>
  handleCheckUpdate: () => void | Promise<void>
  onAutoCheckUpdateChange: (value: boolean) => void
  onUpdateChannelChange: (value: UpdateChannel) => void | Promise<void>
  handleExportBackup: () => void | Promise<void>
  handleValidateBackup: () => void | Promise<void>
  handleRestoreBackup: () => void | Promise<void>
}>()
</script>
