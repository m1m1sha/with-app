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

//  route: [RoutePayload { ip: 10.26.0.1, routes: [Route { is_tcp: false, index: 0, addr: [::ffff:8.134.146.7]:29872, metric: 1, rt: 29 }] }]
export const useAppStore = defineStore("app", () => {
  const menu = ref("/");
  const settingTab = ref("basic");

  const withStatus = ref("stopped");

  const withRoutes = ref<NeedRoute[]>([]);

  return {
    menu,
    settingTab,
    withStatus,
    withRoutes,
  };
});
