<template>
	<div class="markdown-body" v-html="renderHighlightedString(description ?? '')" />
</template>
<script setup lang="ts">
import { renderHighlightedString } from '@kweebex/utils'

withDefaults(
	defineProps<{
		description: string
	}>(),
	{},
)
</script>
