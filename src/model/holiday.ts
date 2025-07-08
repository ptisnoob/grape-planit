/**
 * 节假日数据模型
 */
export interface Holiday {
  /** 节日名称 */
  name: string;
  /** 日期, ISO 8601 格式 */
  date: string;
  /** 是否为休息日 */
  isOffDay: boolean;
}

/**
 * 年度节假日数据
 */
export interface HolidayYear {
  /** 完整年份, 整数 */
  year: number;
  /** 所用国务院文件网址列表 */
  papers: string[];
  /** 节假日列表 */
  days: Holiday[];
}

/**
 * 存储的节假日年份信息
 */
export interface StoredHolidayYear {
  /** 年份 */
  year: number;
  /** 同步时间 */
  syncTime: string;
  /** 节假日数量 */
  count: number;
}

/**
 * 同步节假日参数
 */
export interface SyncHolidayParams {
  year: number;
}

/**
 * 代理配置
 */
export interface ProxySettings {
  id: number;
  enabled: boolean;
  proxyUrl: string;
  createdAt: string;
  updatedAt: string;
}

/**
 * 更新代理配置参数
 */
export interface UpdateProxyParams {
  enabled: boolean;
  proxyUrl: string;
}