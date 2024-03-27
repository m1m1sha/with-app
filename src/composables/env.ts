import pkg from '../../package.json';

export function isDev(): boolean {
  return import.meta.env.DEV
}

export function isProd(): boolean {
  return import.meta.env.PROD
}

export function PKG() {
  return pkg
}