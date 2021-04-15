export interface IAppCard {
  name: string;
  description: string;
  icon: string;
  recommended?: boolean;
  installStatus: 'installed' | 'notInstalled' | 'upgradeable';
}

export interface IBasePackage {
  name: string;
  title: string;
  description: string;
  icon: string;
  url?: string;
  recommended: boolean;
  isInternal: boolean;
  type: 'tool' | 'app';
  version: string | null;
  path: string | null;
  installStatus: 'installed' | 'notInstalled' | 'upgradeable';
}

export enum InstallStatus {
  'installed' = '已安装',
  'notInstalled' = '未安装',
  'upgradeable' = '可升级'
}
