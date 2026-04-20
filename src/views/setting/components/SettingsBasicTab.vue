<template>
  <div class="settings-tab-panel">
    <div class="settings-grid">
      <div class="settings-section">
        <div class="section-header">
          <n-icon size="20"><PowerOutline /></n-icon>
          <h3>{{ props.t('setting.startup.title') }}</h3>
        </div>
        <div class="section-card">
          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-label">{{ props.t('setting.autoStart.app') }}</div>
              <div class="setting-desc">{{ props.t('setting.autoStart.appDesc') }}</div>
            </div>
            <n-switch :value="props.autoStart" @update:value="props.onAutoStartChange" />
          </div>
          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-label">
                {{ props.t('setting.startup.autoHideToTrayOnAutostart') }}
              </div>
              <div class="setting-desc">
                {{ props.t('setting.startup.autoHideToTrayOnAutostartDesc') }}
              </div>
            </div>
            <n-switch
              :value="props.autoHideToTrayOnAutostart"
              @update:value="props.onAutoHideToTrayOnAutostartChange"
            />
          </div>
          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-label">{{ props.t('setting.startup.closeBehavior') }}</div>
              <div class="setting-desc">{{ props.t('setting.startup.closeBehaviorDesc') }}</div>
            </div>
            <n-select
              :value="props.trayCloseBehavior"
              :options="props.trayCloseBehaviorOptions"
              size="small"
              class="setting-input setting-input-wide"
              @update:value="props.onTrayCloseBehaviorChange"
            />
          </div>
        </div>
      </div>

      <div class="settings-section">
        <div class="section-header">
          <n-icon size="20"><GlobeOutline /></n-icon>
          <h3>{{ props.t('setting.general.title') }}</h3>
        </div>
        <div class="section-card">
          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-label">{{ props.t('setting.language.title') }}</div>
              <div class="setting-desc">{{ props.t('setting.language.description') }}</div>
            </div>
            <n-select
              :value="props.localeStore.locale"
              :options="props.languageOptions"
              size="small"
              class="setting-input"
              @update:value="props.onChangeLanguage"
            />
          </div>
        </div>
      </div>

      <div class="settings-section full-width">
        <div class="section-header">
          <n-icon size="20"><ColorPaletteOutline /></n-icon>
          <h3>{{ props.t('setting.theme.title') }}</h3>
        </div>
        <div class="section-card theme-card">
          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-label">{{ props.t('setting.theme.mode') }}</div>
              <div class="setting-desc">{{ props.t('setting.theme.modeDesc') }}</div>
            </div>
            <n-radio-group
              :value="props.themeStore.mode"
              size="small"
              class="theme-mode-selector"
              @update:value="props.onThemeModeChange"
            >
              <n-radio-button value="system">{{ props.t('setting.theme.system') }}</n-radio-button>
              <n-radio-button value="light">{{ props.t('setting.theme.light') }}</n-radio-button>
              <n-radio-button value="dark">{{ props.t('setting.theme.dark') }}</n-radio-button>
            </n-radio-group>
          </div>

          <div class="setting-row align-start">
            <div class="setting-info">
              <div class="setting-label">{{ props.t('setting.theme.accent') }}</div>
              <div class="setting-desc">{{ props.t('setting.theme.accentDesc') }}</div>
            </div>
            <div class="theme-accent">
              <n-color-picker
                :value="props.themeStore.accentColor"
                :modes="['hex']"
                size="small"
                :show-alpha="false"
                @update:value="props.onAccentChange"
              />
              <div class="preset-swatches">
                <button
                  v-for="color in props.accentPresets"
                  :key="color"
                  class="preset-swatch"
                  :style="{ background: color }"
                  @click="props.selectAccentPreset(color)"
                >
                  <span v-if="color === props.themeStore.accentColor" class="swatch-active"></span>
                </button>
              </div>
            </div>
          </div>

          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-label">{{ props.t('setting.theme.compactMode') }}</div>
              <div class="setting-desc">{{ props.t('setting.theme.compactDesc') }}</div>
            </div>
            <n-switch
              :value="props.themeStore.compactMode"
              @update:value="props.onCompactModeChange"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ColorPaletteOutline, GlobeOutline, PowerOutline } from '@vicons/ionicons5'
import type { Locale } from '@/stores/app/LocaleStore'
import type { ThemeMode } from '@/stores/app/ThemeStore'
import type { TrayCloseBehavior } from '@/stores/app/AppStore'
import type {
  useLocaleStore,
  useThemeStore,
} from '@/stores'

type LocaleStoreLike = ReturnType<typeof useLocaleStore>
type ThemeStoreLike = ReturnType<typeof useThemeStore>

interface Option<T extends string = string> {
  label: string
  value: T
}

const props = defineProps<{
  t: (key: string, params?: Record<string, string | number>) => string
  localeStore: LocaleStoreLike
  themeStore: ThemeStoreLike
  autoStart: boolean
  autoHideToTrayOnAutostart: boolean
  trayCloseBehavior: TrayCloseBehavior
  languageOptions: Option<Locale>[]
  trayCloseBehaviorOptions: Option<TrayCloseBehavior>[]
  accentPresets: string[]
  onAutoStartChange: (value: boolean) => void | Promise<void>
  onAutoHideToTrayOnAutostartChange: (value: boolean) => void | Promise<void>
  onTrayCloseBehaviorChange: (value: TrayCloseBehavior) => void | Promise<void>
  onChangeLanguage: (value: string) => void | Promise<void>
  onThemeModeChange: (value: ThemeMode) => void | Promise<void>
  onAccentChange: (value: string) => void | Promise<void>
  selectAccentPreset: (value: string) => void | Promise<void>
  onCompactModeChange: (value: boolean) => void | Promise<void>
}>()
</script>
