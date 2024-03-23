import { defineStore } from "pinia";

export const useTempN2NStore = defineStore("temp_n2n", () => {
  const activeMenu = ref("/");

  return {
    activeMenu,
  };
});
