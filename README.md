# libra_vm
简单的提取
仅仅将vm_runtime依赖的模块全部拿了出来。新建了个项目将这些模块都包含进去，写了个cargo.toml解决依赖问题。写了个main测试vm运行。
目前一笔交易发出后会由于get_module失败导致在validate阶段就失败。
