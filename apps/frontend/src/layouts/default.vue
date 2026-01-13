<template>
	<div class="pointer-events-none fixed inset-0 z-[-1]">
		<div id="fixed-background-teleport" class="relative"></div>
	</div>
	<div class="pointer-events-none absolute inset-0 z-[-1]">
		<div id="absolute-background-teleport" class="relative"></div>
	</div>
	<div class="pointer-events-none absolute inset-0 z-50">
		<div
			class="over-the-top-random-animation"
			:style="{ '--_r-count': rCount }"
			:class="{ threshold: rCount > 20, 'rings-expand': rCount >= 40 }"
		>
			<div>
				<div
					class="animation-ring-3 flex items-center justify-center rounded-full border-4 border-solid border-brand bg-brand-highlight opacity-40"
				></div>
				<div
					class="animation-ring-2 flex items-center justify-center rounded-full border-4 border-solid border-brand bg-brand-highlight opacity-60"
				></div>
				<div
					class="animation-ring-1 flex items-center justify-center rounded-full border-4 border-solid border-brand bg-brand-highlight text-9xl font-extrabold text-contrast"
				>
					?
				</div>
			</div>
		</div>
	</div>
	<div
		ref="main_page"
		class="layout"
		:class="{
			'expanded-mobile-nav': isBrowseMenuOpen,
			'kweebex-parent__no-modal-blurs': !cosmetics.advancedRendering,
		}"
	>
		<RussiaBanner v-if="isRussia" />
		<TaxIdMismatchBanner v-if="showTinMismatchBanner" />
		<TaxComplianceBanner v-if="showTaxComplianceBanner" />
		<VerifyEmailBanner
			v-if="auth.user && !auth.user.email_verified && route.path !== '/auth/verify-email'"
			:has-email="auth?.user?.email"
		/>
		<SubscriptionPaymentFailedBanner
			v-if="
				user.subscriptions.some((x) => x.status === 'payment-failed') &&
				route.path !== '/settings/billing'
			"
		/>
		<StagingBanner v-if="config.public.apiBaseUrl.startsWith('https://staging-api.modrinth.com')" />
		<GeneratedStateErrorsBanner
			:errors="generatedStateErrors"
			:api-url="config.public.apiBaseUrl"
		/>
		<div class="header-wrapper" :class="{ 'header-wrapper--landing': route.path === '/' }">
			<header
				class="kweebex-header experimental-styles-within desktop-only relative z-[5] mx-auto grid max-w-[1280px] grid-cols-[1fr_auto] items-center gap-2 px-6 py-2 lg:grid-cols-[auto_1fr_auto]"
			>
			<div class="kweebex-header-ornaments"></div>
			<div class="kweebex-header-ornament-br"></div>
			<div>
				<NuxtLink
					to="/"
					:aria-label="formatMessage(messages.kweebexHomePage)"
					class="group hover:brightness-[--hover-brightness] focus-visible:brightness-[--hover-brightness]"
				>
					<TextLogo
						aria-hidden="true"
						class="h-7 w-auto text-contrast transition-transform group-active:scale-[0.98]"
					/>
				</NuxtLink>
			</div>
			<div
				:class="`col-span-2 row-start-2 flex flex-wrap justify-center ${flags.projectTypesPrimaryNav ? 'gap-2' : 'gap-4'} lg:col-span-1 lg:row-start-auto`"
			>
				<template v-if="flags.projectTypesPrimaryNav">
					<ButtonStyled
						v-if="FEATURES.mods"
						type="transparent"
						:highlighted="route.name === 'discover-mods' || route.path.startsWith('/mod/')"
						:highlighted-style="
							route.name === 'discover-mods' ? 'main-nav-primary' : 'main-nav-secondary'
						"
					>
						<nuxt-link to="/discover/mods">
							<BoxIcon aria-hidden="true" />
							{{ formatMessage(commonProjectTypeCategoryMessages.mod) }}
						</nuxt-link>
					</ButtonStyled>
					<ButtonStyled
						v-if="FEATURES.modpacks"
						type="transparent"
						:highlighted="route.name === 'discover-modpacks' || route.path.startsWith('/modpack/')"
						:highlighted-style="
							route.name === 'discover-modpacks' ? 'main-nav-primary' : 'main-nav-secondary'
						"
					>
						<nuxt-link to="/discover/modpacks">
							<PackageOpenIcon aria-hidden="true" />
							{{ formatMessage(commonProjectTypeCategoryMessages.modpack) }}
						</nuxt-link>
					</ButtonStyled>
					<ButtonStyled
						v-if="FEATURES.resourcepacks"
						type="transparent"
						:highlighted="
							route.name === 'discover-resourcepacks' || route.path.startsWith('/resourcepack/')
						"
						:highlighted-style="
							route.name === 'discover-resourcepacks' ? 'main-nav-primary' : 'main-nav-secondary'
						"
					>
						<nuxt-link to="/discover/resourcepacks">
							<PaintbrushIcon aria-hidden="true" />
							{{ formatMessage(commonProjectTypeCategoryMessages.resourcepack) }}
						</nuxt-link>
					</ButtonStyled>
					<ButtonStyled
						v-if="FEATURES.datapacks"
						type="transparent"
						:highlighted="
							route.name === 'discover-datapacks' || route.path.startsWith('/datapack/')
						"
						:highlighted-style="
							route.name === 'discover-datapacks' ? 'main-nav-primary' : 'main-nav-secondary'
						"
					>
						<nuxt-link to="/discover/datapacks">
							<BracesIcon aria-hidden="true" />
							{{ formatMessage(commonProjectTypeCategoryMessages.datapack) }}
						</nuxt-link>
					</ButtonStyled>
					<ButtonStyled
						v-if="FEATURES.shaders"
						type="transparent"
						:highlighted="route.name === 'discover-shaders' || route.path.startsWith('/shader/')"
						:highlighted-style="
							route.name === 'discover-shaders' ? 'main-nav-primary' : 'main-nav-secondary'
						"
					>
						<nuxt-link to="/discover/shaders">
							<GlassesIcon aria-hidden="true" />
							{{ formatMessage(commonProjectTypeCategoryMessages.shader) }}
						</nuxt-link>
					</ButtonStyled>
					<ButtonStyled
						v-if="FEATURES.plugins"
						type="transparent"
						:highlighted="route.name === 'discover-plugins' || route.path.startsWith('/plugin/')"
						:highlighted-style="
							route.name === 'discover-plugins' ? 'main-nav-primary' : 'main-nav-secondary'
						"
					>
						<nuxt-link to="/discover/plugins">
							<PlugIcon aria-hidden="true" />
							{{ formatMessage(commonProjectTypeCategoryMessages.plugin) }}
						</nuxt-link>
					</ButtonStyled>
				</template>
				<template v-else>
					<ButtonStyled
						type="transparent"
						:highlighted="isDiscovering || isDiscoveringSubpage"
						:highlighted-style="isDiscoveringSubpage ? 'main-nav-secondary' : 'main-nav-primary'"
					>
						<TeleportOverflowMenu
							:options="[
								{
									id: 'mods',
									action: '/discover/mods',
									shown: FEATURES.mods,
								},
								{
									id: 'modpacks',
									action: '/discover/modpacks',
									shown: FEATURES.modpacks,
								},
								{
									id: 'resourcepacks',
									action: '/discover/resourcepacks',
									shown: FEATURES.resourcepacks,
								},
								{
									id: 'datapacks',
									action: '/discover/datapacks',
									shown: FEATURES.datapacks,
								},
								{
									id: 'shaders',
									action: '/discover/shaders',
									shown: FEATURES.shaders,
								},
								{
									id: 'plugins',
									action: '/discover/plugins',
									shown: FEATURES.plugins,
								},
								{
									id: 'servers',
									action: '/discover/servers',
									shown: flags.serverDiscovery,
								},
							]"
							hoverable
						>
							<BoxIcon
								v-if="route.name === 'discover-mods' || route.path.startsWith('/mod/')"
								aria-hidden="true"
							/>
							<PaintbrushIcon
								v-else-if="
									route.name === 'discover-resourcepacks' || route.path.startsWith('/resourcepack/')
								"
								aria-hidden="true"
							/>
							<BracesIcon
								v-else-if="
									route.name === 'discover-datapacks' || route.path.startsWith('/datapack/')
								"
								aria-hidden="true"
							/>
							<PackageOpenIcon
								v-else-if="route.name === 'discover-modpacks' || route.path.startsWith('/modpack/')"
								aria-hidden="true"
							/>
							<GlassesIcon
								v-else-if="route.name === 'discover-shaders' || route.path.startsWith('/shader/')"
								aria-hidden="true"
							/>
							<PlugIcon
								v-else-if="route.name === 'discover-plugins' || route.path.startsWith('/plugin/')"
								aria-hidden="true"
							/>
							<ServerIcon
								v-else-if="route.name === 'discover-servers' || route.path.startsWith('/server/')"
								aria-hidden="true"
							/>
							<CompassIcon v-else aria-hidden="true" />
							<span class="hidden md:contents">{{
								formatMessage(navMenuMessages.discoverContent)
							}}</span>
							<span class="contents md:hidden">{{ formatMessage(navMenuMessages.discover) }}</span>
							<DropdownIcon aria-hidden="true" class="h-5 w-5" />

							<template #mods>
								<BoxIcon aria-hidden="true" />
								{{ formatMessage(commonProjectTypeCategoryMessages.mod) }}
							</template>
							<template #resourcepacks>
								<PaintbrushIcon aria-hidden="true" />
								{{ formatMessage(commonProjectTypeCategoryMessages.resourcepack) }}
							</template>
							<template #datapacks>
								<BracesIcon aria-hidden="true" />
								{{ formatMessage(commonProjectTypeCategoryMessages.datapack) }}
							</template>
							<template #plugins>
								<PlugIcon aria-hidden="true" />
								{{ formatMessage(commonProjectTypeCategoryMessages.plugin) }}
							</template>
							<template #shaders>
								<GlassesIcon aria-hidden="true" />
								{{ formatMessage(commonProjectTypeCategoryMessages.shader) }}
							</template>
							<template #modpacks>
								<PackageOpenIcon aria-hidden="true" />
								{{ formatMessage(commonProjectTypeCategoryMessages.modpack) }}
							</template>
							<template #servers>
								<ServerIcon aria-hidden="true" />
								{{ formatMessage(commonProjectTypeCategoryMessages.server) }}
							</template>
						</TeleportOverflowMenu>
					</ButtonStyled>
					<ButtonStyled
						v-if="FEATURES.serverHosting"
						type="transparent"
						:highlighted="
							route.name?.startsWith('hosting') ||
							(route.name?.startsWith('discover-') && !!route.query.sid)
						"
						:highlighted-style="
							route.name === 'hosting' ? 'main-nav-primary' : 'main-nav-secondary'
						"
					>
						<nuxt-link to="/hosting">
							<ServerIcon aria-hidden="true" />
							{{ formatMessage(navMenuMessages.hostAServer) }}
						</nuxt-link>
					</ButtonStyled>
					<ButtonStyled type="transparent" :highlighted="route.name === 'app'">
						<nuxt-link to="/app">
							<DownloadIcon aria-hidden="true" />
							<span class="hidden md:contents">{{
								formatMessage(navMenuMessages.getModrinthApp)
							}}</span>
							<span class="contents md:hidden">{{
								formatMessage(navMenuMessages.modrinthApp)
							}}</span>
						</nuxt-link>
					</ButtonStyled>
				</template>
			</div>
			<div class="flex items-center gap-1">
				<ButtonStyled type="transparent">
					<OverflowMenu
						v-if="auth.user && isStaff(auth.user)"
						class="btn-dropdown-animation flex items-center gap-1 rounded-xl bg-transparent px-2 py-1"
						position="bottom"
						direction="left"
						:dropdown-id="`${basePopoutId}-staff`"
						:aria-label="formatMessage(messages.createNew)"
						:options="[
							{
								id: 'review-projects',
								color: 'orange',
								link: '/moderation/',
							},
							{
								id: 'tech-review',
								color: 'orange',
								link: '/moderation/technical-review',
							},
							{
								id: 'review-reports',
								color: 'orange',
								link: '/moderation/reports',
							},
							{
								divider: true,
							},
							{
								id: 'file-lookup',
								link: '/admin/file_lookup',
							},
							{
								divider: true,
								shown: isAdmin(auth.user),
							},
							{
								id: 'user-lookup',
								color: 'primary',
								link: '/admin/user_email',
								shown: isAdmin(auth.user),
							},
							{
								id: 'affiliates',
								color: 'primary',
								link: '/admin/affiliates',
								shown: isAdmin(auth.user),
							},
							{
								id: 'servers-notices',
								color: 'primary',
								link: '/admin/servers/notices',
								shown: isAdmin(auth.user),
							},
							{
								id: 'servers-nodes',
								color: 'primary',
								action: (event) => $refs.modal_batch_credit.show(event),
								shown: isAdmin(auth.user),
							},
						]"
					>
						<ModrinthIcon aria-hidden="true" />
						<DropdownIcon aria-hidden="true" class="h-5 w-5 text-secondary" />
						<template #review-projects>
							<ScaleIcon aria-hidden="true" /> {{ formatMessage(messages.reviewProjects) }}
						</template>
						<template #tech-review>
							<ShieldAlertIcon aria-hidden="true" /> {{ formatMessage(messages.techReview) }}
						</template>
						<template #review-reports>
							<ReportIcon aria-hidden="true" /> {{ formatMessage(messages.reports) }}
						</template>
						<template #user-lookup>
							<UserSearchIcon aria-hidden="true" /> {{ formatMessage(messages.lookupByEmail) }}
						</template>
						<template #file-lookup>
							<FileIcon aria-hidden="true" /> {{ formatMessage(messages.fileLookup) }}
						</template>
						<template #servers-notices>
							<IssuesIcon aria-hidden="true" /> {{ formatMessage(messages.manageServerNotices) }}
						</template>
						<template #affiliates>
							<AffiliateIcon aria-hidden="true" /> {{ formatMessage(messages.manageAffiliates) }}
						</template>
						<template #servers-nodes>
							<ServerIcon aria-hidden="true" /> Credit server nodes
						</template>
					</OverflowMenu>
				</ButtonStyled>
				<ButtonStyled type="transparent">
					<OverflowMenu
						v-if="auth.user"
						class="btn-dropdown-animation flex items-center gap-1 rounded-xl bg-transparent px-2 py-1"
						position="bottom"
						direction="left"
						:dropdown-id="`${basePopoutId}-create`"
						:aria-label="formatMessage(messages.createNew)"
						:options="[
							{
								id: 'new-project',
								action: (event) => $refs.modal_creation.show(event),
							},
							{
								id: 'new-collection',
								action: (event) => $refs.modal_collection_creation.show(event),
							},
							{ divider: true },
							{
								id: 'new-organization',
								action: (event) => $refs.modal_organization_creation.show(event),
							},
						]"
					>
						<PlusIcon aria-hidden="true" />
						<DropdownIcon aria-hidden="true" class="h-5 w-5 text-secondary" />
						<template #new-project>
							<BoxIcon aria-hidden="true" /> {{ formatMessage(messages.newProject) }}
						</template>
						<!-- <template #import-project> <BoxImportIcon /> Import project </template>-->
						<template #new-collection>
							<CollectionIcon aria-hidden="true" /> {{ formatMessage(messages.newCollection) }}
						</template>
						<template #new-organization>
							<OrganizationIcon aria-hidden="true" /> {{ formatMessage(messages.newOrganization) }}
						</template>
					</OverflowMenu>
				</ButtonStyled>
				<OverflowMenu
					v-if="auth.user"
					:dropdown-id="`${basePopoutId}-user`"
					class="btn-dropdown-animation flex items-center gap-1 rounded-xl bg-transparent px-2 py-1"
					:options="userMenuOptions"
				>
					<Avatar :src="auth.user.avatar_url" aria-hidden="true" circle />
					<DropdownIcon class="h-5 w-5 text-secondary" />
					<template #profile>
						<UserIcon aria-hidden="true" /> {{ formatMessage(messages.profile) }}
					</template>
					<template #notifications>
						<BellIcon aria-hidden="true" /> {{ formatMessage(commonMessages.notificationsLabel) }}
					</template>
					<template #reports>
						<ReportIcon aria-hidden="true" /> {{ formatMessage(messages.activeReports) }}
					</template>
					<template #saved>
						<LibraryIcon aria-hidden="true" /> {{ formatMessage(commonMessages.collectionsLabel) }}
					</template>
					<template #servers>
						<ServerIcon aria-hidden="true" /> {{ formatMessage(messages.myServers) }}
					</template>
					<template #plus>
						<ArrowBigUpDashIcon aria-hidden="true" />
						{{ formatMessage(messages.upgradeToKweebexPlus) }}
					</template>
					<template #settings>
						<SettingsIcon aria-hidden="true" /> {{ formatMessage(commonMessages.settingsLabel) }}
					</template>
					<template #flags>
						<ReportIcon aria-hidden="true" /> {{ formatMessage(messages.featureFlags) }}
					</template>
					<template #projects>
						<BoxIcon aria-hidden="true" /> {{ formatMessage(messages.projects) }}
					</template>
					<template #organizations>
						<OrganizationIcon aria-hidden="true" /> {{ formatMessage(messages.organizations) }}
					</template>
					<template #affiliate-links>
						<AffiliateIcon aria-hidden="true" />
						{{ formatMessage(commonMessages.affiliateLinksButton) }}
					</template>
					<template #revenue>
						<CurrencyIcon aria-hidden="true" /> {{ formatMessage(messages.revenue) }}
					</template>
					<template #analytics>
						<ChartIcon aria-hidden="true" /> {{ formatMessage(messages.analytics) }}
					</template>
					<template #moderation>
						<ScaleIcon aria-hidden="true" /> {{ formatMessage(commonMessages.moderationLabel) }}
					</template>
					<template #sign-out>
						<LogOutIcon aria-hidden="true" /> {{ formatMessage(commonMessages.signOutButton) }}
					</template>
				</OverflowMenu>
				<template v-else>
					<ButtonStyled color="brand">
						<nuxt-link to="/auth/sign-in">
							<LogInIcon aria-hidden="true" />
							{{ formatMessage(commonMessages.signInButton) }}
						</nuxt-link>
					</ButtonStyled>
					<ButtonStyled circular>
						<nuxt-link :v-tooltip="formatMessage(commonMessages.settingsLabel)" to="/settings">
							<SettingsIcon :aria-label="formatMessage(commonMessages.settingsLabel)" />
						</nuxt-link>
					</ButtonStyled>
				</template>
			</div>
			</header>
		</div>
		<header class="mobile-navigation mobile-only">
			<div
				class="nav-menu nav-menu-browse"
				:class="{ expanded: isBrowseMenuOpen }"
				@focusin="isBrowseMenuOpen = true"
				@focusout="isBrowseMenuOpen = false"
			>
				<div class="links cascade-links">
					<NuxtLink
						v-for="navRoute in navRoutes"
						:key="navRoute.href"
						:to="navRoute.href"
						class="iconified-button"
					>
						{{ navRoute.label }}
					</NuxtLink>
				</div>
			</div>
			<div
				class="nav-menu nav-menu-mobile"
				:class="{ expanded: isMobileMenuOpen }"
				@focusin="isMobileMenuOpen = true"
				@focusout="isMobileMenuOpen = false"
			>
				<div class="account-container">
					<NuxtLink
						v-if="auth.user"
						:to="`/user/${auth.user.username}`"
						class="iconified-button account-button"
					>
						<Avatar
							:src="auth.user.avatar_url"
							class="user-icon"
							:alt="formatMessage(messages.yourAvatarAlt)"
							aria-hidden="true"
							circle
						/>
						<div class="account-text">
							<div>@{{ auth.user.username }}</div>
							<div>{{ formatMessage(commonMessages.visitYourProfile) }}</div>
						</div>
					</NuxtLink>
					<nuxt-link v-else class="iconified-button brand-button" to="/auth/sign-in">
						<LogInIcon aria-hidden="true" /> {{ formatMessage(commonMessages.signInButton) }}
					</nuxt-link>
				</div>
				<div class="links">
					<template v-if="auth.user">
						<button class="iconified-button danger-button" @click="logoutUser()">
							<LogOutIcon aria-hidden="true" />
							{{ formatMessage(commonMessages.signOutButton) }}
						</button>
						<button class="iconified-button" @click="$refs.modal_creation.show()">
							<PlusIcon aria-hidden="true" />
							{{ formatMessage(commonMessages.createAProjectButton) }}
						</button>
						<NuxtLink class="iconified-button" to="/dashboard/collections">
							<LibraryIcon class="icon" />
							{{ formatMessage(commonMessages.collectionsLabel) }}
						</NuxtLink>
						<NuxtLink v-if="FEATURES.serverHosting" class="iconified-button" to="/hosting/manage">
							<ServerIcon class="icon" />
							{{ formatMessage(commonMessages.serversLabel) }}
						</NuxtLink>
						<NuxtLink
							v-if="auth.user.role === 'moderator' || auth.user.role === 'admin'"
							class="iconified-button"
							to="/moderation"
						>
							<ScaleIcon aria-hidden="true" />
							{{ formatMessage(commonMessages.moderationLabel) }}
						</NuxtLink>
						<NuxtLink v-if="flags.developerMode" class="iconified-button" to="/flags">
							<ReportIcon aria-hidden="true" />
							{{ formatMessage(messages.featureFlags) }}
						</NuxtLink>
					</template>
					<NuxtLink class="iconified-button" to="/settings">
						<SettingsIcon aria-hidden="true" />
						{{ formatMessage(commonMessages.settingsLabel) }}
					</NuxtLink>
					<button class="iconified-button" @click="changeTheme">
						<MoonIcon v-if="$theme.active === 'light'" class="icon" />
						<SunIcon v-else class="icon" />
						<span class="dropdown-item__text">
							{{ formatMessage(messages.changeTheme) }}
						</span>
					</button>
				</div>
			</div>
			<div class="mobile-navbar" :class="{ expanded: isBrowseMenuOpen || isMobileMenuOpen }">
				<NuxtLink
					to="/"
					class="tab button-animation"
					:title="formatMessage(navMenuMessages.home)"
					:aria-label="formatMessage(navMenuMessages.home)"
				>
					<HomeIcon aria-hidden="true" />
				</NuxtLink>
				<button
					class="tab button-animation"
					:class="{ 'router-link-exact-active': isBrowseMenuOpen }"
					:title="formatMessage(navMenuMessages.search)"
					:aria-label="formatMessage(navMenuMessages.search)"
					@click="toggleBrowseMenu()"
				>
					<template v-if="auth.user">
						<SearchIcon aria-hidden="true" />
					</template>
					<template v-else>
						<SearchIcon aria-hidden="true" class="smaller" />
						{{ formatMessage(navMenuMessages.search) }}
					</template>
				</button>
				<template v-if="auth.user">
					<NuxtLink
						to="/dashboard/notifications"
						class="tab button-animation"
						:aria-label="formatMessage(commonMessages.notificationsLabel)"
						:class="{
							'no-active': isMobileMenuOpen || isBrowseMenuOpen,
						}"
						:title="formatMessage(commonMessages.notificationsLabel)"
						@click="
							() => {
								isMobileMenuOpen = false
								isBrowseMenuOpen = false
							}
						"
					>
						<BellIcon aria-hidden="true" />
					</NuxtLink>
					<NuxtLink
						to="/dashboard"
						class="tab button-animation"
						:aria-label="formatMessage(commonMessages.dashboardLabel)"
						:title="formatMessage(commonMessages.dashboardLabel)"
					>
						<ChartIcon aria-hidden="true" />
					</NuxtLink>
				</template>
				<button
					class="tab button-animation"
					:title="formatMessage(messages.toggleMenu)"
					:aria-label="
						isMobileMenuOpen ? formatMessage(messages.closeMenu) : formatMessage(messages.openMenu)
					"
					@click="toggleMobileMenu()"
				>
					<template v-if="!auth.user">
						<HamburgerIcon v-if="!isMobileMenuOpen" aria-hidden="true" />
						<XIcon v-else aria-hidden="true" />
					</template>
					<template v-else>
						<Avatar
							:src="auth.user.avatar_url"
							class="user-icon"
							:class="{ expanded: isMobileMenuOpen }"
							:alt="formatMessage(messages.yourAvatarAlt)"
							aria-hidden="true"
							circle
						/>
					</template>
				</button>
			</div>
		</header>
		<main class="min-h-[calc(100vh-4.5rem-310.59px)]">
			<ProjectCreateModal v-if="auth.user" ref="modal_creation" />
			<CollectionCreateModal ref="modal_collection_creation" />
			<OrganizationCreateModal ref="modal_organization_creation" />
			<BatchCreditModal v-if="auth.user && isAdmin(auth.user)" ref="modal_batch_credit" />
			<slot id="main" />
		</main>
		<KweebexFooter />
	</div>
</template>
<script setup>
import {
	AffiliateIcon,
	ArrowBigUpDashIcon,
	BellIcon,
	BoxIcon,
	BracesIcon,
	ChartIcon,
	CollectionIcon,
	CompassIcon,
	CurrencyIcon,
	DownloadIcon,
	DropdownIcon,
	FileIcon,
	GlassesIcon,
	HamburgerIcon,
	HomeIcon,
	IssuesIcon,
	LibraryIcon,
	LogInIcon,
	LogOutIcon,
	ModrinthIcon,
	MoonIcon,
	OrganizationIcon,
	PackageOpenIcon,
	PaintbrushIcon,
	PlugIcon,
	PlusIcon,
	ReportIcon,
	ScaleIcon,
	SearchIcon,
	ServerIcon,
	SettingsIcon,
	ShieldAlertIcon,
	SunIcon,
	UserIcon,
	UserSearchIcon,
	XIcon,
} from '@kweebex/assets'
import {
	Avatar,
	ButtonStyled,
	commonMessages,
	commonProjectTypeCategoryMessages,
	defineMessages,
	OverflowMenu,
	useVIntl,
} from '@kweebex/ui'
import { isAdmin, isStaff, UserBadge } from '@kweebex/utils'
import { FEATURES } from '../../../../config'

import TextLogo from '~/components/brand/TextLogo.vue'
import BatchCreditModal from '~/components/ui/admin/BatchCreditModal.vue'
import GeneratedStateErrorsBanner from '~/components/ui/banner/GeneratedStateErrorsBanner.vue'
import RussiaBanner from '~/components/ui/banner/RussiaBanner.vue'
import StagingBanner from '~/components/ui/banner/StagingBanner.vue'
import SubscriptionPaymentFailedBanner from '~/components/ui/banner/SubscriptionPaymentFailedBanner.vue'
import TaxComplianceBanner from '~/components/ui/banner/TaxComplianceBanner.vue'
import TaxIdMismatchBanner from '~/components/ui/banner/TaxIdMismatchBanner.vue'
import VerifyEmailBanner from '~/components/ui/banner/VerifyEmailBanner.vue'
import CollectionCreateModal from '~/components/ui/create/CollectionCreateModal.vue'
import OrganizationCreateModal from '~/components/ui/create/OrganizationCreateModal.vue'
import ProjectCreateModal from '~/components/ui/create/ProjectCreateModal.vue'
import KweebexFooter from '~/components/ui/KweebexFooter.vue'
import TeleportOverflowMenu from '~/components/ui/servers/TeleportOverflowMenu.vue'
import { errors as generatedStateErrors } from '~/generated/state.json'
import { getProjectTypeMessage } from '~/utils/i18n-project-type.ts'

const country = useUserCountry()

const { formatMessage } = useVIntl()

const auth = await useAuth()
const user = await useUser()

const cosmetics = useCosmetics()
const flags = useFeatureFlags()

const config = useRuntimeConfig()
const route = useNativeRoute()
const router = useNativeRouter()
const link = config.public.siteUrl + route.path.replace(/\/+$/, '')

const { data: payoutBalance } = await useAsyncData('payout/balance', () =>
	useBaseFetch('payout/balance', { apiVersion: 3 }),
)

const showTaxComplianceBanner = computed(() => {
	if (flags.value.testTaxForm && auth.value.user) return true
	const bal = payoutBalance.value
	if (!bal) return false
	const thresholdMet = (bal.withdrawn_ytd ?? 0) >= 600
	const status = bal.form_completion_status ?? 'unknown'
	const isComplete = status === 'complete'
	const isTinMismatch = status === 'tin-mismatch'
	return !!auth.value.user && thresholdMet && !isComplete && !isTinMismatch
})

const showTinMismatchBanner = computed(() => {
	const bal = payoutBalance.value
	if (!bal) return false
	const status = bal.form_completion_status ?? 'unknown'
	return !!auth.value.user && status === 'tin-mismatch'
})

const basePopoutId = useId()

const navMenuMessages = defineMessages({
	home: {
		id: 'layout.nav.home',
		defaultMessage: 'Home',
	},
	search: {
		id: 'layout.nav.search',
		defaultMessage: 'Search',
	},
	discoverContent: {
		id: 'layout.nav.discover-content',
		defaultMessage: 'Discover content',
	},
	discover: {
		id: 'layout.nav.discover',
		defaultMessage: 'Discover',
	},
	hostAServer: {
		id: 'layout.nav.host-a-server',
		defaultMessage: 'Host a server',
	},
	getModrinthApp: {
		id: 'layout.nav.get-kweebex-app',
		defaultMessage: 'Get Kweebex App',
	},
	modrinthApp: {
		id: 'layout.nav.kweebex-app',
		defaultMessage: 'Kweebex App',
	},
})

const messages = defineMessages({
	toggleMenu: {
		id: 'layout.menu-toggle.action',
		defaultMessage: 'Toggle menu',
	},
	yourAvatarAlt: {
		id: 'layout.avatar.alt',
		defaultMessage: 'Your avatar',
	},
	changeTheme: {
		id: 'layout.action.change-theme',
		defaultMessage: 'Change theme',
	},
	kweebexHomePage: {
		id: 'layout.nav.kweebex-home-page',
		defaultMessage: 'Kweebex home page',
	},
	createNew: {
		id: 'layout.action.create-new',
		defaultMessage: 'Create new...',
	},
	reviewProjects: {
		id: 'layout.action.review-projects',
		defaultMessage: 'Project review',
	},
	techReview: {
		id: 'layout.action.tech-review',
		defaultMessage: 'Tech review',
	},
	reports: {
		id: 'layout.action.reports',
		defaultMessage: 'Review reports',
	},
	lookupByEmail: {
		id: 'layout.action.lookup-by-email',
		defaultMessage: 'Lookup by email',
	},
	fileLookup: {
		id: 'layout.action.file-lookup',
		defaultMessage: 'File lookup',
	},
	manageServerNotices: {
		id: 'layout.action.manage-server-notices',
		defaultMessage: 'Manage server notices',
	},
	manageAffiliates: {
		id: 'layout.action.manage-affiliates',
		defaultMessage: 'Manage affiliate links',
	},
	newProject: {
		id: 'layout.action.new-project',
		defaultMessage: 'New project',
	},
	newCollection: {
		id: 'layout.action.new-collection',
		defaultMessage: 'New collection',
	},
	newOrganization: {
		id: 'layout.action.new-organization',
		defaultMessage: 'New organization',
	},
	profile: {
		id: 'layout.nav.profile',
		defaultMessage: 'Profile',
	},
	savedProjects: {
		id: 'layout.nav.saved-projects',
		defaultMessage: 'Saved projects',
	},
	upgradeToKweebexPlus: {
		id: 'layout.nav.upgrade-to-kweebex-plus',
		defaultMessage: 'Upgrade to Kweebex+',
	},
	featureFlags: {
		id: 'layout.nav.feature-flags',
		defaultMessage: 'Feature flags',
	},
	projects: {
		id: 'layout.nav.projects',
		defaultMessage: 'Projects',
	},
	organizations: {
		id: 'layout.nav.organizations',
		defaultMessage: 'Organizations',
	},
	revenue: {
		id: 'layout.nav.revenue',
		defaultMessage: 'Revenue',
	},
	analytics: {
		id: 'layout.nav.analytics',
		defaultMessage: 'Analytics',
	},
	activeReports: {
		id: 'layout.nav.active-reports',
		defaultMessage: 'Active reports',
	},
	myServers: {
		id: 'layout.nav.my-servers',
		defaultMessage: 'My servers',
	},
	openMenu: {
		id: 'layout.mobile.open-menu',
		defaultMessage: 'Open menu',
	},
	closeMenu: {
		id: 'layout.mobile.close-menu',
		defaultMessage: 'Close menu',
	},
})

useHead({
	link: [
		{
			rel: 'canonical',
			href: link,
		},
	],
})
useSeoMeta({
	title: 'Kweebex',
	description: () =>
		formatMessage({
			id: 'layout.meta.description',
			defaultMessage:
				'Download Hytale mods and modpacks on Kweebex. ' +
				'Discover and publish projects on Kweebex with a modern, easy to use interface and API.',
		}),
	publisher: 'Kweebex',
	themeColor: '#3d7ea6',
	colorScheme: 'dark light',

	// OpenGraph
	ogTitle: 'Kweebex',
	ogSiteName: 'Kweebex',
	ogDescription: () =>
		formatMessage({
			id: 'layout.meta.og-description',
			defaultMessage: 'Discover and publish Hytale content!',
		}),
	ogType: 'website',
	ogImage: '/branding/logo-square.png',
	ogUrl: link,

	// Twitter
	twitterCard: 'summary',
	twitterSite: '@kweebex',
})

const isMobileMenuOpen = ref(false)
const isBrowseMenuOpen = ref(false)
const navRoutes = computed(() => [
	FEATURES.mods && {
		id: 'mods',
		label: formatMessage(getProjectTypeMessage('mod', true)),
		href: '/discover/mods',
	},
	FEATURES.modpacks && {
		label: formatMessage(getProjectTypeMessage('modpack', true)),
		href: '/discover/modpacks',
	},
	FEATURES.plugins && {
		label: formatMessage(getProjectTypeMessage('plugin', true)),
		href: '/discover/plugins',
	},
	FEATURES.datapacks && {
		label: formatMessage(getProjectTypeMessage('datapack', true)),
		href: '/discover/datapacks',
	},
	FEATURES.shaders && {
		label: formatMessage(getProjectTypeMessage('shader', true)),
		href: '/discover/shaders',
	},
	FEATURES.resourcepacks && {
		label: formatMessage(getProjectTypeMessage('resourcepack', true)),
		href: '/discover/resourcepacks',
	},
].filter(Boolean))

const userMenuOptions = computed(() => {
	let options = [
		{
			id: 'profile',
			link: `/user/${auth.value.user.username}`,
		},
		{
			id: 'plus',
			link: '/plus',
			color: 'purple',
			shown: !flags.value.hidePlusPromoInUserMenu && !isPermission(auth.value.user.badges, 1 << 0),
		},
		{
			id: 'servers',
			link: '/hosting/manage',
			shown: FEATURES.serverHosting,
		},
		{
			id: 'flags',
			link: '/flags',
			shown: flags.value.developerMode,
		},
		{
			id: 'settings',
			link: '/settings',
		},
	]

	// TODO: Only show if user has projects
	options = [
		...options,
		{
			divider: true,
		},
		{
			id: 'notifications',
			link: '/dashboard/notifications',
		},
		{
			id: 'reports',
			link: '/dashboard/reports',
		},
		{
			id: 'saved',
			link: '/dashboard/collections',
		},
		{
			divider: true,
		},
		{
			id: 'projects',
			link: '/dashboard/projects',
		},
		{
			id: 'organizations',
			link: '/dashboard/organizations',
		},
		{
			id: 'analytics',
			link: '/dashboard/analytics',
		},
		{
			id: 'affiliate-links',
			link: '/dashboard/affiliate-links',
			shown: auth.value.user.badges & UserBadge.AFFILIATE,
		},
		{
			id: 'revenue',
			link: '/dashboard/revenue',
		},
	]

	options = [
		...options,
		{
			divider: true,
		},
		{
			id: 'sign-out',
			color: 'danger',
			action: () => logoutUser(),
			hoverFilled: true,
		},
	]
	return options
})

const isDiscovering = computed(
	() => route.name && route.name.startsWith('discover-') && !route.query.sid,
)

const isDiscoveringSubpage = computed(
	() => route.name && route.name.startsWith('type-id') && !route.query.sid,
)

const isRussia = computed(() => country.value === 'ru')

const rCount = ref(0)

const randomProjects = ref([])
const disableRandomProjects = ref(false)

const disableRandomProjectsForRoute = computed(
	() =>
		route.name.startsWith('hosting') ||
		route.name.includes('settings') ||
		route.name.includes('admin'),
)

async function onKeyDown(event) {
	if (disableRandomProjects.value || disableRandomProjectsForRoute.value) {
		return
	}

	if (event.key === 'r') {
		rCount.value++

		if (randomProjects.value.length < 3) {
			randomProjects.value = await useBaseFetch('projects_random?count=50').catch((err) => {
				console.error(err)
				return []
			})
		}
	}

	if (rCount.value >= 40) {
		rCount.value = 0
		const randomProject = randomProjects.value[0]
		await router.push(`/project/${randomProject.slug}`)
		randomProjects.value.splice(0, 1)
	}
}

function onKeyUp(event) {
	if (event.key === 'r') {
		rCount.value = 0
	}
}

onMounted(() => {
	if (window && import.meta.client) {
		window.history.scrollRestoration = 'auto'
	}

	runAnalytics()

	window.addEventListener('keydown', onKeyDown)
	window.addEventListener('keyup', onKeyUp)
})

watch(
	() => route.path,
	() => {
		isMobileMenuOpen.value = false
		isBrowseMenuOpen.value = false

		if (import.meta.client) {
			document.body.style.overflowY = 'scroll'
			document.body.setAttribute('tabindex', '-1')
			document.body.removeAttribute('tabindex')
		}

		runAnalytics()
	},
)

async function logoutUser() {
	await logout()
}

function runAnalytics() {
	const config = useRuntimeConfig()
	const replacedUrl = config.public.apiBaseUrl.replace('v2/', '')

	try {
		setTimeout(() => {
			$fetch(`${replacedUrl}analytics/view`, {
				method: 'POST',
				body: {
					url: window.location.href,
				},
				headers: {
					Authorization: auth.value.token,
				},
			})
				.then(() => {})
				.catch(() => {})
		})
	} catch (e) {
		console.error(`Sending analytics failed (CORS error? If so, ignore)`, e)
	}
}
function toggleMobileMenu() {
	isMobileMenuOpen.value = !isMobileMenuOpen.value
	if (isMobileMenuOpen.value) {
		isBrowseMenuOpen.value = false
	}
}
function toggleBrowseMenu() {
	isBrowseMenuOpen.value = !isBrowseMenuOpen.value

	if (isBrowseMenuOpen.value) {
		isMobileMenuOpen.value = false
	}
}

const { cycle: changeTheme } = useTheme()
</script>

<style lang="scss">
@import '~/assets/styles/global.scss';
// @import '@kweebex/assets';

// Header wrapper - full width background that matches the page
.header-wrapper {
	width: 100%;
	background: var(--color-bg);
	
	// Landing page - match hero gradient
	&.header-wrapper--landing {
		background: linear-gradient(135deg, #0d1829 0%, #15243a 40%, #1e3a5f 100%);
	}
}

// Kweebex Header Styling - Hytale inspired fantasy UI panel
// Double border effect with inner golden corner brackets
.kweebex-header {
	position: relative;
	// Navy gradient background - slightly lighter in middle
	background: linear-gradient(180deg, 
		#15243a 0%, 
		#1a2f4a 30%,
		#1a2f4a 70%,
		#15243a 100%);
	backdrop-filter: blur(12px);
	-webkit-backdrop-filter: blur(12px);
	// Dark outer border
	border: 2px solid #0a1525;
	border-top: none;
	border-radius: 0 0 12px 12px;
	// Double border effect + shadows
	box-shadow: 
		// Outer shadow for depth
		0 4px 20px rgba(0, 0, 0, 0.5),
		0 8px 32px rgba(0, 0, 0, 0.3),
		// Inner teal border (the "light inner" of double border)
		inset 0 0 0 2px rgba(61, 126, 166, 0.4),
		// Subtle bottom highlight (adjusted for flush top)
		inset 0 -1px 0 rgba(91, 163, 204, 0.2),
		// Inner ambient glow
		inset 0 0 20px rgba(61, 126, 166, 0.08);
	transition: all var(--transition-normal);
	overflow: visible;
	
	// Bottom edge glow highlight (moved from top since header is now flush)
	&::before {
		content: '';
		position: absolute;
		bottom: 2px;
		left: 20px;
		right: 20px;
		height: 1px;
		background: linear-gradient(90deg,
			transparent 0%,
			rgba(91, 163, 204, 0.4) 20%,
			rgba(91, 163, 204, 0.5) 50%,
			rgba(91, 163, 204, 0.4) 80%,
			transparent 100%);
		pointer-events: none;
		z-index: 5;
	}

	// This pseudo is reserved for header content overlay if needed
	&::after {
		content: '';
		position: absolute;
		inset: 0;
		border-radius: inherit;
		pointer-events: none;
		z-index: 0;
	}
}

// Hidden corner elements (not used)
.kweebex-header-ornaments,
.kweebex-header-ornament-br {
	display: none;
}

// Dark mode - same styling (already dark themed)
.dark-mode .kweebex-header,
.dark .kweebex-header {
	background: linear-gradient(180deg, 
		#0d1829 0%, 
		#15243a 30%,
		#15243a 70%,
		#0d1829 100%);
}

.layout {
	min-height: 100vh;
	display: block;

	@media screen and (min-width: 1024px) {
		min-height: calc(100vh - var(--spacing-card-bg));
	}

	main {
		grid-area: main;
		padding-top: 1.5rem;
	}
}

@media (min-width: 1024px) {
	.layout {
		main {
			.alpha-alert {
				margin: 1rem;

				.wrapper {
					padding: 1rem 2rem 1rem 1rem;
				}
			}
		}
	}
}

@media (max-width: 1200px) {
	.app-btn {
		display: none;
	}
}

.mobile-navigation {
	display: none;

	.nav-menu {
		width: 100%;
		position: fixed;
		bottom: calc(var(--size-mobile-navbar-height) - var(--size-rounded-card));
		padding-bottom: var(--size-rounded-card);
		left: 0;
		background: var(--glass-bg);
		backdrop-filter: blur(var(--glass-blur));
		-webkit-backdrop-filter: blur(var(--glass-blur));
		z-index: 11; // 20 = modals, 10 = svg icons
		transform: translateY(100%);
		transition: transform 0.4s cubic-bezier(0.54, 0.84, 0.42, 1), box-shadow var(--transition-normal);
		border-radius: var(--size-rounded-card) var(--size-rounded-card) 0 0;
		box-shadow: 0 0 20px 2px rgba(0, 0, 0, 0);
		border-top: 1px solid var(--glass-border);

		.links,
		.account-container {
			display: grid;
			grid-template-columns: repeat(1, 1fr);
			grid-gap: 1rem;
			justify-content: center;
			padding: 1rem;

			.iconified-button {
				width: 100%;
				max-width: 500px;
				padding: 0.75rem;
				justify-content: center;
				font-weight: 600;
				font-size: 1rem;
				margin: 0 auto;
			}
		}

		.cascade-links {
			@media screen and (min-width: 354px) {
				grid-template-columns: repeat(2, 1fr);
			}

			@media screen and (min-width: 674px) {
				grid-template-columns: repeat(3, 1fr);
			}
		}

		&-browse {
			&.expanded {
				transform: translateY(0);
				box-shadow: 0 0 20px 2px rgba(0, 0, 0, 0.3);
			}
		}

		&-mobile {
			.account-container {
				padding-bottom: 0;

				.account-button {
					padding: var(--spacing-card-md);
					display: flex;
					align-items: center;
					justify-content: center;
					gap: 0.5rem;

					.user-icon {
						width: 2.25rem;
						height: 2.25rem;
					}

					.account-text {
						flex-grow: 0;
					}
				}
			}

			&.expanded {
				transform: translateY(0);
				box-shadow: 0 0 20px 2px rgba(0, 0, 0, 0.3);
			}
		}
	}

	.mobile-navbar {
		display: flex;
		height: calc(var(--size-mobile-navbar-height) + env(safe-area-inset-bottom));
		border-radius: var(--size-rounded-card) var(--size-rounded-card) 0 0;
		padding-bottom: env(safe-area-inset-bottom);
		position: fixed;
		left: 0;
		bottom: 0;
		background: var(--glass-bg);
		backdrop-filter: blur(var(--glass-blur));
		-webkit-backdrop-filter: blur(var(--glass-blur));
		box-shadow: 0 0 20px 2px rgba(0, 0, 0, 0.3);
		z-index: 11; // 20 = modals, 10 = svg icons
		width: 100%;
		align-items: center;
		justify-content: space-between;
		transition: border-radius 0.3s ease-out, background var(--transition-normal);
		border-top: 1px solid var(--glass-border);
		box-sizing: border-box;

		&.expanded {
			box-shadow: none;
			border-radius: 0;
		}

		.tab {
			position: relative;
			background: none;
			display: flex;
			flex-basis: 0;
			justify-content: center;
			align-items: center;
			flex-direction: row;
			gap: 0.25rem;
			font-weight: bold;
			padding: 0;
			transition: color ease-in-out 0.15s;
			color: var(--color-text-inactive);
			text-align: center;

			&.browse {
				svg {
					transform: rotate(180deg);
					transition: transform ease-in-out 0.3s;

					&.closed {
						transform: rotate(0deg);
					}
				}
			}

			&.bubble {
				&::after {
					background-color: var(--color-brand);
					border-radius: var(--size-rounded-max);
					content: '';
					height: 0.5rem;
					position: absolute;
					left: 1.5rem;
					top: 0;
					width: 0.5rem;
				}
			}

			svg {
				height: 1.75rem;
				width: 1.75rem;

				&.smaller {
					width: 1.25rem;
					height: 1.25rem;
				}
			}

			.user-icon {
				width: 2rem;
				height: 2rem;
				transition: border ease-in-out 0.15s;
				border: 0 solid var(--color-brand);
				box-sizing: border-box;

				&.expanded {
					border: 2px solid var(--color-brand);
				}
			}

			&:hover,
			&:focus {
				color: var(--color-text);
			}

			&:first-child {
				margin-left: 2rem;
			}

			&:last-child {
				margin-right: 2rem;
			}

			&.router-link-exact-active:not(&.no-active) {
				svg {
					color: var(--color-brand);
				}

				color: var(--color-brand);
			}
		}
	}
}

@media (any-hover: none) and (max-width: 640px) {
	.desktop-only {
		display: none;
	}
}

@media (any-hover: none) and (max-width: 640px) {
	.mobile-navigation {
		display: flex;
	}
}

.over-the-top-random-animation {
	position: fixed;
	z-index: 100;
	inset: 0;
	display: flex;
	justify-content: center;
	align-items: center;
	pointer-events: none;
	scale: 0.5;
	transition: all 0.5s ease-out;
	opacity: 0;
	animation:
		tilt-shaking calc(0.2s / (max((var(--_r-count) - 20), 1) / 20)) linear infinite,
		translate-x-shaking calc(0.3s / (max((var(--_r-count) - 20), 1) / 20)) linear infinite,
		translate-y-shaking calc(0.25s / (max((var(--_r-count) - 20), 1) / 20)) linear infinite;

	&.threshold {
		opacity: 1;
	}

	&.rings-expand {
		scale: 0.8;
		opacity: 0;

		.animation-ring-1 {
			width: 25rem;
			height: 25rem;
		}

		.animation-ring-2 {
			width: 50rem;
			height: 50rem;
		}

		.animation-ring-3 {
			width: 100rem;
			height: 100rem;
		}
	}

	> div {
		position: relative;
		display: flex;
		justify-content: center;
		align-items: center;
		width: fit-content;
		height: fit-content;

		> * {
			position: absolute;
			scale: calc(1 + max((var(--_r-count) - 20), 0) * 0.1);
			transition: all 0.2s ease-out;
			width: 20rem;
			height: 20rem;
		}
	}
}

@keyframes tilt-shaking {
	0% {
		rotate: 0deg;
	}

	25% {
		rotate: calc(1deg * (var(--_r-count) - 20));
	}

	50% {
		rotate: 0deg;
	}

	75% {
		rotate: calc(-1deg * (var(--_r-count) - 20));
	}

	100% {
		rotate: 0deg;
	}
}

@keyframes translate-x-shaking {
	0% {
		translate: 0;
	}

	25% {
		translate: calc(2px * (var(--_r-count) - 20));
	}

	50% {
		translate: 0;
	}

	75% {
		translate: calc(-2px * (var(--_r-count) - 20));
	}

	100% {
		translate: 0;
	}
}

@keyframes translate-y-shaking {
	0% {
		transform: translateY(0);
	}

	25% {
		transform: translateY(calc(2px * (var(--_r-count) - 20)));
	}

	50% {
		transform: translateY(0);
	}

	75% {
		transform: translateY(calc(-2px * (var(--_r-count) - 20)));
	}

	100% {
		transform: translateY(0);
	}
}
</style>
<style src="vue-multiselect/dist/vue-multiselect.css"></style>
