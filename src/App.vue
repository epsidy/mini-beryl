<template>
    <RouterView v-slot="{ Component }">
        <Transition name="scale" mode="out-in">
            <KeepAlive include="Greet">
                <component :is="Component"/>
            </KeepAlive>
        </Transition>
    </RouterView>
</template>

<script setup lang="ts">
</script>

<style scoped>
.scale-enter-active,
.scale-leave-active {
    transition: all 0.5s ease;
}

.scale-enter-from,
.scale-leave-to {
    opacity: 0;
    transform: scale(0.9);
}
</style>