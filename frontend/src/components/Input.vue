<script setup lang="ts">
type Props = {
	variant?: "normal" | "danger";
	type?: "text" | "password" | "field";
};

withDefaults(defineProps<Props>(), {
	variant: "normal",
	type: "text",
});

const model = defineModel<string>();
</script>

<template>
	<input
		v-if="type === 'text' || type === 'password'"
		v-model="model"
		:type="type"
		:class="['input', `input--${variant}`]"
	/>

	<textarea
		v-else
		v-model="model"
		:class="['input', `input--${variant}`]"
	></textarea>
</template>

<style scoped>
.input--normal {
	--border-color: var(--color-surface-2);
}

.input--danger {
	--border-color: var(--color-danger);
}

input {
	height: 36px;
}

textarea {
	height: 68px;
}

input,
textarea {
	color: var(--color-text);
	font-size: 1em;
	font-family: inherit;
	width: 100%;
	display: block;
	box-sizing: border-box;
	border: var(--border-width) solid var(--border-color);
	border-radius: var(--radius-sm);
	background: var(--color-surface-1);
	outline: none;
	padding: 5px;
	filter: var(--shadow-sm);
	transition:
		border-color var(--transition-fast),
		filter var(--transition-slow);

	&:focus {
		border-color: var(--color-primary);
	}
}
</style>
