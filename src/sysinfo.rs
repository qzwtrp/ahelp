use std::env;
use sysinfo::{Disks, System};

pub struct Context {
    pub os: String,
    pub kernel: String,
    pub hostname: String,
    pub arch: String,
    pub cpu_cores: usize,
    pub cpu_threads: usize,
    pub cpu_usage: f32,
    pub ram_total_mb: u64,
    pub ram_used_mb: u64,
    pub ram_percent: f32,
    pub disk_total_gb: u64,
    pub disk_used_gb: u64,
    pub disk_percent: f32,
    pub shell: String,
    pub user: String,
    pub home: String,
    pub editor: String,
    pub term: String,
    pub pwd: String,
}

pub fn collect() -> Context {
    let mut sys = System::new_all();
    sys.refresh_all();

    let disks = Disks::new_with_refreshed_list();
    let (total_gb, used_gb, disk_pct) = disks
        .iter()
        .find(|d| d.mount_point() == std::path::Path::new("/"))
        .map(|d| {
            let t = d.total_space();
            let a = d.available_space();
            let u = t - a;
            (
                t / 1024 / 1024 / 1024,
                u / 1024 / 1024 / 1024,
                (u as f64 / t as f64 * 100.0) as f32,
            )
        })
        .unwrap_or((0, 0, 0.0));

    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let ram_total = sys.total_memory();
    let ram_used = sys.used_memory();

    Context {
        os: System::name().unwrap_or_else(|| "Linux".into()),
        kernel: System::kernel_version().unwrap_or_default(),
        hostname: System::host_name().unwrap_or_default(),
        arch: env::consts::ARCH.into(),
        cpu_cores: sys.physical_core_count().unwrap_or(0),
        cpu_threads: sys.cpus().len(),
        cpu_usage,
        ram_total_mb: ram_total / 1024 / 1024,
        ram_used_mb: ram_used / 1024 / 1024,
        ram_percent: if ram_total > 0 {
            (ram_used as f32 / ram_total as f32) * 100.0
        } else {
            0.0
        },
        disk_total_gb: total_gb,
        disk_used_gb: used_gb,
        disk_percent: disk_pct,
        shell: env::var("SHELL")
            .unwrap_or_else(|_| "/bin/sh".into())
            .split('/')
            .last()
            .unwrap_or("sh")
            .into(),
        user: env::var("USER").unwrap_or_else(|_| "unknown".into()),
        home: dirs::home_dir()
            .map(|p| p.to_string_lossy().into())
            .unwrap_or_else(|| "/".into()),
        editor: env::var("EDITOR").unwrap_or_else(|_| "nano".into()),
        term: env::var("TERM").unwrap_or_else(|_| "xterm".into()),
        pwd: env::current_dir()
            .map(|p| p.to_string_lossy().into())
            .unwrap_or_else(|_| "/".into()),
    }
}

pub fn format_context(ctx: &Context) -> String {
    format!(
        r#"=== SYSTEM CONTEXT ===
OS:        {os} ({kernel})
Host:      {host} ({arch})
User:      {user} @ {shell}
PWD:       {pwd}
CPU:       {cores}c/{threads}t  {cpu:.0}% usage
RAM:       {ramu} / {ramt} MB  ({ramp:.0}%)
Disk (/):  {disku} / {diskt} GB  ({diskp:.0}%)
Term:      {term}  Editor: {editor}
======================

You are a senior Linux sysadmin companion. Given the system context above, provide concise, accurate, and copy-paste ready commands. Prefer GNU/coreutils. Explain briefly what each command does. If the user asks for a script, output a ready-to-run block. Never hallucinate package names — use standard repo names. Use the same shell syntax as the user's shell."#,
        os = ctx.os,
        kernel = ctx.kernel,
        host = ctx.hostname,
        arch = ctx.arch,
        user = ctx.user,
        shell = ctx.shell,
        pwd = ctx.pwd,
        cores = ctx.cpu_cores,
        threads = ctx.cpu_threads,
        cpu = ctx.cpu_usage,
        ramu = ctx.ram_used_mb,
        ramt = ctx.ram_total_mb,
        ramp = ctx.ram_percent,
        disku = ctx.disk_used_gb,
        diskt = ctx.disk_total_gb,
        diskp = ctx.disk_percent,
        term = ctx.term,
        editor = ctx.editor,
    )
}
