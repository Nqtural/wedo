<script setup lang="ts">
import { ref } from "vue";

type Props = {
	link: string;
	hidden?: boolean;
};

const props = withDefaults(defineProps<Props>(), {
	hidden: false,
});

const copied = ref(false);

async function clickCopy() {
	await navigator.clipboard.writeText(props.link);
	copied.value = true;
	setTimeout(() => {
		copied.value = false;
	}, 3000);
}
</script>

<template>
	<div
		@click="clickCopy"
		title="Click to copy"
		class="wrapper"
		:class="{ hidden: hidden, copied: copied }"
	>
		<p>{{ link }}</p>
		<span v-if="!copied">Copy</span>
		<span v-else>Copied</span>
	</div>
</template>

<style scoped>
.wrapper {
	display: flex;
	align-items: center;
	--gap: 10px;
	gap: var(--gap);
	padding-inline: var(--gap);
	height: 35px;
	overflow: hidden;
	opacity: 1;
	cursor: pointer;
	width: fit-content;
	border: var(--border-width) solid var(--color-surface-2);
	border-radius: var(--radius-sm);
	transition:
		height var(--transition-fast),
		opacity var(--transition-slow),
		border-color var(--transition-fast);

	&.hidden {
		height: 0;
		opacity: 0;
	}

	&.copied {
		border-color: var(--color-success);
	}
}

p {
	font-size: 0.8em;
	font-weight: bold;
	color: var(--color-primary);
	margin: 0;
}
</style>
