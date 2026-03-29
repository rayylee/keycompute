use std::collections::HashMap;
use std::sync::LazyLock;

pub static ZH: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();

    // ── 导航 ────────────────────────────────────
    m.insert("nav.home", "首页");
    m.insert("nav.usage", "用量统计");
    m.insert("nav.billing", "账单管理");
    m.insert("nav.api_keys", "API Keys");
    m.insert("nav.payments", "支付中心");
    m.insert("nav.payments.balance", "余额查询");
    m.insert("nav.payments.orders", "订单列表");
    m.insert("nav.payments.recharge", "充值");
    m.insert("nav.distribution", "分销中心");
    m.insert("nav.distribution.earnings", "分销收益");
    m.insert("nav.distribution.referrals", "推荐列表");
    m.insert("nav.distribution.invite", "邀请管理");
    m.insert("nav.user", "个人中心");
    m.insert("nav.user.profile", "个人资料");
    m.insert("nav.user.security", "安全设置");
    m.insert("nav.users", "用户管理");
    m.insert("nav.accounts", "账号管理");
    m.insert("nav.pricing", "定价管理");
    m.insert("nav.payment_orders", "支付订单");
    m.insert("nav.distribution_records", "分销记录");
    m.insert("nav.tenants", "租户管理");
    m.insert("nav.system", "系统诊断");
    m.insert("nav.settings", "系统设置");

    // ── 认证 ────────────────────────────────────
    m.insert("auth.login", "登录");
    m.insert("auth.register", "注册");
    m.insert("auth.logout", "退出登录");
    m.insert("auth.forgot_password", "忘记密码");
    m.insert("auth.reset_password", "重置密码");
    m.insert("auth.email", "邮箱");
    m.insert("auth.password", "密码");
    m.insert("auth.confirm_password", "确认密码");
    m.insert("auth.name", "姓名");
    m.insert("auth.remember_me", "记住我");
    m.insert("auth.no_account", "还没有账号？");
    m.insert("auth.has_account", "已有账号？");
    m.insert("auth.send_reset_email", "发送重置邮件");
    m.insert("auth.back_to_login", "返回登录");

    // ── 页面标题 ─────────────────────────────────
    m.insert("page.home", "仪表盘");
    m.insert("page.usage", "用量统计");
    m.insert("page.billing", "账单管理");
    m.insert("page.api_keys", "API Key 管理");
    m.insert("page.payments", "支付中心");
    m.insert("page.distribution", "分销中心");
    m.insert("page.profile", "个人资料");
    m.insert("page.security", "安全设置");
    m.insert("page.users", "用户管理");
    m.insert("page.accounts", "账号管理");
    m.insert("page.pricing", "定价管理");
    m.insert("page.payment_orders", "支付订单");
    m.insert("page.distribution_records", "分销记录");
    m.insert("page.tenants", "租户管理");
    m.insert("page.system", "系统诊断");
    m.insert("page.settings", "系统设置");
    m.insert("page.not_found", "页面不存在");

    // ── 表单 ────────────────────────────────────
    m.insert("form.save", "保存");
    m.insert("form.cancel", "取消");
    m.insert("form.confirm", "确认");
    m.insert("form.delete", "删除");
    m.insert("form.create", "新建");
    m.insert("form.edit", "编辑");
    m.insert("form.search", "搜索");
    m.insert("form.reset", "重置");
    m.insert("form.submit", "提交");
    m.insert("form.required", "此字段为必填项");
    m.insert("form.invalid_email", "请输入有效的邮箱地址");
    m.insert("form.password_too_short", "密码至少 8 位");
    m.insert("form.password_mismatch", "两次密码不一致");

    // ── 表格 ────────────────────────────────────
    m.insert("table.no_data", "暂无数据");
    m.insert("table.loading", "加载中...");
    m.insert("table.actions", "操作");
    m.insert("table.status", "状态");
    m.insert("table.created_at", "创建时间");
    m.insert("table.name", "名称");
    m.insert("table.email", "邮箱");
    m.insert("table.role", "角色");

    // ── 通用 ────────────────────────────────────
    m.insert("common.loading", "加载中");
    m.insert("common.error", "出错了");
    m.insert("common.success", "操作成功");
    m.insert("common.confirm_delete", "确定要删除吗？此操作不可撤销。");
    m.insert("common.copied", "已复制到剪贴板");
    m.insert("common.copy", "复制");
    m.insert("common.refresh", "刷新");
    m.insert("common.back", "返回");
    m.insert("common.yes", "是");
    m.insert("common.no", "否");
    m.insert("common.admin", "管理员");
    m.insert("common.user", "普通用户");
    m.insert("common.no_permission", "您没有权限访问此页面");
    m.insert("common.balance", "余额");
    m.insert("common.amount", "金额");
    m.insert("common.currency", "货币");
    m.insert("common.tokens", "Token 数");
    m.insert("common.requests", "请求数");
    m.insert("common.cost", "费用");

    m
});
