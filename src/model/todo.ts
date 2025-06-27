//   { id: 1, title: '完成项目报告', dueDate: '2024-07-20', notes: '包含Q2数据', expanded: false }
export interface Todo {
    id: number;
    title: string;
    notes?: string;
    category?: string;
    // 1、不重要也不紧急 2、重要但不紧急 3、紧急但不重要 4、重要且紧急
    level: number;
    cycle: 'one' | 'day' | 'week' | 'month' | 'year';
    // 截止时间
    startTime: string;
    endTime?: string;
    // 0: 未开始 2: 已完成 
    status: number;
    // Ui 用的字段，无需存，默认false
    expanded: boolean;
}