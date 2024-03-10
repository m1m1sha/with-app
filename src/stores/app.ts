import { defineStore } from "pinia";

export interface WithRoute {
  ip: string;
  routes: Route[];
}

export interface Route {
  is_tcp: boolean;
  index: number;
  addr: string;
  metric: number;
  rt: number;
}

export interface NeedRoute {
  ip: string;
  rt: number;
  channel: string;
}

export enum WithStatus {
  Stopping,
  Stopped,
  Connecting,
  Connected,
}

export const useAppStore = defineStore("app", () => {
  const menu = ref("/");
  const settingTab = ref("basic");

  const withStatus = ref(WithStatus.Stopped);

  const withRoutes = ref<NeedRoute[]>([]);

  return {
    menu,
    settingTab,
    withStatus,
    withRoutes,
  };
});
