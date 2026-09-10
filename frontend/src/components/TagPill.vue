<script setup lang="ts">
import { normalizeTagName } from "@/tag";

defineProps<{
	name: string;
	color: Record<string, string>;
	applied: boolean;
}>();
</script>

<template>
	<div class="tags__pill" :class="{ applied: applied }" :style="color">
		<p>#{{ normalizeTagName(name) }}</p>
	</div>
</template>

<style scoped>
.tags__pill {
	transition:
		color var(--transition-fast),
		border-color var(--transition-fast),
		background-color var(--transition-fast);
	--tag-color-dim: color-mix(in srgb, var(--tag-color) 30%, var(--color-bg));
	border: 1px solid var(--color-surface-2);
	color: var(--color-text-subtle);
	background-color: var(--color-surface-1);
	border-radius: 99999px;
	font-size: 12px;
	display: flex;
	align-items: center;
	padding-inline: 6px;
	height: 20px;
	user-select: none;
	-webkit-user-select: none;
	-ms-user-select: none;

	&.applied {
		border: 1px solid var(--tag-color);
		color: var(--tag-color);
		background-color: var(--tag-color-dim);
	}
}

.tags__pill p {
	margin: 0;
}
</style>
