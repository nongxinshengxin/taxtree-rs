# taxtree

NCBI Taxonomy 数据库的 Rust 命令行工具与库。从 **names.dmp** 和 **nodes.dmp** 构建分类学索引，支持分类等级查询、进化树构建、谱系展开，输出 TSV / JSON / Newick 格式。

## 背景

[NCBI Taxonomy](https://www.ncbi.nlm.nih.gov/taxonomy) 收录了地球上已描述物种中约 10% 的生物分类与命名信息，当前包含超过 **76 万**个分类单元（taxa）。本项目是 R 包 [taxtree](https://github.com/nongxinshengxin/taxtree) 的 Rust 移植版本，保留了其核心的分类查询与树构建能力，同时提供更快的执行速度和更小的资源占用。

## 安装

### 预编译二进制

从 [Releases](https://github.com/nongxinshengxin/taxtree/releases) 页面下载对应平台的二进制文件，放入 `PATH` 即可。

### 从源码编译

需要 Rust 工具链（[rustup](https://rustup.rs/)）：

```bash
git clone https://github.com/nongxinshengxin/taxtree.git
cd taxtree/crates
cargo build --release
```

编译产物位于 `target/release/taxtree`，可将其复制到任意 `PATH` 目录。

## 数据准备

使用前需要从 NCBI 下载 Taxonomy 数据库的 dump 文件：

```bash
# 下载并解压
wget https://ftp.ncbi.nih.gov/pub/taxonomy/taxdump.tar.gz
mkdir taxonomy && tar -xzf taxdump.tar.gz -C taxonomy

# 只需要这两个文件
ls taxonomy/names.dmp taxonomy/nodes.dmp
```

> **提示**：完整数据库约 70 MB 压缩包，解压后约 500 MB。项目测试夹具中包含一份 mini 版本的数据，仅用于验证功能。

## 快速开始

```bash
# 1. 构建索引（首次使用）
taxtree index --dump-dir ./taxonomy --out ./taxonomy.ttidx

# 2. 查询分类等级
taxtree rank --index ./taxonomy.ttidx --name "Homo sapiens"

# 3. 从名称列表构建进化树
echo -e "Homo sapiens\nPan troglodytes\nMus musculus" > taxa.txt
taxtree tree --index ./taxonomy.ttidx --input taxa.txt --format newick --out tree.nwk

# 4. 展开某个节点的子分类
taxtree lineage --index ./taxonomy.ttidx --name Primates --depth 2 --format json
```

## 命令参考

所有命令均通过 `--help` 提供完整说明，例如 `taxtree index --help`。

### index — 构建索引

解析 `names.dmp` 和 `nodes.dmp`，构建预计算索引文件（`.ttidx`），供后续查询使用。

```
taxtree index --dump-dir <DIR> --out <FILE>
```

| 参数 | 说明 |
|------|------|
| `--dump-dir` | 包含 `names.dmp` 和 `nodes.dmp` 的目录 |
| `--out` | 输出的索引文件路径，推荐后缀 `.ttidx` |

索引文件包含 MAGIC 头和版本号，后续加载时会校验兼容性。完整数据库的索引构建通常在 10–20 秒内完成，索引文件约 50–80 MB。

---

### rank — 分类等级查询

根据分类单元名称查询其 taxid 和等级（species / genus / family / order / class / phylum / kingdom / superkingdom 等）。

```
taxtree rank --index <FILE> (--name <NAME> | --input <FILE>) [--format <FMT>] [--out <FILE>]
```

| 参数 | 说明 |
|------|------|
| `--index` | 索引文件路径 |
| `--name` | 单个分类名称 |
| `--input` | 每行一个名称的文本文件 |
| `--format` | 输出格式：`tsv`（默认）或 `json` |
| `--out` | 输出文件路径，省略则打印到 stdout |

**示例：**

```bash
# 单个查询
taxtree rank --index taxonomy.ttidx --name "Escherichia coli"
# 输出: name            taxid   rank
#       Escherichia coli  562    species

# 批量查询
taxtree rank --index taxonomy.ttidx --input names.txt --format json --out ranks.json
```

> **注意**：若同一名称对应多个 taxid（如不同界下的同名分类），命令会报错并列出所有候选 ID。

---

### tree — 构建进化树

给定一组分类名称（或 taxid），追溯每个分类的完整祖先链，合并为一棵包含所有输入分类及其共同祖先的树。

```
taxtree tree --index <FILE> (--input <FILE> | --taxid <ID>...) [--format <FMT>] [--out <FILE>]
```

| 参数 | 说明 |
|------|------|
| `--index` | 索引文件路径 |
| `--input` | 每行一个名称的文本文件 |
| `--taxid` | 直接用 taxid 指定（可多次使用，与 `--input` 可同时使用） |
| `--format` | 输出格式：`newick`（默认）、`tsv` 或 `json` |
| `--out` | 输出文件路径 |

**示例：**

```bash
# Newick 格式（默认），可用于 ggtree / iTOL / FigTree
taxtree tree --index taxonomy.ttidx --input taxa.txt --out tree.nwk

# TSV 格式，方便导入电子表格或数据库
taxtree tree --index taxonomy.ttidx --input taxa.txt --format tsv

# 混合使用名称文件和 taxid
taxtree tree --index taxonomy.ttidx --input taxa.txt --taxid 9606 --taxid 10090

# 纯 taxid 输入
taxtree tree --index taxonomy.ttidx --taxid 9606 --taxid 9598 --format json
```

**Newick 输出说明：**

- 标签中的空白字符被替换为 `_`，特殊字符 `()',:;` 被替换为 `_`
- 输出以分号 `;` 结尾，符合 Newick 标准
- 可直接导入 [ggtree](https://bioconductor.org/packages/ggtree/)（R）、[iTOL](https://itol.embl.de/)、[FigTree](http://tree.bio.ed.ac.uk/software/figtree/) 等工具

**JSON 输出结构：**

```json
{
  "root_id": 1,
  "nodes": [
    { "id": 9606, "name": "Homo sapiens", "rank": "species", "parent_id": 9605 }
  ],
  "edges": [
    { "child": {...}, "parent": {...} }
  ]
}
```

---

### lineage — 谱系展开

以某个分类节点为起点，向下展开其子节点，支持指定展开深度。

```
taxtree lineage --index <FILE> (--name <NAME> | --taxid <ID>) [--depth <N>] [--format <FMT>] [--out <FILE>]
```

| 参数 | 说明 |
|------|------|
| `--index` | 索引文件路径 |
| `--name` | 起始分类名称 |
| `--taxid` | 起始 taxid（与 `--name` 二选一） |
| `--depth` | 展开深度，默认为 1（仅直接子节点） |
| `--format` | 输出格式：`tsv`（默认）或 `json` |
| `--out` | 输出文件路径 |

**示例：**

```bash
# 查看 Primates（灵长目）的直接子分类
taxtree lineage --index taxonomy.ttidx --name Primates

# 向下展开 3 层
taxtree lineage --index taxonomy.ttidx --name Primates --depth 3 --format json

# 使用 taxid
taxtree lineage --index taxonomy.ttidx --taxid 9443 --depth 2
```

**TSV 输出列：**

```
child_taxid    child_name    parent_taxid    parent_name    rank
```

---

## 输出格式速查

| 格式 | 适用命令 | 说明 |
|------|----------|------|
| **TSV** | rank, tree, lineage | 制表符分隔，带表头，适合 `grep`/`awk` 处理或导入 Excel |
| **JSON** | rank, tree, lineage | 结构化数据，适合程序化消费 |
| **Newick** | tree | 括号化树格式，用于系统发育树可视化工具 |

## 作为库使用

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
taxtree-core = { git = "https://github.com/nongxinshengxin/taxtree", branch = "main" }
```

### 核心 API

```rust
use taxtree_core::{TaxonomyIndex, TaxonId, OutputFormat};

// 从 dump 文件构建索引
let index = TaxonomyIndex::build_from_dump("./taxonomy")?;

// 保存 / 加载索引
index.save("./taxonomy.ttidx")?;
let index = TaxonomyIndex::load("./taxonomy.ttidx")?;

// 查询
let record = index.rank_by_name("Homo sapiens")?;
assert_eq!(record.id, 9606);
assert_eq!(record.rank, "species");

// 祖先链
let ancestors = index.ancestors(9606)?;
// [9606(Homo sapiens), 9605(Homo), 9604(Hominidae), ..., 1(root)]

// 子孙展开
let edges = index.descendants(9443, 2)?; // Primates 下 2 层

// 构建树
let tree = index.build_tree(&[9606, 9598])?;

// 格式化输出
use taxtree_core::format;
let newick = format::newick::tree(&tree);
let json = format::json::tree(&tree)?;
let tsv = format::tsv::edges(&tree.edges);
```

### 错误处理

所有 API 返回 `Result<T, TaxTreeError>`，`TaxTreeError` 实现了 `std::error::Error` 和 `Display`：

```rust
match index.rank_by_name("unknown") {
    Err(TaxTreeError::NameNotFound(name)) => eprintln!("未找到: {name}"),
    Err(TaxTreeError::AmbiguousName { name, candidates }) => {
        eprintln!("名称 {name} 有歧义，候选 taxid: {candidates:?}");
    }
    Err(e) => eprintln!("其他错误: {e}"),
    Ok(record) => println!("{} 的等级是 {}", record.name, record.rank),
}
```

## 性能

完整 NCBI Taxonomy 数据库（约 76 万 taxa，数据截至 2024 年 1 月「可能已更新」）：

| 操作 | 耗时 |
|------|------|
| 索引构建（含 dump 解析） | ~15 秒 |
| 索引加载（从 `.ttidx` 文件） | ~2 秒 |
| 名称查询 | <1 ms |
| 祖先追溯（物种→根） | <1 ms |
| 树构建（100 个输入分类） | ~50 ms |

索引文件大小约 60 MB（`bincode` 序列化）。

## 与原 R 版 taxtree 的对比

| 功能 | R 版 | Rust 版 |
|------|------|---------|
| 分类等级查询（name2rank） | 支持 | `taxtree rank` |
| 谱系展开（find_Lineage） | 支持 | `taxtree lineage` |
| 进化树构建（make_Taxtree） | 支持 | `taxtree tree` |
| 内置绘图（plot_taxTree） | 基于 ggtree | 不内置，导出 Newick 后用外部工具 |
| 导出 Newick（write_taxTree） | 基于 ape | `--format newick` |
| 批量处理 | 向量化输入 | 文件输入 / 多次 `--taxid` |
| 运行时依赖 | R + BiocManager + ggtree | 单二进制文件，无运行时依赖 |

## 引用

如果使用本项目，请引用：

- 原始 taxtree R 包：<https://github.com/nongxinshengxin/taxtree>
- NCBI Taxonomy 数据库：<https://www.ncbi.nlm.nih.gov/taxonomy>
- Yu G, Smith DK, Zhu H, Guan Y, Lam TTY (2017). ggtree: an R package for visualization and annotation of phylogenetic trees. *Methods in Ecology and Evolution*, 8(1):28-36.

## 许可

MIT License
