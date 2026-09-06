<script setup lang="ts">
type Props = {
	variant?: "primary" | "success" | "danger";
	type?: "button" | "submit" | "reset";
	to?: string | Record<string, unknown>;
};

const props = withDefaults(defineProps<Props>(), {
	type: "button",
});
</script>

<template>
	<RouterLink
		v-if="to !== undefined"
		:to="to"
		:class="['button', `button--${variant}`]"
	>
		<slot />
	</RouterLink>

	<button v-else :type="type" :class="['button', `button--${variant}`]">
		<slot />
	</button>
</template>

<style scoped>
.button--primary {
	--button-color: var(--color-primary);
}

.button--success {
	--button-color: var(--color-success);
}

.button--danger {
	--button-color: var(--color-danger);
}

.button {
	display: flex;
	justify-content: center;
	align-items: center;
	color: var(--color-text);
	text-decoration: none;
	border: 2px solid black;
	height: 35px;
	asepct-ratio: 1/1;
	cursor: pointer;
	pointer-events: auto;
	border-radius: var(--radius-sm);
	background: var(--color-surface-2);
	box-sizing: border-box;
	border: 2px solid color-mix(in srgb, var(--button-color) 70%, black);
	transition:
		background-color var(--transition-fast),
		border-color var(--transition-fast);

	&:hover {
		border-color: var(--button-color);
		background: color-mix(in srgb, var(--button-color) 70%, black);
	}
}
</style>
