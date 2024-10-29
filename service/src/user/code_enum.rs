/// Event Code 事件码
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Code {
	/// 用户登录
	Login = 1,
	/// 用户登出
	Logout,
	/// 订单提交|同步BOPP数据
	OrderCreate,
	/// 预排单
	Preorder,
	/// 大计划取消
	SchedulerCancel = 5,
	/// 大计划作废
	SchedulerDiscard,
	/// 大计划创建
	SchedulerCreate,
	/// 大计划编排
	SchedulerEdit,
	/// 大计划提交
	SchedulerCommit,
	/// 大计划拆分
	SchedulerSlice,
	/// 大计划冻结
	ScheduleFreeze,
	/// 大计划解冻
	ScheduleUnfreeze,
	/// 工艺作废
	CrafterDiscard,
	/// 工艺创建
	CrafterCreate,
	/// 工艺编排
	CrafterEdit,
	/// 工艺提交
	CrafterCommit,
	/// 工艺拆分
	CrafterSlice,
	/// 工艺冻结
	CrafterFreeze,
	/// 工艺取消
	CrafterCancel,
	/// 工艺解冻
	CrafterUnfreeze,
	/// 主机
	Programmer,
	/// 分切
	Slice,
	/// 工艺当前卷
	CraftNowProduce,
	/// 大计划完成
	ScheduleComplete = 24,
	/// 工艺完成
	CraftComplete = 25,
	/// 大计划补单
	ScheduleSupplement = 26,
	/// 大计划删除
	ScheduleDelete = 27,
}
