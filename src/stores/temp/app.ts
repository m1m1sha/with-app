import { defineStore } from "pinia";

export const useTempAppStore = defineStore("temp_app", () => {
  const activeMenu = ref("/");
  const advancedSetting = ref(false);

  return {
    activeMenu,
    advancedSetting,
  };
});
