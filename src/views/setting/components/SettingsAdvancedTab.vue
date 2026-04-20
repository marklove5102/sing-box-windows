<template>
  <div class="settings-tab-panel">
    <div class="settings-grid">
      <div class="settings-section">
        <div class="section-header">
          <n-icon size="20"><GlobeOutline /></n-icon>
          <h3>{{ props.t('setting.network.title') }}</h3>
        </div>
        <div class="section-card">
          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-label">{{ props.t('setting.network.ipv6') }}</div>
              <div class="setting-desc">{{ props.t('setting.network.ipv6Desc') }}</div>
            </div>
            <n-switch :value="props.appStore.preferIpv6" @update:value="props.onIpVersionChange" />
          </div>
          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-label">{{ props.t('setting.network.ports') }}</div>
              <div class="setting-desc">{{ props.t('setting.network.portsDesc') }}</div>
            </div>
            <n-button size="small" secondary @click="props.showPortSettings">
              {{ props.t('setting.network.configure') }}
            </n-button>
          </div>
          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-label">{{ props.t('setting.network.allowLanAccess') }}</div>
              <div class="setting-desc">{{ props.t('setting.network.allowLanAccessDesc') }}</div>
            </div>
            <n-switch :value="props.appStore.allowLanAccess" @update:value="props.onLanAccessChange" />
          </div>
        </div>
      </div>

      <div class="settings-section full-width">
        <div class="section-header">
          <n-icon size="20"><OptionsOutline /></n-icon>
          <h3>{{ props.t('setting.proxyAdvanced.title') }}</h3>
        </div>
        <div class="section-card">
          <n-form label-placement="top" class="advanced-form">
            <n-grid :cols="24" :x-gap="24" :y-gap="16">
              <n-grid-item :span="24">
                <n-form-item :label="props.t('setting.proxyAdvanced.systemBypass')">
                  <n-input
                    v-model:value="proxyAdvancedForm.systemProxyBypass"
                    type="textarea"
                    :rows="3"
                    :placeholder="props.t('setting.proxyAdvanced.systemBypassPlaceholder')"
                  />
                </n-form-item>
              </n-grid-item>

              <n-grid-item :span="24">
                <div class="subsection-title">{{ props.t('setting.proxyAdvanced.tunTitle') }}</div>
              </n-grid-item>

              <n-grid-item :span="12" :s="24" :m="12">
                <n-form-item :label="props.t('setting.proxyAdvanced.tunMtu')">
                  <n-input-number v-model:value="proxyAdvancedForm.tunMtu" :min="576" :max="9000" />
                </n-form-item>
              </n-grid-item>

              <n-grid-item :span="12" :s="24" :m="12">
                <n-form-item :label="props.t('setting.proxyAdvanced.tunStack')">
                  <n-select v-model:value="proxyAdvancedForm.tunStack" :options="props.tunStackOptions" />
                </n-form-item>
              </n-grid-item>

              <n-grid-item :span="24">
                <n-form-item :label="props.t('setting.proxyAdvanced.tunRouteExcludeAddress')">
                  <n-input
                    v-model:value="proxyAdvancedForm.tunRouteExcludeAddressText"
                    type="textarea"
                    :rows="4"
                    :placeholder="props.t('setting.proxyAdvanced.tunRouteExcludeAddressPlaceholder')"
                  />
                </n-form-item>
              </n-grid-item>

              <n-grid-item :span="24">
                <div class="toggles-row">
                  <div class="toggle-item">
                    <span>{{ props.t('setting.proxyAdvanced.enableIpv6') }}</span>
                    <n-switch v-model:value="proxyAdvancedForm.tunEnableIpv6" />
                  </div>
                  <div class="toggle-item">
                    <span>{{ props.t('setting.proxyAdvanced.autoRoute') }}</span>
                    <n-switch v-model:value="proxyAdvancedForm.tunAutoRoute" />
                  </div>
                  <div class="toggle-item">
                    <span>{{ props.t('setting.proxyAdvanced.strictRoute') }}</span>
                    <n-switch v-model:value="proxyAdvancedForm.tunStrictRoute" />
                  </div>
                  <div class="toggle-item">
                    <span>{{ props.t('setting.proxyAdvanced.tunSelfHeal') }}</span>
                    <n-switch v-model:value="proxyAdvancedForm.tunSelfHealEnabled" />
                  </div>
                </div>
              </n-grid-item>

              <n-grid-item :span="12" :s="24" :m="12">
                <n-form-item :label="props.t('setting.proxyAdvanced.tunSelfHealCooldown')">
                  <n-input-number
                    v-model:value="proxyAdvancedForm.tunSelfHealCooldownSecs"
                    :min="15"
                    :max="600"
                    :disabled="!proxyAdvancedForm.tunSelfHealEnabled"
                  />
                </n-form-item>
              </n-grid-item>

              <n-grid-item :span="24">
                <n-button
                  type="primary"
                  block
                  :loading="savingAdvanced"
                  @click="saveProxyAdvancedSettings"
                >
                  {{ props.t('setting.proxyAdvanced.save') }}
                </n-button>
              </n-grid-item>
            </n-grid>
          </n-form>
        </div>
      </div>

      <div class="settings-section full-width">
        <div class="section-header">
          <n-icon size="20"><OptionsOutline /></n-icon>
          <h3>{{ props.t('setting.singboxProfile.title') }}</h3>
        </div>
        <div class="section-card">
          <div v-if="props.usingOriginalConfig" class="alert-box info">
            <n-icon size="18"><InformationCircleOutline /></n-icon>
            <span>{{ props.t('setting.singboxProfile.originalConfigHint') }}</span>
          </div>
          <n-form label-placement="top" class="advanced-form">
            <n-grid :cols="24" :x-gap="24" :y-gap="16">
              <n-grid-item :span="24">
                <div class="subsection-title">{{ props.t('setting.singboxProfile.routingTitle') }}</div>
              </n-grid-item>

              <n-grid-item :span="12" :s="24" :m="12">
                <n-form-item :label="props.t('setting.singboxProfile.defaultOutbound')">
                  <n-select
                    v-model:value="singboxProfileForm.defaultProxyOutbound"
                    :options="defaultOutboundOptions"
                  />
                </n-form-item>
              </n-grid-item>

              <n-grid-item :span="12" :s="24" :m="12">
                <n-form-item :label="props.t('setting.singboxProfile.downloadDetour')">
                  <n-select
                    v-model:value="singboxProfileForm.downloadDetour"
                    :options="downloadDetourOptions"
                  />
                </n-form-item>
              </n-grid-item>

              <n-grid-item :span="24">
                <div class="toggles-row">
                  <div class="toggle-item">
                    <span>{{ props.t('setting.singboxProfile.blockAds') }}</span>
                    <n-switch v-model:value="singboxProfileForm.blockAds" />
                  </div>
                  <div class="toggle-item">
                    <span>{{ props.t('setting.singboxProfile.dnsHijack') }}</span>
                    <n-switch v-model:value="singboxProfileForm.dnsHijack" />
                  </div>
                  <div class="toggle-item">
                    <span>{{ props.t('setting.singboxProfile.enableAppGroups') }}</span>
                    <n-switch v-model:value="singboxProfileForm.enableAppGroups" />
                  </div>
                  <div class="toggle-item">
                    <span>{{ props.t('setting.singboxProfile.fakeDnsEnabled') }}</span>
                    <n-switch v-model:value="singboxProfileForm.fakeDnsEnabled" />
                  </div>
                </div>
              </n-grid-item>

              <n-grid-item :span="24">
                <div class="subsection-title">{{ props.t('setting.singboxProfile.fakeDnsTitle') }}</div>
              </n-grid-item>

              <n-grid-item :span="12" :s="24" :m="12">
                <n-form-item :label="props.t('setting.singboxProfile.fakeDnsFilterMode')">
                  <n-select
                    v-model:value="singboxProfileForm.fakeDnsFilterMode"
                    :options="fakeDnsFilterOptions"
                    :disabled="!singboxProfileForm.fakeDnsEnabled"
                  />
                </n-form-item>
              </n-grid-item>

              <n-grid-item :span="12" :s="24" :m="12">
                <n-form-item :label="props.t('setting.singboxProfile.fakeDnsIpv4Range')">
                  <n-input
                    v-model:value="singboxProfileForm.fakeDnsIpv4Range"
                    placeholder="198.18.0.0/15"
                    :disabled="!singboxProfileForm.fakeDnsEnabled"
                  />
                </n-form-item>
              </n-grid-item>

              <n-grid-item :span="12" :s="24" :m="12">
                <n-form-item :label="props.t('setting.singboxProfile.fakeDnsIpv6Range')">
                  <n-input
                    v-model:value="singboxProfileForm.fakeDnsIpv6Range"
                    placeholder="fc00::/18"
                    :disabled="!singboxProfileForm.fakeDnsEnabled"
                  />
                </n-form-item>
              </n-grid-item>

              <n-grid-item :span="24">
                <div class="subsection-title">{{ props.t('setting.singboxProfile.dnsTitle') }}</div>
              </n-grid-item>

              <n-grid-item :span="12" :s="24" :m="12">
                <n-form-item :label="props.t('setting.singboxProfile.dnsProxy')">
                  <n-input
                    v-model:value="singboxProfileForm.dnsProxy"
                    placeholder="https://1.1.1.1/dns-query"
                  />
                </n-form-item>
              </n-grid-item>

              <n-grid-item :span="12" :s="24" :m="12">
                <n-form-item :label="props.t('setting.singboxProfile.dnsCn')">
                  <n-input
                    v-model:value="singboxProfileForm.dnsCn"
                    placeholder="h3://dns.alidns.com/dns-query"
                  />
                </n-form-item>
              </n-grid-item>

              <n-grid-item :span="12" :s="24" :m="12">
                <n-form-item :label="props.t('setting.singboxProfile.dnsResolver')">
                  <n-input
                    v-model:value="singboxProfileForm.dnsResolver"
                    placeholder="114.114.114.114"
                  />
                </n-form-item>
              </n-grid-item>

              <n-grid-item :span="12" :s="24" :m="12">
                <n-form-item :label="props.t('setting.singboxProfile.urltestUrl')">
                  <n-input
                    v-model:value="singboxProfileForm.urltestUrl"
                    placeholder="http://cp.cloudflare.com/generate_204"
                  />
                </n-form-item>
              </n-grid-item>

              <n-grid-item :span="24">
                <n-button
                  type="primary"
                  block
                  :loading="savingSingboxProfile"
                  @click="saveSingboxProfileSettings"
                >
                  {{ props.t('setting.singboxProfile.save') }}
                </n-button>
              </n-grid-item>
            </n-grid>
          </n-form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { GlobeOutline, InformationCircleOutline, OptionsOutline } from '@vicons/ionicons5'
import { useMessage } from 'naive-ui'
import type { useAppStore } from '@/stores'
import { useAdvancedSettingsForm } from '@/views/setting/useAdvancedSettingsForm'

type LabeledOption = { label: string; value: string }
type AppStoreLike = ReturnType<typeof useAppStore>

const props = defineProps<{
  t: (key: string, params?: Record<string, string | number>) => string
  appStore: AppStoreLike
  tunStackOptions: LabeledOption[]
  usingOriginalConfig: boolean
  onIpVersionChange: (value: boolean) => void | Promise<void>
  onLanAccessChange: (value: boolean) => void | Promise<void>
  showPortSettings: () => void
}>()

const message = useMessage()

const {
  savingAdvanced,
  proxyAdvancedForm,
  savingSingboxProfile,
  singboxProfileForm,
  defaultOutboundOptions,
  downloadDetourOptions,
  fakeDnsFilterOptions,
  saveProxyAdvancedSettings,
  saveSingboxProfileSettings,
} = useAdvancedSettingsForm({
  appStore: props.appStore,
  message,
  t: props.t,
})
</script>
