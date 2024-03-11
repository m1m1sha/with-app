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
  rt: string;
  channel: string;
}

export enum WithStatus {
  Stopping,
  Stopped,
  Connecting,
  Connected,
}

export interface WithLocalInfo {
  virtual_gateway: string;
  virtual_ip: string;
  virtual_netmask: string;
}

export const useAppStore = defineStore("app", () => {
  const menu = ref("/");
  const settingTab = ref("basic");

  const withStatus = ref(WithStatus.Stopped);

  const withRoutes = ref<NeedRoute[]>([]);
  const withTryConnect = ref(0);

  const withLocalInfo = ref<WithLocalInfo | null>();
  const withGatewayRoute = ref<NeedRoute | null>();

  return {
    menu,
    settingTab,
    withStatus,
    withRoutes,
    withLocalInfo,
    withGatewayRoute,
    withTryConnect,
  };
});
