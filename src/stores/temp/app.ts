import { defineStore } from "pinia";

export const useTempAppStore = defineStore("temp_app", () => {
  const appConfigVisible = ref(false);

  return {
    appConfigVisible,
  };
});
