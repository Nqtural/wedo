<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from "vue";

import Header from "../components/Header.vue";

defineProps<{
	title: string;
}>();

const list = ref<HTMLElement | null>(null);
const canScrollUp = ref(false);
const canScrollDown = ref(false);

const updateScrollShadows = () => {
	if (!list.value) return;

	const { scrollTop, scrollHeight, clientHeight } = list.value;

	canScrollUp.value = scrollTop > 0;
	canScrollDown.value = scrollTop + clientHeight < scrollHeight - 1;
};

let resizeObserver: ResizeObserver;

onMounted(() => {
	updateScrollShadows();

	resizeObserver = new ResizeObserver(() => {
		updateScrollShadows();
	});

	if (list.value) {
		resizeObserver.observe(list.value);
	}
});

onBeforeUnmount(() => {
	resizeObserver?.disconnect();
});
</script>

<template>
	<Header />

	<div class="list-page">
		<div class="header">
			<div class="header__main">
				<slot name="back" />
				<h2>{{ title }}</h2>
				<slot name="filter-button" />
			</div>
			<Transition name="grow-vertical">
				<div v-if="$slots.filter" class="filter">
					<slot name="filter" />
				</div>
			</Transition>
		</div>
		<hr />
		<div
			class="list-wrapper"
			:class="{
				'shadow-top': canScrollUp,
				'shadow-bottom': canScrollDown,
			}"
		>
			<ul ref="list" @scroll="updateScrollShadows">
				<slot />
			</ul>
		</div>
		<slot name="actions" />
	</div>
</template>

<style scoped>
.grow-vertical-enter-active,
.grow-vertical-leave-active {
	transition:
		max-height var(--transition-slow),
		margin-top var(--transition-slow);
}

.grow-vertical-enter-from,
.grow-vertical-leave-to {
	max-height: 0;
}

.grow-vertical-enter-to,
.grow-vertical-leave-from {
	max-height: 15em;
}

.list-page {
	display: flex;
	flex-direction: column;
	gap: 20px;
	align-items: center;
	width: min(50em, calc(100vw - 5em));
	background: var(--color-surface-0);
	border-radius: var(--radius-sm);
	color: var(--color-text);
	padding: 20px;
	max-height: 450px;

	/* push content further up */
	margin-bottom: 10em;
}

.header {
	width: 100%;

	& .header__main {
		display: flex;
		gap: 20px;
	}
}

h2 {
	display: inline-block;
	float: left;
	margin: 0;
	margin-right: auto;
}

.filter {
	display: flex;
	flex-direction: column;
	gap: 10px;
	width: 100%;
	overflow: hidden;

	&::before {
		/* hacky solution to animate space
		between main header and filter */
		content: "";
		display: block;
		flex: 0 0 20px;
	}
}

hr {
	height: 1.5px;
	background: var(--color-text);
	width: 100%;
	border: none;
	margin: 0;
	flex-shrink: 0;
}

.list-wrapper {
	position: relative;
	width: 100%;
	max-height: 18em;
}

ul {
	margin: 0;
	padding: 0;
	list-style: none;
	display: flex;
	gap: 10px;
	flex-direction: column;
	max-height: 18em;
	overflow-y: auto;
	width: 100%;
}

.list-wrapper::before,
.list-wrapper::after {
	content: "";
	position: absolute;
	left: 0;
	right: 0;
	height: 5px;
	z-index: 1;
	pointer-events: none;
	opacity: 0;
	transition: opacity var(--transition-fast);
}

.list-wrapper::before {
	top: 0;
	background: linear-gradient(
		to bottom,
		var(--surface-0) 0%,
		color-mix(in srgb, var(--surface-0) 80%, transparent) 25%,
		color-mix(in srgb, var(--surface-0) 30%, transparent) 60%,
		transparent 100%
	);
}

.list-wrapper::after {
	bottom: 0;
	background: linear-gradient(
		to top,
		var(--surface-0) 0%,
		color-mix(in srgb, var(--surface-0) 80%, transparent) 25%,
		color-mix(in srgb, var(--surface-0) 30%, transparent) 60%,
		transparent 100%
	);
}

.list-wrapper.shadow-top::before,
.list-wrapper.shadow-bottom::after {
	opacity: 1;
}
</style>
