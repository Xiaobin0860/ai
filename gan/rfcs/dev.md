# gan

## 运行

1. 安装[rust](https://www.rust-lang.org/tools/install) \
   `cargo build --release`编译
2. 复制可执行程序, 配置文件(`app_cfg.toml`,`auto_cfg_template.toml`格式用法文件内有注释)和`api`文件(`文生图api.json`,`图生图api.json`,或其它导出文件)到运行目录, 执行程序

```
# clean参数用于清理之前的跑图进度
gan --cfg app_cfg.toml [--clean]
```

3. 使用`svn/meikao/tools/pngclean.py <fromDir> <outDir> [recursive]`来清理图片参数, 输出目录可给出去用来选图
4. 使用`svn/meikao/tools/pnginfo2.py <pickDir> <originDir> <outExcel> [recursive]`来生成选图的参数文件

## `comfy`流程修改

1. 使用最新版`comfyui`编辑[文生图](../fixtures/data/美考文生图流程.json)/[图生图](../fixtures/data/美考图生图流程.json)流程. \
   注意保证`node`的`tile`在整个流程中唯一, 需要根据`tile`找到`node`修改参数

2. 导出`api`格式文件, 保存到[文生图 api](../fixtures/data/文生图api.json)/[图生图 api](../fixtures/data/图生图api.json) \
   用于跑单元测试, 可以验证变更的流程是不能正确解析 \
   可以编辑导出多个`api文件`入到程序运行目录, 给[参数配置文件](../fixtures/conf/auto_cfg_template.toml)中的`workflows`字段使用

## 增加新`node`及其随机参数配置

1. [class_names](../fixtures/data/class_names.json)中增加新屡有`node`的`tile`到`class_type`的映射 \
   参照之前的`node`, 创建新的解析结构, 主要修改`src/comfy/`里`inputs.rs`和`node.rs`文件, 增加相应的类型

2. [参数配置](../fixtures/conf/auto_cfg_template.toml)中增加新`node`的参数配置 \
   修改`app_args.rs`和`generator.rs`处理新参数解析及生成
