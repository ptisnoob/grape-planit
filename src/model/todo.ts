//   { id: 1, title: '完成项目报告', startTime: 1690732800, endTime: 1690819200, notes: '包含Q2数据', expanded: false }
export interface Todo {
    id: number;
    title: string;
    notes?: string;
    category?: string;
    // 0、重要不紧急 1、重要且紧急 2、不重要不紧急 3、不重要但紧急
    level: number;
    cycle: 'one' | 'day' | 'week' | 'month' | 'year';
    // 开始时间戳（秒）
    startTime: number;
    // 结束时间戳（秒），可为空
    endTime?: number;
    // 0: 未开始 1: 已完成 2: 已删除
    status: number;
    // Ui 用的字段，无需存，默认false
    expanded: boolean;
}

export type TodoVo = {
    title: string;
    notes?: string;
    category?: string;
    // 0、重要不紧急 1、重要且紧急 2、不重要不紧急 3、不重要但紧急
    level: number;
    cycle: 'one' | 'day' | 'week' | 'month' | 'year';
    start_time: string;
    end_time?: string;
}