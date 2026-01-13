<template>
	<img
		:src="logoSrc"
		alt="Kweebex"
		:class="{ animate: loading }"
		draggable="false"
	/>
</template>

<script setup lang="ts">
const loading = useLoading()

const logoSrc = '/branding/logo-wide.png'
</script>

<style lang="scss" scoped>
img {
	height: 4rem;
	width: auto;
	object-fit: contain;
	margin-top: -0.75rem;
	margin-bottom: -0.75rem;
	user-select: none;
	-webkit-user-drag: none;
	pointer-events: auto;
}

.animate {
	animation: pulse 1.5s ease-in-out infinite;
}

@keyframes pulse {
	0%, 100% {
		opacity: 1;
	}
	50% {
		opacity: 0.6;
	}
}
</style>
