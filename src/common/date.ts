/**
 * GDate 类 - 日期处理工具类
 * 支持多种输入类型：时间戳、日期字符串、Date对象或null
 */
export class GDate {
    private date: Date;

    /**
     * 构造函数
     * @param input 可选参数：时间戳number、时间格式字符串、Date对象或null
     */
    constructor(input?: number | string | Date | null) {
        if (input === null || input === undefined) {
            this.date = new Date();
        } else if (typeof input === 'number') {
            // 时间戳传入的只会到秒，需要转换为毫秒
            this.date = new Date(input * 1000);
        } else if (typeof input === 'string') {
            this.date = new Date(input);
        } else if (input instanceof Date) {
            this.date = new Date(input.getTime());
        } else {
            this.date = new Date();
        }
    }

    /**
     * 格式化日期
     * @param template 模板字符串，默认为 'YYYY-MM-DD'
     * @returns 格式化后的日期字符串
     */
    format(template: string = 'YYYY-MM-DD'): string {
        const year = this.date.getFullYear();
        const month = String(this.date.getMonth() + 1).padStart(2, '0');
        const day = String(this.date.getDate()).padStart(2, '0');
        const hours = String(this.date.getHours()).padStart(2, '0');
        const minutes = String(this.date.getMinutes()).padStart(2, '0');
        const seconds = String(this.date.getSeconds()).padStart(2, '0');

        return template
            .replace(/YYYY/g, String(year))
            .replace(/MM/g, month)
            .replace(/DD/g, day)
            .replace(/HH/g, hours)
            .replace(/mm/g, minutes)
            .replace(/ss/g, seconds);
    }

    /**
     * 获取原始Date对象
     */
    getDate(): Date {
        return new Date(this.date.getTime());
    }

    /**
     * 获取时间戳
     */
    getTime(): number {
        return this.date.getTime();
    }

    /**
     * 获取年份
     */
    getYear(): number {
        return this.date.getFullYear();
    }

    /**
     * 获取月份 (1-12)
     */
    getMonth(): number {
        return this.date.getMonth() + 1;
    }

    /**
     * 获取日期 (1-31)
     */
    getDay(): number {
        return this.date.getDate();
    }

    /**
     * 获取星期几 (0-6, 0为周日)
     */
    getWeekDay(): number {
        return this.date.getDay();
    }

    /**
     * 获取小时 (0-23)
     */
    getHours(): number {
        return this.date.getHours();
    }

    /**
     * 获取分钟 (0-59)
     */
    getMinutes(): number {
        return this.date.getMinutes();
    }

    /**
     * 获取秒数 (0-59)
     */
    getSeconds(): number {
        return this.date.getSeconds();
    }

    /**
     * 添加天数
     * @param days 要添加的天数
     */
    addDays(days: number): GDate {
        const newDate = new Date(this.date.getTime());
        newDate.setDate(newDate.getDate() + days);
        return new GDate(newDate);
    }

    /**
     * 添加月份
     * @param months 要添加的月数
     */
    addMonths(months: number): GDate {
        const newDate = new Date(this.date.getTime());
        newDate.setMonth(newDate.getMonth() + months);
        return new GDate(newDate);
    }

    /**
     * 添加年份
     * @param years 要添加的年数
     */
    addYears(years: number): GDate {
        const newDate = new Date(this.date.getTime());
        newDate.setFullYear(newDate.getFullYear() + years);
        return new GDate(newDate);
    }

    /**
     * 获取月初日期
     */
    getStartOfMonth(): GDate {
        const newDate = new Date(this.date.getFullYear(), this.date.getMonth(), 1);
        return new GDate(newDate);
    }

    /**
     * 获取月末日期
     */
    getEndOfMonth(): GDate {
        const newDate = new Date(this.date.getFullYear(), this.date.getMonth() + 1, 0);
        return new GDate(newDate);
    }

    /**
     * 获取今天开始时间 (00:00:00)
     */
    getStartOfDay(): GDate {
        const newDate = new Date(this.date.getFullYear(), this.date.getMonth(), this.date.getDate());
        return new GDate(newDate);
    }

    /**
     * 获取今天结束时间 (23:59:59)
     */
    getEndOfDay(): GDate {
        const newDate = new Date(this.date.getFullYear(), this.date.getMonth(), this.date.getDate(), 23, 59, 59, 999);
        return new GDate(newDate);
    }

    /**
     * 判断是否是今天
     */
    isToday(): boolean {
        const today = new Date();
        return this.date.getFullYear() === today.getFullYear() &&
            this.date.getMonth() === today.getMonth() &&
            this.date.getDate() === today.getDate();
    }

    /**
     * 判断是否是闰年
     */
    isLeapYear(): boolean {
        const year = this.date.getFullYear();
        return (year % 4 === 0 && year % 100 !== 0) || (year % 400 === 0);
    }

    /**
     * 计算与另一个日期的天数差
     * @param other 另一个GDate对象
     */
    diffDays(other: GDate): number {
        const diffTime = Math.abs(this.date.getTime() - other.date.getTime());
        return Math.ceil(diffTime / (1000 * 60 * 60 * 24));
    }

    /**
     * 比较两个日期
     * @param other 另一个GDate对象
     * @returns 1: 当前日期大于other, 0: 相等, -1: 当前日期小于other
     */
    compare(other: GDate): number {
        if (this.date.getTime() > other.date.getTime()) return 1;
        if (this.date.getTime() < other.date.getTime()) return -1;
        return 0;
    }

    /**
     * 判断是否在指定日期之前
     */
    isBefore(other: GDate): boolean {
        return this.date.getTime() < other.date.getTime();
    }

    /**
     * 判断是否在指定日期之后
     */
    isAfter(other: GDate): boolean {
        return this.date.getTime() > other.date.getTime();
    }

    /**
     * 判断是否与指定日期相同
     */
    isSame(other: GDate): boolean {
        return this.date.getTime() === other.date.getTime();
    }

    /**
     * 克隆当前日期对象
     */
    clone(): GDate {
        return new GDate(this.date);
    }

    /**
     * 转换为字符串
     */
    toString(): string {
        return this.date.toString();
    }

    /**
     * 转换为ISO字符串
     */
    toISOString(): string {
        return this.date.toISOString();
    }

    /**
     * 静态方法：获取当前日期
     */
    static now(): GDate {
        return new GDate();
    }

    /**
     * 静态方法：从时间戳创建
     * @param timestamp 秒级时间戳
     */
    static fromTimestamp(timestamp: number): GDate {
        return new GDate(timestamp);
    }

    /**
     * 静态方法：从字符串创建
     */
    static fromString(dateString: string): GDate {
        return new GDate(dateString);
    }

    /**
     * 获取中文格式的详细时间字符串
     * 格式：现在是2025年7月3日16:18，周四
     */
    toChineseString(): string {
        const weekDays = ['日', '一', '二', '三', '四', '五', '六'];
        const weekDay = weekDays[this.getWeekDay()];
        return `${this.getYear()}年${this.getMonth()}月${this.getDay()}日${this.getHours()}:${String(this.getMinutes()).padStart(2, '0')}，周${weekDay}`;
    }
}

// 默认导出
export default GDate;