export interface AppConfig {
  autoUpdate: boolean;
  autoStart: boolean;
  startMinimize: boolean;
  systemNotification: boolean;
}

export const DEFAULT_APP_CONFIG: AppConfig = {
  autoUpdate: true,
  autoStart: false,
  startMinimize: false,
  systemNotification: true,
};
