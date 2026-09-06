<script setup lang="ts">
defineProps<{
	modelValue: boolean;
}>();

const emit = defineEmits<{
	"update:modelValue": [value: boolean];
}>();
</script>

<template>
	<div
		class="checkbox"
		:class="{ toggled: modelValue }"
		@click="emit('update:modelValue', !modelValue)"
	>
		<span>✓</span>
	</div>
</template>

<style scoped>
.checkbox {
	height: 35px;
	aspect-ratio: 1/1;
	box-sizing: border-box;
	border: var(--border-width) solid var(--color-surface-2);
	border-radius: var(--radius-sm);
	background: var(--color-surface-1);
	transition:
		background-color var(--transition-fast),
		border-color var(--transition-fast);
	display: flex;
	align-items: center;
	justify-content: center;
	pointer-events: auto;
	cursor: pointer;

	& > span {
		font-size: 1.5em;
		font-weight: bold;
		height: fit-content;
		width: fit-content;
		user-select: none;
		-webkit-user-select: none;
		-moz-user-select: none;
		-ms-user-select: none;
		opacity: 0;
		transition: opacity var(--transition-fast);
	}

	&.toggled {
		border-color: var(--color-success);
		background-color: color-mix(in srgb, var(--color-success) 70%, black);

		& > span {
			opacity: 1;
		}
	}
}
</style>
