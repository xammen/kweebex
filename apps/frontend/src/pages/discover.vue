<script setup lang="ts">
import { commonProjectTypeCategoryMessages, useVIntl } from '@kweebex/ui'
import { FEATURES } from '../../../../config'

import NavTabs from '~/components/ui/NavTabs.vue'

const { formatMessage } = useVIntl()

const flags = useFeatureFlags()
const cosmetics = useCosmetics()
const route = useRoute()

const allowTabChanging = computed(() => !route.query.sid)

const selectableProjectTypes = [
	{
		label: formatMessage(commonProjectTypeCategoryMessages.mod),
		href: `/discover/mods`,
		type: 'mods',
		shown: FEATURES.mods,
	},
	{
		label: formatMessage(commonProjectTypeCategoryMessages.modpack),
		href: `/discover/modpacks`,
		type: 'modpacks',
		shown: FEATURES.modpacks,
	},
	{
		label: formatMessage(commonProjectTypeCategoryMessages.resourcepack),
		href: `/discover/resourcepacks`,
		type: 'resourcepacks',
		shown: FEATURES.resourcepacks,
	},
	{
		label: formatMessage(commonProjectTypeCategoryMessages.datapack),
		href: `/discover/datapacks`,
		type: 'datapacks',
		shown: FEATURES.datapacks,
	},
	{
		label: formatMessage(commonProjectTypeCategoryMessages.shader),
		href: `/discover/shaders`,
		type: 'shaders',
		shown: FEATURES.shaders,
	},
	{
		label: formatMessage(commonProjectTypeCategoryMessages.plugin),
		href: `/discover/plugins`,
		type: 'plugins',
		shown: FEATURES.plugins,
	},
	{
		label: formatMessage(commonProjectTypeCategoryMessages.server),
		href: `/discover/servers`,
		type: 'servers',
		shown: flags.value.serverDiscovery,
	},
].filter(item => item.shown !== false)
</script>
<template>
	<div class="new-page sidebar" :class="{ 'alt-layout': !cosmetics.rightSearchLayout }">
		<section class="normal-page__header mb-4 flex flex-col gap-4">
			<div id="discover-header-prefix" class="empty:hidden"></div>
			<NavTabs
				v-if="!flags.projectTypesPrimaryNav && allowTabChanging"
				:links="selectableProjectTypes"
				class="hidden md:flex"
			/>
		</section>
		<NuxtPage />
	</div>
</template>
