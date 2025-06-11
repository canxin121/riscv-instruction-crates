# RISC-V 指令按扩展分类详细报告

**报告生成时间**: 2025-06-11 20:02:42

## 📊 概览统计

| 扩展 | 标准指令数量 | 压缩指令数量 | 总计 | 描述 |
|------|-------------|-------------|------|------|
| B | 12 | 0 | 12 | 位操作扩展 |
| C | 0 | 34 | 34 | 压缩指令扩展 |
| D | 42 | 0 | 42 | 双精度浮点扩展 |
| F | 37 | 0 | 37 | 单精度浮点扩展 |
| H | 15 | 0 | 15 | 虚拟化扩展 |
| I | 55 | 0 | 55 | 基本整数指令集 |
| M | 13 | 0 | 13 | 乘法除法扩展 |
| Q | 43 | 0 | 43 | 四精度浮点扩展 |
| S | 2 | 0 | 2 | 特权架构扩展 |
| Sdext | 1 | 0 | 1 | 调试扩展 |
| Smdbltrp | 1 | 0 | 1 | M模式双陷阱扩展 |
| Smrnmi | 1 | 0 | 1 | M模式可恢复非屏蔽中断扩展 |
| Svinval | 5 | 0 | 5 | 细粒度地址转换缓存无效化扩展 |
| V | 627 | 0 | 627 | 向量扩展 |
| Zaamo | 18 | 0 | 18 | 原子内存操作扩展 |
| Zabha | 20 | 0 | 20 | 字节和半字原子操作扩展 |
| Zacas | 3 | 0 | 3 | 比较交换原子操作扩展 |
| Zalasr | 8 | 0 | 8 | 加载保留/存储条件扩展 |
| Zalrsc | 4 | 0 | 4 | LR/SC原子操作扩展 |
| Zawrs | 2 | 0 | 2 | 等待保留集扩展 |
| Zba | 8 | 0 | 8 | 地址生成位操作扩展 |
| Zbb | 14 | 0 | 14 | 基本位操作扩展 |
| Zbc | 1 | 0 | 1 | 进位位操作扩展 |
| Zbkb | 6 | 0 | 6 | 位操作加密扩展(基本) |
| Zbkx | 2 | 0 | 2 | 位操作加密扩展(交叉) |
| Zbs | 8 | 0 | 8 | 单位位操作扩展 |
| Zcb | 0 | 12 | 12 | 压缩基本扩展 |
| Zcd | 0 | 4 | 4 | 压缩双精度浮点扩展 |
| Zcf | 0 | 4 | 4 | 压缩单精度浮点扩展 |
| Zcmop | 0 | 1 | 1 | 压缩可能操作扩展 |
| Zcmp | 6 | 0 | 6 | 压缩指针操作扩展 |
| Zfbfmin | 2 | 0 | 2 | 标量BF16转换扩展 |
| Zfh | 41 | 0 | 41 | 半精度浮点扩展 |
| Zicbom | 3 | 0 | 3 | 缓存块管理扩展 |
| Zicboz | 1 | 0 | 1 | 缓存块清零扩展 |
| Zicfilp | 1 | 0 | 1 | 控制流完整性扩展 |
| Zicfiss | 7 | 0 | 7 | 影子栈扩展 |
| Zicond | 2 | 0 | 2 | 条件操作扩展 |
| Zicsr | 6 | 0 | 6 | 控制状态寄存器扩展 |
| Zifencei | 1 | 0 | 1 | 指令同步扩展 |
| Zilsd | 2 | 0 | 2 | 负载存储成对扩展 |
| Zimop | 2 | 0 | 2 | 可能操作扩展 |
| Zkn | 2 | 0 | 2 | 加密NIST算法扩展 |
| Zknd | 5 | 0 | 5 | NIST AES解密扩展 |
| Zkne | 4 | 0 | 4 | NIST AES加密扩展 |
| Zknh | 14 | 0 | 14 | NIST SHA哈希扩展 |
| Zks | 4 | 0 | 4 | 加密ShangMi算法扩展 |
| Zvbb | 16 | 0 | 16 | 向量基本位操作扩展 |
| Zvbc | 4 | 0 | 4 | 向量进位位操作扩展 |
| Zvfbfmin | 2 | 0 | 2 | 向量BF16转换扩展 |
| Zvfbfwma | 2 | 0 | 2 | 向量BF16乘加扩展 |
| Zvkg | 2 | 0 | 2 | 向量GCM/GMAC扩展 |
| Zvkned | 11 | 0 | 11 | 向量NIST AES扩展 |
| Zvknha | 3 | 0 | 3 | 向量NIST SHA-2扩展 |
| Zvks | 5 | 0 | 5 | 向量ShangMi扩展 |

## 🔧 B 扩展指令

**扩展描述**: 位操作扩展

**指令总数**: 12 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `andn` | RV32/64 | 3 | `andn {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `clmul` | RV32/64 | 3 | `clmul {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `clmulh` | RV32/64 | 3 | `clmulh {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `orn` | RV32/64 | 3 | `orn {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `rev8` | RV32/64 | 2 | `rev8 {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `rol` | RV32/64 | 3 | `rol {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `rolw` | RV64 | 3 | `rolw {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `ror` | RV32/64 | 3 | `ror {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `rori` | RV32/64 | 3 | `rori {xd}, {xs1}, {shamt}` | `xd`, `shamt`, `xs1` | `xd`: RV64:5, RV32:5; `shamt`: RV32:5, RV64:6; `xs1`: RV64:5, RV32:5 | 无 |
| `roriw` | RV64 | 3 | `roriw {xd}, {xs1}, {shamt}` | `shamt`, `xs1`, `xd` | `shamt`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `rorw` | RV64 | 3 | `rorw {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `xnor` | RV32/64 | 3 | `xnor {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |

---

## 🔧 C 扩展指令

**扩展描述**: 压缩指令扩展

**指令总数**: 34 条

### 📦 压缩指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `c.add` | RV32/64 | 2 | `c.add {xd}, {xs2}` | `xs2`, `xd` | `xs2`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | `xs2`: 禁止:0; `xd`: 禁止:0 |
| `c.addi` | RV32/64 | 2 | `c.addi {xd}, {imm}` | `imm`, `xd` | `imm`: RV64:6, RV32:6; `xd`: RV64:5, RV32:5 | `imm`: 禁止:0; `xd`: 禁止:0 |
| `c.addi16sp` | RV32/64 | 1 | `c.addi16sp sp, {imm}` | `imm` | `imm`: RV64:6, RV32:6 | `imm`: 16的倍数, 禁止:0 |
| `c.addi4spn` | RV32/64 | 2 | `c.addi4spn {xd}, sp, {uimm}` | `uimm`, `xd` | `uimm`: RV64:8, RV32:8; `xd`: RV64:3, RV32:3 | `uimm`: 4的倍数, 禁止:0 |
| `c.addiw` | RV64 | 2 | `c.addiw {xd}, {imm}` | `imm`, `xd` | `imm`: RV64:6; `xd`: RV64:5 | `xd`: 禁止:0 |
| `c.addw` | RV64 | 2 | `c.addw {xd}, {xs2}` | `xs2`, `xd` | `xs2`: RV64:3; `xd`: RV64:3 | 无 |
| `c.and` | RV32/64 | 2 | `c.and {xd}, {xs2}` | `xs2`, `xd` | `xs2`: RV32:3, RV64:3; `xd`: RV32:3, RV64:3 | 无 |
| `c.andi` | RV32/64 | 2 | `c.andi {xd}, {imm}` | `imm`, `xd` | `imm`: RV64:6, RV32:6; `xd`: RV32:3, RV64:3 | 无 |
| `c.beqz` | RV32/64 | 2 | `c.beqz {xs1}, {imm}` | `imm`, `xs1` | `imm`: RV64:8, RV32:8; `xs1`: RV64:3, RV32:3 | `imm`: 2的倍数 |
| `c.bnez` | RV32/64 | 2 | `c.bnez {xs1}, {imm}` | `imm`, `xs1` | `imm`: RV64:8, RV32:8; `xs1`: RV64:3, RV32:3 | `imm`: 2的倍数 |
| `c.ebreak` | RV32/64 | 0 | `c.ebreak` | 无 | 无 | 无 |
| `c.j` | RV32/64 | 1 | `c.j {imm}` | `imm` | `imm`: RV32:11, RV64:11 | `imm`: 2的倍数 |
| `c.jal` | RV32 | 1 | `c.jal {imm}` | `imm` | `imm`: RV32:11 | `imm`: 2的倍数 |
| `c.jalr` | RV32/64 | 1 | `c.jalr {xs1}` | `xs1` | `xs1`: RV64:5, RV32:5 | `xs1`: 禁止:0 |
| `c.jr` | RV32/64 | 1 | `c.jr {xs1}` | `xs1` | `xs1`: RV32:5, RV64:5 | `xs1`: 禁止:0 |
| `c.ld` | RV64 | 3 | `c.ld {xd}, {uimm}({xs1})` | `uimm`, `xd`, `xs1` | `uimm`: RV64:5; `xd`: RV64:3; `xs1`: RV64:3 | `uimm`: 8的倍数 |
| `c.ldsp` | RV64 | 2 | `c.ldsp {xd}, {uimm}(sp)` | `uimm`, `xd` | `uimm`: RV64:6; `xd`: RV64:5 | `uimm`: 8的倍数; `xd`: 禁止:0 |
| `c.li` | RV32/64 | 2 | `c.li {xd}, {imm}` | `imm`, `xd` | `imm`: RV32:6, RV64:6; `xd`: RV64:5, RV32:5 | `xd`: 禁止:0 |
| `c.lui` | RV32/64 | 2 | `Rust代码: 略` | `imm`, `xd` | `imm`: RV32:6, RV64:6; `xd`: RV32:5, RV64:5 | `imm`: 范围[-32,31], 禁止:0; `xd`: 禁止:0,2 |
| `c.lw` | RV32/64 | 3 | `c.lw {xd}, {uimm}({xs1})` | `uimm`, `xd`, `xs1` | `uimm`: RV32:5, RV64:5; `xd`: RV32:3, RV64:3; `xs1`: RV64:3, RV32:3 | `uimm`: 4的倍数 |
| `c.lwsp` | RV32/64 | 2 | `c.lwsp {xd}, {uimm}(sp)` | `uimm`, `xd` | `uimm`: RV32:6, RV64:6; `xd`: RV64:5, RV32:5 | `uimm`: 4的倍数; `xd`: 禁止:0 |
| `c.mv` | RV32/64 | 2 | `c.mv {xd}, {xs2}` | `xd`, `xs2` | `xd`: RV32:5, RV64:5; `xs2`: RV64:5, RV32:5 | `xd`: 禁止:0; `xs2`: 禁止:0 |
| `c.nop` | RV32/64 | 0 | `c.nop` | 无 | 无 | 无 |
| `c.or` | RV32/64 | 2 | `c.or {xd}, {xs2}` | `xs2`, `xd` | `xs2`: RV32:3, RV64:3; `xd`: RV32:3, RV64:3 | 无 |
| `c.sd` | RV64 | 3 | `c.sd {xs2}, {uimm}({xs1})` | `uimm`, `xs2`, `xs1` | `uimm`: RV64:5; `xs2`: RV64:3; `xs1`: RV64:3 | `uimm`: 8的倍数 |
| `c.sdsp` | RV64 | 2 | `c.sdsp {xs2}, {uimm}(sp)` | `uimm`, `xs2` | `uimm`: RV64:6; `xs2`: RV64:5 | `uimm`: 8的倍数 |
| `c.slli` | RV32/64 | 2 | `c.slli {xd}, {shamt}` | `shamt`, `xd` | `shamt`: RV64:6, RV32:5; `xd`: RV32:5, RV64:5 | `shamt`: 禁止:0 |
| `c.srai` | RV32/64 | 2 | `c.srai {xd}, {shamt}` | `xd`, `shamt` | `xd`: RV32:3, RV64:3; `shamt`: RV64:6, RV32:5 | `shamt`: 禁止:0 |
| `c.srli` | RV32/64 | 2 | `c.srli {xd}, {shamt}` | `shamt`, `xd` | `shamt`: RV32:5, RV64:6; `xd`: RV64:3, RV32:3 | `shamt`: 禁止:0 |
| `c.sub` | RV32/64 | 2 | `c.sub {xd}, {xs2}` | `xs2`, `xd` | `xs2`: RV64:3, RV32:3; `xd`: RV32:3, RV64:3 | 无 |
| `c.subw` | RV64 | 2 | `c.subw {xd}, {xs2}` | `xs2`, `xd` | `xs2`: RV64:3; `xd`: RV64:3 | 无 |
| `c.sw` | RV32/64 | 3 | `c.sw {xs2}, {uimm}({xs1})` | `uimm`, `xs2`, `xs1` | `uimm`: RV64:5, RV32:5; `xs2`: RV64:3, RV32:3; `xs1`: RV64:3, RV32:3 | `uimm`: 4的倍数 |
| `c.swsp` | RV32/64 | 2 | `c.swsp {xs2}, {uimm}(sp)` | `uimm`, `xs2` | `uimm`: RV32:6, RV64:6; `xs2`: RV64:5, RV32:5 | `uimm`: 4的倍数 |
| `c.xor` | RV32/64 | 2 | `c.xor {xd}, {xs2}` | `xs2`, `xd` | `xs2`: RV32:3, RV64:3; `xd`: RV32:3, RV64:3 | 无 |

---

## 🔧 D 扩展指令

**扩展描述**: 双精度浮点扩展

**指令总数**: 42 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `fadd.d` | RV32/64 | 4 | `fadd.d {fd}, {fs1}, {fs2}, {rm}` | `fs2`, `fs1`, `rm`, `fd` | `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `fd`: RV64:5, RV32:5 | 无 |
| `fclass.d` | RV32/64 | 2 | `fclass.d {xd}, {fs1}` | `fs1`, `xd` | `fs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `fcvt.d.l` | RV64 | 3 | `fcvt.d.l {fd}, {xs1}, {rm}` | `xs1`, `rm`, `fd` | `xs1`: RV64:5; `rm`: RV64:3; `fd`: RV64:5 | 无 |
| `fcvt.d.lu` | RV64 | 3 | `fcvt.d.lu {fd}, {xs1}, {rm}` | `xs1`, `rm`, `fd` | `xs1`: RV64:5; `rm`: RV64:3; `fd`: RV64:5 | 无 |
| `fcvt.d.s` | RV32/64 | 2 | `fcvt.d.s {fd}, {fs1}` | `fs1`, `fd` | `fs1`: RV32:5, RV64:5; `fd`: RV32:5, RV64:5 | 无 |
| `fcvt.d.w` | RV32/64 | 2 | `fcvt.d.w {fd}, {xs1}` | `xs1`, `fd` | `xs1`: RV32:5, RV64:5; `fd`: RV64:5, RV32:5 | 无 |
| `fcvt.d.wu` | RV32/64 | 2 | `fcvt.d.wu {fd}, {xs1}` | `xs1`, `fd` | `xs1`: RV32:5, RV64:5; `fd`: RV32:5, RV64:5 | 无 |
| `fcvt.l.d` | RV64 | 3 | `fcvt.l.d {xd}, {fs1}, {rm}` | `fs1`, `rm`, `xd` | `fs1`: RV64:5; `rm`: RV64:3; `xd`: RV64:5 | 无 |
| `fcvt.lu.d` | RV64 | 3 | `fcvt.lu.d {xd}, {fs1}, {rm}` | `fs1`, `rm`, `xd` | `fs1`: RV64:5; `rm`: RV64:3; `xd`: RV64:5 | 无 |
| `fcvt.s.d` | RV32/64 | 3 | `fcvt.s.d {fd}, {fs1}, {rm}` | `fs1`, `rm`, `fd` | `fs1`: RV64:5, RV32:5; `rm`: RV32:3, RV64:3; `fd`: RV64:5, RV32:5 | 无 |
| `fcvt.w.d` | RV32/64 | 3 | `fcvt.w.d {xd}, {fs1}, {rm}` | `fs1`, `rm`, `xd` | `fs1`: RV64:5, RV32:5; `rm`: RV64:3, RV32:3; `xd`: RV64:5, RV32:5 | 无 |
| `fcvt.wu.d` | RV32/64 | 3 | `fcvt.wu.d {xd}, {fs1}, {rm}` | `fs1`, `rm`, `xd` | `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `xd`: RV64:5, RV32:5 | 无 |
| `fcvtmod.w.d` | RV32/64 | 2 | `fcvtmod.w.d {xd}, {fs1}, rtz` | `fs1`, `xd` | `fs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `fdiv.d` | RV32/64 | 4 | `fdiv.d {fd}, {fs1}, {fs2}, {rm}` | `fs2`, `fs1`, `rm`, `fd` | `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `rm`: RV64:3, RV32:3; `fd`: RV32:5, RV64:5 | 无 |
| `feq.d` | RV32/64 | 3 | `feq.d {xd}, {fs1}, {fs2}` | `fs2`, `fs1`, `xd` | `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `fld` | RV32/64 | 3 | `fld {fd}, {imm}({xs1})` | `imm`, `xs1`, `fd` | `imm`: RV32:12, RV64:12; `xs1`: RV64:5, RV32:5; `fd`: RV32:5, RV64:5 | 无 |
| `fle.d` | RV32/64 | 3 | `fle.d {xd}, {fs1}, {fs2}` | `fs2`, `fs1`, `xd` | `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `fleq.d` | RV32/64 | 3 | `fleq.d {xd}, {fs1}, {fs2}` | `fs2`, `fs1`, `xd` | `fs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `fli.d` | RV32/64 | 2 | `fli.d {fd}, {uimm}` | `uimm`, `fd` | `uimm`: RV32:5, RV64:5; `fd`: RV32:5, RV64:5 | 无 |
| `flt.d` | RV32/64 | 3 | `flt.d {xd}, {fs1}, {fs2}` | `fs2`, `fs1`, `xd` | `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `fltq.d` | RV32/64 | 3 | `fltq.d {xd}, {fs1}, {fs2}` | `fs2`, `fs1`, `xd` | `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `fmadd.d` | RV32/64 | 5 | `fmadd.d {fd}, {fs1}, {fs2}, {fs3}, {rm}` | `fs3`, `fs2`, `fs1`, `rm`, `fd` | `fs3`: RV32:5, RV64:5; `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `fd`: RV32:5, RV64:5 | 无 |
| `fmax.d` | RV32/64 | 3 | `fmax.d {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `fd`: RV64:5, RV32:5 | 无 |
| `fmaxm.d` | RV32/64 | 3 | `fmaxm.d {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `fd`: RV32:5, RV64:5 | 无 |
| `fmin.d` | RV32/64 | 3 | `fmin.d {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `fd`: RV32:5, RV64:5 | 无 |
| `fminm.d` | RV32/64 | 3 | `fminm.d {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `fd`: RV64:5, RV32:5 | 无 |
| `fmsub.d` | RV32/64 | 5 | `fmsub.d {fd}, {fs1}, {fs2}, {fs3}, {rm}` | `fs3`, `fs2`, `fs1`, `rm`, `fd` | `fs3`: RV32:5, RV64:5; `fs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `rm`: RV32:3, RV64:3; `fd`: RV64:5, RV32:5 | 无 |
| `fmul.d` | RV32/64 | 4 | `fmul.d {fd}, {fs1}, {fs2}, {rm}` | `fs2`, `fs1`, `rm`, `fd` | `fs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `rm`: RV64:3, RV32:3; `fd`: RV64:5, RV32:5 | 无 |
| `fmv.d.x` | RV64 | 2 | `fmv.d.x {fd}, {xs1}` | `xs1`, `fd` | `xs1`: RV64:5; `fd`: RV64:5 | 无 |
| `fmv.x.d` | RV64 | 2 | `fmv.x.d {xd}, {fs1}` | `fs1`, `xd` | `fs1`: RV64:5; `xd`: RV64:5 | 无 |
| `fmvh.x.d` | RV32 | 2 | `fmvh.x.d {xd}, {fs1}` | `fs1`, `xd` | `fs1`: RV32:5; `xd`: RV32:5 | 无 |
| `fmvp.d.x` | RV32 | 3 | `fmvp.d.x {fd}, {xs1}, {xs2}` | `xs2`, `xs1`, `fd` | `xs2`: RV32:5; `xs1`: RV32:5; `fd`: RV32:5 | 无 |
| `fnmadd.d` | RV32/64 | 5 | `fnmadd.d {fd}, {fs1}, {fs2}, {fs3}, {rm}` | `fs3`, `fs2`, `fs1`, `rm`, `fd` | `fs3`: RV64:5, RV32:5; `fs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `rm`: RV64:3, RV32:3; `fd`: RV64:5, RV32:5 | 无 |
| `fnmsub.d` | RV32/64 | 5 | `fnmsub.d {fd}, {fs1}, {fs2}, {fs3}, {rm}` | `fs3`, `fs2`, `fs1`, `rm`, `fd` | `fs3`: RV64:5, RV32:5; `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `fd`: RV32:5, RV64:5 | 无 |
| `fround.d` | RV32/64 | 3 | `fround.d {fd}, {fs1}, {rm}` | `fs1`, `rm`, `fd` | `fs1`: RV32:5, RV64:5; `rm`: RV64:3, RV32:3; `fd`: RV64:5, RV32:5 | 无 |
| `froundnx.d` | RV32/64 | 3 | `froundnx.d {fd}, {fs1}, {rm}` | `fs1`, `rm`, `fd` | `fs1`: RV32:5, RV64:5; `rm`: RV64:3, RV32:3; `fd`: RV64:5, RV32:5 | 无 |
| `fsd` | RV32/64 | 3 | `fsd {fs2}, {imm}({xs1})` | `imm`, `fs2`, `xs1` | `imm`: RV32:12, RV64:12; `fs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5 | 无 |
| `fsgnj.d` | RV32/64 | 3 | `fsgnj.d {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `fd`: RV64:5, RV32:5 | 无 |
| `fsgnjn.d` | RV32/64 | 3 | `fsgnjn.d {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `fd`: RV64:5, RV32:5 | 无 |
| `fsgnjx.d` | RV32/64 | 3 | `fsgnjx.d {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `fd`: RV64:5, RV32:5 | 无 |
| `fsqrt.d` | RV32/64 | 3 | `fsqrt.d {fd}, {fs1}, {rm}` | `fs1`, `rm`, `fd` | `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `fd`: RV32:5, RV64:5 | 无 |
| `fsub.d` | RV32/64 | 4 | `fsub.d {fd}, {fs1}, {fs2}, {rm}` | `fs2`, `fs1`, `rm`, `fd` | `fs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `fd`: RV32:5, RV64:5 | 无 |

---

## 🔧 F 扩展指令

**扩展描述**: 单精度浮点扩展

**指令总数**: 37 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `fadd.s` | RV32/64 | 4 | `fadd.s {fd}, {fs1}, {fs2}, {rm}` | `fs2`, `fs1`, `rm`, `fd` | `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `rm`: RV64:3, RV32:3; `fd`: RV32:5, RV64:5 | 无 |
| `fclass.s` | RV32/64 | 2 | `fclass.s {xd}, {fs1}` | `fs1`, `xd` | `fs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `fcvt.l.s` | RV64 | 3 | `fcvt.l.s {xd}, {fs1}, {rm}` | `fs1`, `rm`, `xd` | `fs1`: RV64:5; `rm`: RV64:3; `xd`: RV64:5 | 无 |
| `fcvt.lu.s` | RV64 | 3 | `fcvt.lu.s {xd}, {fs1}, {rm}` | `fs1`, `rm`, `xd` | `fs1`: RV64:5; `rm`: RV64:3; `xd`: RV64:5 | 无 |
| `fcvt.s.l` | RV64 | 3 | `fcvt.s.l {fd}, {xs1}, {rm}` | `xs1`, `rm`, `fd` | `xs1`: RV64:5; `rm`: RV64:3; `fd`: RV64:5 | 无 |
| `fcvt.s.lu` | RV64 | 3 | `fcvt.s.lu {fd}, {xs1}, {rm}` | `xs1`, `rm`, `fd` | `xs1`: RV64:5; `rm`: RV64:3; `fd`: RV64:5 | 无 |
| `fcvt.s.w` | RV32/64 | 3 | `fcvt.s.w {fd}, {xs1}, {rm}` | `xs1`, `rm`, `fd` | `xs1`: RV32:5, RV64:5; `rm`: RV64:3, RV32:3; `fd`: RV32:5, RV64:5 | 无 |
| `fcvt.s.wu` | RV32/64 | 3 | `fcvt.s.wu {fd}, {xs1}, {rm}` | `xs1`, `rm`, `fd` | `xs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `fd`: RV64:5, RV32:5 | 无 |
| `fcvt.w.s` | RV32/64 | 3 | `fcvt.w.s {xd}, {fs1}, {rm}` | `fs1`, `rm`, `xd` | `fs1`: RV64:5, RV32:5; `rm`: RV32:3, RV64:3; `xd`: RV32:5, RV64:5 | 无 |
| `fcvt.wu.s` | RV32/64 | 3 | `fcvt.wu.s {xd}, {fs1}, {rm}` | `fs1`, `rm`, `xd` | `fs1`: RV64:5, RV32:5; `rm`: RV64:3, RV32:3; `xd`: RV32:5, RV64:5 | 无 |
| `fdiv.s` | RV32/64 | 4 | `fdiv.s {fd}, {fs1}, {fs2}, {rm}` | `fs2`, `fs1`, `rm`, `fd` | `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `fd`: RV32:5, RV64:5 | 无 |
| `feq.s` | RV32/64 | 3 | `feq.s {xd}, {fs1}, {fs2}` | `fs2`, `fs1`, `xd` | `fs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `fle.s` | RV32/64 | 3 | `fle.s {xd}, {fs1}, {fs2}` | `fs2`, `fs1`, `xd` | `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `fleq.s` | RV32/64 | 3 | `fleq.s {xd}, {fs1}, {fs2}` | `fs2`, `fs1`, `xd` | `fs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `fli.s` | RV32/64 | 2 | `fli.s {fd}, {uimm}` | `uimm`, `fd` | `uimm`: RV32:5, RV64:5; `fd`: RV32:5, RV64:5 | 无 |
| `flt.s` | RV32/64 | 3 | `flt.s {xd}, {fs1}, {fs2}` | `fs2`, `fs1`, `xd` | `fs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `fltq.s` | RV32/64 | 3 | `fltq.s {xd}, {fs1}, {fs2}` | `fs2`, `fs1`, `xd` | `fs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `flw` | RV32/64 | 3 | `flw {fd}, {imm}({xs1})` | `imm`, `xs1`, `fd` | `imm`: RV64:12, RV32:12; `xs1`: RV32:5, RV64:5; `fd`: RV64:5, RV32:5 | 无 |
| `fmadd.s` | RV32/64 | 5 | `fmadd.s {fd}, {fs1}, {fs2}, {fs3}, {rm}` | `fs3`, `fs2`, `fs1`, `rm`, `fd` | `fs3`: RV64:5, RV32:5; `fs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `rm`: RV32:3, RV64:3; `fd`: RV64:5, RV32:5 | 无 |
| `fmax.s` | RV32/64 | 3 | `fmax.s {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `fd`: RV64:5, RV32:5 | 无 |
| `fmaxm.s` | RV32/64 | 3 | `fmaxm.s {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `fd`: RV32:5, RV64:5 | 无 |
| `fmin.s` | RV32/64 | 3 | `fmin.s {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `fd`: RV64:5, RV32:5 | 无 |
| `fminm.s` | RV32/64 | 3 | `fminm.s {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `fd`: RV32:5, RV64:5 | 无 |
| `fmsub.s` | RV32/64 | 5 | `fmsub.s {fd}, {fs1}, {fs2}, {fs3}, {rm}` | `fs3`, `fs2`, `fs1`, `rm`, `fd` | `fs3`: RV32:5, RV64:5; `fs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `fd`: RV64:5, RV32:5 | 无 |
| `fmul.s` | RV32/64 | 4 | `fmul.s {fd}, {fs1}, {fs2}, {rm}` | `fs2`, `fs1`, `rm`, `fd` | `fs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `rm`: RV64:3, RV32:3; `fd`: RV64:5, RV32:5 | 无 |
| `fmv.w.x` | RV32/64 | 2 | `fmv.w.x {fd}, {xs1}` | `xs1`, `fd` | `xs1`: RV32:5, RV64:5; `fd`: RV32:5, RV64:5 | 无 |
| `fmv.x.w` | RV32/64 | 2 | `fmv.x.w {xd}, {fs1}` | `fs1`, `xd` | `fs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `fnmadd.s` | RV32/64 | 5 | `fnmadd.s {fd}, {fs1}, {fs2}, {fs3}, {rm}` | `fs3`, `fs2`, `fs1`, `rm`, `fd` | `fs3`: RV64:5, RV32:5; `fs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `rm`: RV32:3, RV64:3; `fd`: RV64:5, RV32:5 | 无 |
| `fnmsub.s` | RV32/64 | 5 | `fnmsub.s {fd}, {fs1}, {fs2}, {fs3}, {rm}` | `fs3`, `fs2`, `fs1`, `rm`, `fd` | `fs3`: RV64:5, RV32:5; `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `fd`: RV32:5, RV64:5 | 无 |
| `fround.s` | RV32/64 | 3 | `fround.s {fd}, {fs1}, {rm}` | `fs1`, `rm`, `fd` | `fs1`: RV64:5, RV32:5; `rm`: RV32:3, RV64:3; `fd`: RV64:5, RV32:5 | 无 |
| `froundnx.s` | RV32/64 | 3 | `froundnx.s {fd}, {fs1}, {rm}` | `fs1`, `rm`, `fd` | `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `fd`: RV32:5, RV64:5 | 无 |
| `fsgnj.s` | RV32/64 | 3 | `fsgnj.s {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `fd`: RV32:5, RV64:5 | 无 |
| `fsgnjn.s` | RV32/64 | 3 | `fsgnjn.s {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `fd`: RV64:5, RV32:5 | 无 |
| `fsgnjx.s` | RV32/64 | 3 | `fsgnjx.s {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `fd`: RV32:5, RV64:5 | 无 |
| `fsqrt.s` | RV32/64 | 3 | `fsqrt.s {fd}, {fs1}, {rm}` | `fs1`, `rm`, `fd` | `fs1`: RV64:5, RV32:5; `rm`: RV64:3, RV32:3; `fd`: RV32:5, RV64:5 | 无 |
| `fsub.s` | RV32/64 | 4 | `fsub.s {fd}, {fs1}, {fs2}, {rm}` | `fs2`, `fs1`, `rm`, `fd` | `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `rm`: RV64:3, RV32:3; `fd`: RV64:5, RV32:5 | 无 |
| `fsw` | RV32/64 | 3 | `fsw {fs2}, {imm}({xs1})` | `imm`, `fs2`, `xs1` | `imm`: RV32:12, RV64:12; `fs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5 | 无 |

---

## 🔧 H 扩展指令

**扩展描述**: 虚拟化扩展

**指令总数**: 15 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `hfence.gvma` | RV32/64 | 2 | `hfence.gvma {xs1}, {xs2}` | `xs2`, `xs1` | `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5 | 无 |
| `hfence.vvma` | RV32/64 | 2 | `hfence.vvma {xs1}, {xs2}` | `xs2`, `xs1` | `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5 | 无 |
| `hlv.b` | RV32/64 | 2 | `hlv.b {xd}, 0({xs1})` | `xs1`, `xd` | `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `hlv.bu` | RV32/64 | 2 | `hlv.bu {xd}, 0({xs1})` | `xs1`, `xd` | `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `hlv.d` | RV64 | 2 | `hlv.d {xd}, 0({xs1})` | `xs1`, `xd` | `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `hlv.h` | RV32/64 | 2 | `hlv.h {xd}, 0({xs1})` | `xs1`, `xd` | `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `hlv.hu` | RV32/64 | 2 | `hlv.hu {xd}, 0({xs1})` | `xs1`, `xd` | `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `hlv.w` | RV32/64 | 2 | `hlv.w {xd}, 0({xs1})` | `xs1`, `xd` | `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `hlv.wu` | RV64 | 2 | `hlv.wu {xd}, 0({xs1})` | `xs1`, `xd` | `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `hlvx.hu` | RV32/64 | 2 | `hlvx.hu {xd}, 0({xs1})` | `xs1`, `xd` | `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `hlvx.wu` | RV32/64 | 2 | `hlvx.wu {xd}, 0({xs1})` | `xs1`, `xd` | `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `hsv.b` | RV32/64 | 2 | `hsv.b {xs2}, 0({xs1})` | `xs2`, `xs1` | `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5 | 无 |
| `hsv.d` | RV64 | 2 | `hsv.d {xs2}, 0({xs1})` | `xs2`, `xs1` | `xs2`: RV64:5; `xs1`: RV64:5 | 无 |
| `hsv.h` | RV32/64 | 2 | `hsv.h {xs2}, 0({xs1})` | `xs2`, `xs1` | `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5 | 无 |
| `hsv.w` | RV32/64 | 2 | `hsv.w {xs2}, 0({xs1})` | `xs2`, `xs1` | `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5 | 无 |

---

## 🔧 I 扩展指令

**扩展描述**: 基本整数指令集

**指令总数**: 55 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `add` | RV32/64 | 3 | `add {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `addi` | RV32/64 | 3 | `addi {xd}, {xs1}, {imm}` | `imm`, `xs1`, `xd` | `imm`: RV64:12, RV32:12; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `addiw` | RV64 | 3 | `addiw {xd}, {xs1}, {imm}` | `imm`, `xs1`, `xd` | `imm`: RV64:12; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `addw` | RV64 | 3 | `addw {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `and` | RV32/64 | 3 | `and {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `andi` | RV32/64 | 3 | `andi {xd}, {xs1}, {imm}` | `imm`, `xs1`, `xd` | `imm`: RV32:12, RV64:12; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `auipc` | RV32/64 | 2 | `auipc {xd}, {uimm}` | `uimm`, `xd` | `uimm`: RV64:20, RV32:20; `xd`: RV64:5, RV32:5 | `uimm`: 4096的倍数 |
| `beq` | RV32/64 | 3 | `beq {xs1}, {xs2}, {imm}` | `imm`, `xs2`, `xs1` | `imm`: RV64:12, RV32:12; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5 | `imm`: 2的倍数 |
| `bge` | RV32/64 | 3 | `bge {xs1}, {xs2}, {imm}` | `imm`, `xs2`, `xs1` | `imm`: RV64:12, RV32:12; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5 | `imm`: 2的倍数 |
| `bgeu` | RV32/64 | 3 | `bgeu {xs1}, {xs2}, {imm}` | `imm`, `xs2`, `xs1` | `imm`: RV32:12, RV64:12; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5 | `imm`: 2的倍数 |
| `blt` | RV32/64 | 3 | `blt {xs1}, {xs2}, {imm}` | `imm`, `xs2`, `xs1` | `imm`: RV32:12, RV64:12; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5 | `imm`: 2的倍数 |
| `bltu` | RV32/64 | 3 | `bltu {xs1}, {xs2}, {imm}` | `imm`, `xs2`, `xs1` | `imm`: RV32:12, RV64:12; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5 | `imm`: 2的倍数 |
| `bne` | RV32/64 | 3 | `bne {xs1}, {xs2}, {imm}` | `imm`, `xs2`, `xs1` | `imm`: RV32:12, RV64:12; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5 | `imm`: 2的倍数 |
| `ebreak` | RV32/64 | 0 | `ebreak` | 无 | 无 | 无 |
| `ecall` | RV32/64 | 0 | `ecall` | 无 | 无 | 无 |
| `fence` | RV32/64 | 5 | `fence {pred}, {succ}` | `fm`, `pred`, `succ`, `xs1`, `xd` | `fm`: RV64:4, RV32:4; `pred`: RV64:4, RV32:4; `succ`: RV64:4, RV32:4; `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `fence.tso` | RV32/64 | 2 | `fence.tso` | `xs1`, `xd` | `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `jal` | RV32/64 | 2 | `jal {xd}, {imm}` | `imm`, `xd` | `imm`: RV32:20, RV64:20; `xd`: RV32:5, RV64:5 | `imm`: 2的倍数 |
| `jalr` | RV32/64 | 3 | `jalr {xd}, {imm}({xs1})` | `imm`, `xs1`, `xd` | `imm`: RV64:12, RV32:12; `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `lb` | RV32/64 | 3 | `lb {xd}, {imm}({xs1})` | `imm`, `xs1`, `xd` | `imm`: RV64:12, RV32:12; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `lbu` | RV32/64 | 3 | `lbu {xd}, {imm}({xs1})` | `imm`, `xs1`, `xd` | `imm`: RV32:12, RV64:12; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `ld` | RV64 | 3 | `ld {xd}, {imm}({xs1})` | `imm`, `xs1`, `xd` | `imm`: RV64:12; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `lh` | RV32/64 | 3 | `lh {xd}, {imm}({xs1})` | `imm`, `xs1`, `xd` | `imm`: RV64:12, RV32:12; `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `lhu` | RV32/64 | 3 | `lhu {xd}, {imm}({xs1})` | `imm`, `xs1`, `xd` | `imm`: RV32:12, RV64:12; `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `lui` | RV32/64 | 2 | `lui {xd}, {uimm}` | `uimm`, `xd` | `uimm`: RV64:20, RV32:20; `xd`: RV64:5, RV32:5 | `uimm`: 4096的倍数 |
| `lw` | RV32/64 | 3 | `lw {xd}, {imm}({xs1})` | `imm`, `xs1`, `xd` | `imm`: RV64:12, RV32:12; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `lwu` | RV64 | 3 | `lwu {xd}, {imm}({xs1})` | `imm`, `xs1`, `xd` | `imm`: RV64:12; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `mret` | RV32/64 | 0 | `mret` | 无 | 无 | 无 |
| `or` | RV32/64 | 3 | `or {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `ori` | RV32/64 | 3 | `ori {xd}, {xs1}, {imm}` | `imm`, `xs1`, `xd` | `imm`: RV32:12, RV64:12; `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `sb` | RV32/64 | 3 | `sb {xs2}, {imm}({xs1})` | `imm`, `xs2`, `xs1` | `imm`: RV64:12, RV32:12; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5 | 无 |
| `sd` | RV64 | 3 | `sd {xs2}, {imm}({xs1})` | `imm`, `xs1`, `xs2` | `imm`: RV64:12; `xs1`: RV64:5; `xs2`: RV64:5 | 无 |
| `sh` | RV32/64 | 3 | `sh {xs2}, {imm}({xs1})` | `imm`, `xs2`, `xs1` | `imm`: RV64:12, RV32:12; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5 | 无 |
| `sll` | RV32/64 | 3 | `sll {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `slli` | RV32/64 | 3 | `slli {xd}, {xs1}, {shamt}` | `xd`, `shamt`, `xs1` | `xd`: RV64:5, RV32:5; `shamt`: RV64:6, RV32:5; `xs1`: RV32:5, RV64:5 | 无 |
| `slliw` | RV64 | 3 | `slliw {xd}, {xs1}, {shamt}` | `shamt`, `xs1`, `xd` | `shamt`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `sllw` | RV64 | 3 | `sllw {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `slt` | RV32/64 | 3 | `slt {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `slti` | RV32/64 | 3 | `slti {xd}, {xs1}, {imm}` | `imm`, `xs1`, `xd` | `imm`: RV32:12, RV64:12; `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `sltiu` | RV32/64 | 3 | `sltiu {xd}, {xs1}, {imm}` | `imm`, `xs1`, `xd` | `imm`: RV64:12, RV32:12; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `sltu` | RV32/64 | 3 | `sltu {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `sra` | RV32/64 | 3 | `sra {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `srai` | RV32/64 | 3 | `srai {xd}, {xs1}, {shamt}` | `xd`, `xs1`, `shamt` | `xd`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `shamt`: RV32:5, RV64:6 | 无 |
| `sraiw` | RV64 | 3 | `sraiw {xd}, {xs1}, {shamt}` | `shamt`, `xs1`, `xd` | `shamt`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `sraw` | RV64 | 3 | `sraw {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `srl` | RV32/64 | 3 | `srl {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `srli` | RV32/64 | 3 | `srli {xd}, {xs1}, {shamt}` | `shamt`, `xd`, `xs1` | `shamt`: RV64:6, RV32:5; `xd`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5 | 无 |
| `srliw` | RV64 | 3 | `srliw {xd}, {xs1}, {shamt}` | `shamt`, `xs1`, `xd` | `shamt`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `srlw` | RV64 | 3 | `srlw {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `sub` | RV32/64 | 3 | `sub {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `subw` | RV64 | 3 | `subw {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `sw` | RV32/64 | 3 | `sw {xs2}, {imm}({xs1})` | `imm`, `xs2`, `xs1` | `imm`: RV64:12, RV32:12; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5 | 无 |
| `wfi` | RV32/64 | 0 | `wfi` | 无 | 无 | 无 |
| `xor` | RV32/64 | 3 | `xor {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `xori` | RV32/64 | 3 | `xori {xd}, {xs1}, {imm}` | `imm`, `xs1`, `xd` | `imm`: RV64:12, RV32:12; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |

---

## 🔧 M 扩展指令

**扩展描述**: 乘法除法扩展

**指令总数**: 13 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `div` | RV32/64 | 3 | `div {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `divu` | RV32/64 | 3 | `divu {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `divuw` | RV64 | 3 | `divuw {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `divw` | RV64 | 3 | `divw {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `mul` | RV32/64 | 3 | `mul {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `mulh` | RV32/64 | 3 | `mulh {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `mulhsu` | RV32/64 | 3 | `mulhsu {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `mulhu` | RV32/64 | 3 | `mulhu {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `mulw` | RV64 | 3 | `mulw {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `rem` | RV32/64 | 3 | `rem {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `remu` | RV32/64 | 3 | `remu {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `remuw` | RV64 | 3 | `remuw {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `remw` | RV64 | 3 | `remw {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |

---

## 🔧 Q 扩展指令

**扩展描述**: 四精度浮点扩展

**指令总数**: 43 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `fadd.q` | RV32/64 | 4 | `fadd.q {fd}, {fs1}, {fs2}, {rm}` | `fs2`, `fs1`, `rm`, `fd` | `fs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `rm`: RV64:3, RV32:3; `fd`: RV64:5, RV32:5 | 无 |
| `fclass.q` | RV32/64 | 2 | `fclass.q {xd}, {fs1}` | `fs1`, `xd` | `fs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `fcvt.d.q` | RV32/64 | 3 | `fcvt.d.q {fd}, {fs1}, {rm}` | `fs1`, `rm`, `fd` | `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `fd`: RV64:5, RV32:5 | 无 |
| `fcvt.h.q` | RV32/64 | 3 | `fcvt.h.q {fd}, {fs1}, {rm}` | `fs1`, `rm`, `fd` | `fs1`: RV32:5, RV64:5; `rm`: RV64:3, RV32:3; `fd`: RV64:5, RV32:5 | 无 |
| `fcvt.l.q` | RV64 | 3 | `fcvt.l.q {xd}, {fs1}, {rm}` | `fs1`, `rm`, `xd` | `fs1`: RV64:5; `rm`: RV64:3; `xd`: RV64:5 | 无 |
| `fcvt.lu.q` | RV64 | 3 | `fcvt.lu.q {xd}, {fs1}, {rm}` | `fs1`, `rm`, `xd` | `fs1`: RV64:5; `rm`: RV64:3; `xd`: RV64:5 | 无 |
| `fcvt.q.d` | RV32/64 | 2 | `fcvt.q.d {fd}, {fs1}` | `fs1`, `fd` | `fs1`: RV64:5, RV32:5; `fd`: RV32:5, RV64:5 | 无 |
| `fcvt.q.h` | RV32/64 | 2 | `fcvt.q.h {fd}, {fs1}` | `fs1`, `fd` | `fs1`: RV64:5, RV32:5; `fd`: RV32:5, RV64:5 | 无 |
| `fcvt.q.l` | RV64 | 3 | `fcvt.q.l {fd}, {xs1}, {rm}` | `xs1`, `rm`, `fd` | `xs1`: RV64:5; `rm`: RV64:3; `fd`: RV64:5 | 无 |
| `fcvt.q.lu` | RV64 | 3 | `fcvt.q.lu {fd}, {xs1}, {rm}` | `xs1`, `rm`, `fd` | `xs1`: RV64:5; `rm`: RV64:3; `fd`: RV64:5 | 无 |
| `fcvt.q.s` | RV32/64 | 2 | `fcvt.q.s {fd}, {fs1}` | `fs1`, `fd` | `fs1`: RV64:5, RV32:5; `fd`: RV64:5, RV32:5 | 无 |
| `fcvt.q.w` | RV32/64 | 2 | `fcvt.q.w {fd}, {xs1}` | `xs1`, `fd` | `xs1`: RV64:5, RV32:5; `fd`: RV32:5, RV64:5 | 无 |
| `fcvt.q.wu` | RV32/64 | 2 | `fcvt.q.wu {fd}, {xs1}` | `xs1`, `fd` | `xs1`: RV32:5, RV64:5; `fd`: RV64:5, RV32:5 | 无 |
| `fcvt.s.q` | RV32/64 | 3 | `fcvt.s.q {fd}, {fs1}, {rm}` | `fs1`, `rm`, `fd` | `fs1`: RV32:5, RV64:5; `rm`: RV64:3, RV32:3; `fd`: RV32:5, RV64:5 | 无 |
| `fcvt.w.q` | RV32/64 | 3 | `fcvt.w.q {xd}, {fs1}, {rm}` | `fs1`, `rm`, `xd` | `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `xd`: RV32:5, RV64:5 | 无 |
| `fcvt.wu.q` | RV32/64 | 3 | `fcvt.wu.q {xd}, {fs1}, {rm}` | `fs1`, `rm`, `xd` | `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `xd`: RV64:5, RV32:5 | 无 |
| `fdiv.q` | RV32/64 | 4 | `fdiv.q {fd}, {fs1}, {fs2}, {rm}` | `fs2`, `fs1`, `rm`, `fd` | `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `fd`: RV64:5, RV32:5 | 无 |
| `feq.q` | RV32/64 | 3 | `feq.q {xd}, {fs1}, {fs2}` | `fs2`, `fs1`, `xd` | `fs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `fle.q` | RV32/64 | 3 | `fle.q {xd}, {fs1}, {fs2}` | `fs2`, `fs1`, `xd` | `fs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `fleq.q` | RV32/64 | 3 | `fleq.q {xd}, {fs1}, {fs2}` | `fs2`, `fs1`, `xd` | `fs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `fli.q` | RV32/64 | 2 | `fli.q {fd}, {uimm}` | `uimm`, `fd` | `uimm`: RV64:5, RV32:5; `fd`: RV64:5, RV32:5 | 无 |
| `flq` | RV32/64 | 3 | `flq {fd}, {imm}({xs1})` | `imm`, `xs1`, `fd` | `imm`: RV32:12, RV64:12; `xs1`: RV32:5, RV64:5; `fd`: RV32:5, RV64:5 | 无 |
| `flt.q` | RV32/64 | 3 | `flt.q {xd}, {fs1}, {fs2}` | `fs2`, `fs1`, `xd` | `fs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `fltq.q` | RV32/64 | 3 | `fltq.q {xd}, {fs1}, {fs2}` | `fs2`, `fs1`, `xd` | `fs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `fmadd.q` | RV32/64 | 5 | `fmadd.q {fd}, {fs1}, {fs2}, {fs3}, {rm}` | `fs3`, `fs2`, `fs1`, `rm`, `fd` | `fs3`: RV32:5, RV64:5; `fs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `rm`: RV32:3, RV64:3; `fd`: RV32:5, RV64:5 | 无 |
| `fmax.q` | RV32/64 | 3 | `fmax.q {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `fd`: RV32:5, RV64:5 | 无 |
| `fmaxm.q` | RV32/64 | 3 | `fmaxm.q {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `fd`: RV32:5, RV64:5 | 无 |
| `fmin.q` | RV32/64 | 3 | `fmin.q {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `fd`: RV32:5, RV64:5 | 无 |
| `fminm.q` | RV32/64 | 3 | `fminm.q {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `fd`: RV64:5, RV32:5 | 无 |
| `fmsub.q` | RV32/64 | 5 | `fmsub.q {fd}, {fs1}, {fs2}, {fs3}, {rm}` | `fs3`, `fs2`, `fs1`, `rm`, `fd` | `fs3`: RV64:5, RV32:5; `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `rm`: RV64:3, RV32:3; `fd`: RV32:5, RV64:5 | 无 |
| `fmul.q` | RV32/64 | 4 | `fmul.q {fd}, {fs1}, {fs2}, {rm}` | `fs2`, `fs1`, `rm`, `fd` | `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `fd`: RV32:5, RV64:5 | 无 |
| `fmvh.x.q` | RV64 | 2 | `fmvh.x.q {xd}, {fs1}` | `fs1`, `xd` | `fs1`: RV64:5; `xd`: RV64:5 | 无 |
| `fmvp.q.x` | RV64 | 3 | `fmvp.q.x {fd}, {xs1}, {xs2}` | `xs2`, `xs1`, `fd` | `xs2`: RV64:5; `xs1`: RV64:5; `fd`: RV64:5 | 无 |
| `fnmadd.q` | RV32/64 | 5 | `fnmadd.q {fd}, {fs1}, {fs2}, {fs3}, {rm}` | `fs3`, `fs2`, `fs1`, `rm`, `fd` | `fs3`: RV32:5, RV64:5; `fs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `rm`: RV64:3, RV32:3; `fd`: RV64:5, RV32:5 | 无 |
| `fnmsub.q` | RV32/64 | 5 | `fnmsub.q {fd}, {fs1}, {fs2}, {fs3}, {rm}` | `fs3`, `fs2`, `fs1`, `rm`, `fd` | `fs3`: RV64:5, RV32:5; `fs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `rm`: RV64:3, RV32:3; `fd`: RV64:5, RV32:5 | 无 |
| `fround.q` | RV32/64 | 3 | `fround.q {fd}, {fs1}, {rm}` | `fs1`, `rm`, `fd` | `fs1`: RV64:5, RV32:5; `rm`: RV64:3, RV32:3; `fd`: RV32:5, RV64:5 | 无 |
| `froundnx.q` | RV32/64 | 3 | `froundnx.q {fd}, {fs1}, {rm}` | `fs1`, `rm`, `fd` | `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `fd`: RV32:5, RV64:5 | 无 |
| `fsgnj.q` | RV32/64 | 3 | `fsgnj.q {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `fd`: RV32:5, RV64:5 | 无 |
| `fsgnjn.q` | RV32/64 | 3 | `fsgnjn.q {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `fd`: RV32:5, RV64:5 | 无 |
| `fsgnjx.q` | RV32/64 | 3 | `fsgnjx.q {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `fd`: RV32:5, RV64:5 | 无 |
| `fsq` | RV32/64 | 3 | `fsq {fs2}, {imm}({xs1})` | `imm`, `fs2`, `xs1` | `imm`: RV64:12, RV32:12; `fs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5 | 无 |
| `fsqrt.q` | RV32/64 | 3 | `fsqrt.q {fd}, {fs1}, {rm}` | `fs1`, `rm`, `fd` | `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `fd`: RV64:5, RV32:5 | 无 |
| `fsub.q` | RV32/64 | 4 | `fsub.q {fd}, {fs1}, {fs2}, {rm}` | `fs2`, `fs1`, `rm`, `fd` | `fs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `rm`: RV32:3, RV64:3; `fd`: RV32:5, RV64:5 | 无 |

---

## 🔧 S 扩展指令

**扩展描述**: 特权架构扩展

**指令总数**: 2 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `sfence.vma` | RV32/64 | 2 | `sfence.vma {xs1}, {xs2}` | `xs2`, `xs1` | `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5 | 无 |
| `sret` | RV32/64 | 0 | `sret` | 无 | 无 | 无 |

---

## 🔧 Sdext 扩展指令

**扩展描述**: 调试扩展

**指令总数**: 1 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `dret` | RV32/64 | 0 | `dret` | 无 | 无 | 无 |

---

## 🔧 Smdbltrp 扩展指令

**扩展描述**: M模式双陷阱扩展

**指令总数**: 1 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `sctrclr` | RV32/64 | 0 | `sctrclr` | 无 | 无 | 无 |

---

## 🔧 Smrnmi 扩展指令

**扩展描述**: M模式可恢复非屏蔽中断扩展

**指令总数**: 1 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `mnret` | RV32/64 | 0 | `mnret` | 无 | 无 | 无 |

---

## 🔧 Svinval 扩展指令

**扩展描述**: 细粒度地址转换缓存无效化扩展

**指令总数**: 5 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `hinval.gvma` | RV32/64 | 2 | `hinval.gvma {xs1}, {xs2}` | `xs2`, `xs1` | `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5 | 无 |
| `hinval.vvma` | RV32/64 | 2 | `hinval.vvma {xs1}, {xs2}` | `xs2`, `xs1` | `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5 | 无 |
| `sfence.inval.ir` | RV32/64 | 0 | `sfence.inval.ir` | 无 | 无 | 无 |
| `sfence.w.inval` | RV32/64 | 0 | `sfence.w.inval` | 无 | 无 | 无 |
| `sinval.vma` | RV32/64 | 2 | `sinval.vma {xs1}, {xs2}` | `xs2`, `xs1` | `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5 | 无 |

---

## 🔧 V 扩展指令

**扩展描述**: 向量扩展

**指令总数**: 627 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `vaadd.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vaadd.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vaaddu.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vaaddu.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vadc.vim` | RV32/64 | 3 | `vadc.vim {vd}, {vs2}, {imm}, v0` | `vs2`, `imm`, `vd` | `vs2`: RV32:5, RV64:5; `imm`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vadc.vvm` | RV32/64 | 3 | `vadc.vvm {vd}, {vs2}, {vs1}, v0` | `vs2`, `vs1`, `vd` | `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vadc.vxm` | RV32/64 | 3 | `vadc.vxm {vd}, {vs2}, {xs1}, v0` | `vs2`, `xs1`, `vd` | `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vadd.vi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `imm`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `imm`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vadd.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vadd.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vand.vi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `imm`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `imm`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vand.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vand.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vasub.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vasub.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vasubu.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vasubu.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vcompress.vm` | RV32/64 | 3 | `vcompress.vm {vd}, {vs2}, {vs1}` | `vs2`, `vs1`, `vd` | `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vcpop.m` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `xd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `vdiv.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vdiv.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vdivu.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vdivu.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfadd.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfadd.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfclass.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfcvt.f.x.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfcvt.f.xu.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfcvt.rtz.x.f.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfcvt.rtz.xu.f.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfcvt.x.f.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfcvt.xu.f.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfdiv.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfdiv.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfirst.m` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `xd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `vfmacc.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfmacc.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfmadd.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfmadd.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfmax.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfmax.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfmerge.vfm` | RV32/64 | 3 | `vfmerge.vfm {vd}, {vs2}, {fs1}, v0` | `vs2`, `fs1`, `vd` | `vs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfmin.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfmin.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfmsac.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfmsac.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfmsub.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfmsub.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfmul.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfmul.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfmv.f.s` | RV32/64 | 2 | `vfmv.f.s {fd}, {vs2}` | `vs2`, `fd` | `vs2`: RV64:5, RV32:5; `fd`: RV64:5, RV32:5 | 无 |
| `vfmv.s.f` | RV32/64 | 2 | `vfmv.s.f {vd}, {fs1}` | `fs1`, `vd` | `fs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfmv.v.f` | RV32/64 | 2 | `vfmv.v.f {vd}, {fs1}` | `fs1`, `vd` | `fs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfncvt.f.f.w` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfncvt.f.x.w` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfncvt.f.xu.w` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfncvt.rod.f.f.w` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfncvt.rtz.x.f.w` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfncvt.rtz.xu.f.w` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfncvt.x.f.w` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfncvt.xu.f.w` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfnmacc.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfnmacc.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfnmadd.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfnmadd.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfnmsac.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfnmsac.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfnmsub.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfnmsub.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfrdiv.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfrec7.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfredmax.vs` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfredmin.vs` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfredosum.vs` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfredusum.vs` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfrsqrt7.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfrsub.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfsgnj.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfsgnj.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfsgnjn.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfsgnjn.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfsgnjx.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfsgnjx.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfslide1down.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfslide1up.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfsqrt.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfsub.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfsub.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfwadd.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfwadd.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfwadd.wf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfwadd.wv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfwcvt.f.f.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfwcvt.f.x.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfwcvt.f.xu.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfwcvt.rtz.x.f.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfwcvt.rtz.xu.f.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfwcvt.x.f.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfwcvt.xu.f.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfwmacc.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfwmacc.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfwmsac.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfwmsac.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfwmul.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfwmul.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfwnmacc.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfwnmacc.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfwnmsac.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfwnmsac.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfwredosum.vs` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfwredusum.vs` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfwsub.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfwsub.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfwsub.wf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfwsub.wv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vid.v` | RV32/64 | 2 | `Rust代码: 略` | `vm`, `vd` | `vm`: RV64:1, RV32:1; `vd`: RV32:5, RV64:5 | 无 |
| `viota.m` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vl1re16.v` | RV32/64 | 2 | `vl1re16.v {vd}, ({xs1})` | `xs1`, `vd` | `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vl1re32.v` | RV32/64 | 2 | `vl1re32.v {vd}, ({xs1})` | `xs1`, `vd` | `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vl1re64.v` | RV32/64 | 2 | `vl1re64.v {vd}, ({xs1})` | `xs1`, `vd` | `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vl1re8.v` | RV32/64 | 2 | `vl1re8.v {vd}, ({xs1})` | `xs1`, `vd` | `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vl2re16.v` | RV32/64 | 2 | `vl2re16.v {vd}, ({xs1})` | `xs1`, `vd` | `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vl2re32.v` | RV32/64 | 2 | `vl2re32.v {vd}, ({xs1})` | `xs1`, `vd` | `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vl2re64.v` | RV32/64 | 2 | `vl2re64.v {vd}, ({xs1})` | `xs1`, `vd` | `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vl2re8.v` | RV32/64 | 2 | `vl2re8.v {vd}, ({xs1})` | `xs1`, `vd` | `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vl4re16.v` | RV32/64 | 2 | `vl4re16.v {vd}, ({xs1})` | `xs1`, `vd` | `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vl4re32.v` | RV32/64 | 2 | `vl4re32.v {vd}, ({xs1})` | `xs1`, `vd` | `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vl4re64.v` | RV32/64 | 2 | `vl4re64.v {vd}, ({xs1})` | `xs1`, `vd` | `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vl4re8.v` | RV32/64 | 2 | `vl4re8.v {vd}, ({xs1})` | `xs1`, `vd` | `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vl8re16.v` | RV32/64 | 2 | `vl8re16.v {vd}, ({xs1})` | `xs1`, `vd` | `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vl8re32.v` | RV32/64 | 2 | `vl8re32.v {vd}, ({xs1})` | `xs1`, `vd` | `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vl8re64.v` | RV32/64 | 2 | `vl8re64.v {vd}, ({xs1})` | `xs1`, `vd` | `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vl8re8.v` | RV32/64 | 2 | `vl8re8.v {vd}, ({xs1})` | `xs1`, `vd` | `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vle16.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vle16ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vle32.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vle32ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vle64.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vle64ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vle8.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vle8ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlm.v` | RV32/64 | 2 | `vlm.v {vd}, ({xs1})` | `xs1`, `vd` | `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vloxei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vloxei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vloxei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vloxei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vloxseg2ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vloxseg2ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vloxseg2ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vloxseg2ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vloxseg3ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vloxseg3ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vloxseg3ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vloxseg3ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vloxseg4ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vloxseg4ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vloxseg4ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vloxseg4ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vloxseg5ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vloxseg5ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vloxseg5ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vloxseg5ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vloxseg6ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vloxseg6ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vloxseg6ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vloxseg6ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vloxseg7ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vloxseg7ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vloxseg7ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vloxseg7ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vloxseg8ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vloxseg8ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vloxseg8ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vloxseg8ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlse16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlse32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlse64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlse8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg2e16.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg2e16ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg2e32.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlseg2e32ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg2e64.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlseg2e64ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlseg2e8.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlseg2e8ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg3e16.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg3e16ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg3e32.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg3e32ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg3e64.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg3e64ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg3e8.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlseg3e8ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlseg4e16.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlseg4e16ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg4e32.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg4e32ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg4e64.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlseg4e64ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg4e8.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlseg4e8ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlseg5e16.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg5e16ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg5e32.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg5e32ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlseg5e64.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg5e64ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg5e8.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg5e8ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlseg6e16.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlseg6e16ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlseg6e32.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg6e32ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg6e64.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg6e64ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlseg6e8.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlseg6e8ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg7e16.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlseg7e16ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlseg7e32.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg7e32ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg7e64.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg7e64ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg7e8.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlseg7e8ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg8e16.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg8e16ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlseg8e32.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg8e32ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlseg8e64.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg8e64ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlseg8e8.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlseg8e8ff.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlsseg2e16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlsseg2e32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlsseg2e64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlsseg2e8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlsseg3e16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlsseg3e32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlsseg3e64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlsseg3e8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlsseg4e16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlsseg4e32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlsseg4e64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlsseg4e8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlsseg5e16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlsseg5e32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlsseg5e64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlsseg5e8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlsseg6e16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlsseg6e32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlsseg6e64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlsseg6e8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlsseg7e16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlsseg7e32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlsseg7e64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlsseg7e8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlsseg8e16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlsseg8e32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vlsseg8e64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vlsseg8e8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vluxei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vluxei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vluxei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vluxei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vluxseg2ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vluxseg2ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vluxseg2ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vluxseg2ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vluxseg3ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vluxseg3ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vluxseg3ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vluxseg3ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vluxseg4ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vluxseg4ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vluxseg4ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vluxseg4ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vluxseg5ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vluxseg5ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vluxseg5ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vluxseg5ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vluxseg6ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vluxseg6ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vluxseg6ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vluxseg6ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vluxseg7ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vluxseg7ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vluxseg7ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vluxseg7ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vluxseg8ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vluxseg8ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vluxseg8ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vluxseg8ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmacc.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmacc.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmadc.vi` | RV32/64 | 3 | `vmadc.vi {vd}, {vs2}, {imm}` | `vs2`, `imm`, `vd` | `vs2`: RV64:5, RV32:5; `imm`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmadc.vim` | RV32/64 | 3 | `vmadc.vim {vd}, {vs2}, {imm}, v0` | `vs2`, `imm`, `vd` | `vs2`: RV32:5, RV64:5; `imm`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmadc.vv` | RV32/64 | 3 | `vmadc.vv {vd}, {vs2}, {vs1}` | `vs2`, `vs1`, `vd` | `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmadc.vvm` | RV32/64 | 3 | `vmadc.vvm {vd}, {vs2}, {vs1}, v0` | `vs2`, `vs1`, `vd` | `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmadc.vx` | RV32/64 | 3 | `vmadc.vx {vd}, {vs2}, {xs1}` | `vs2`, `xs1`, `vd` | `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmadc.vxm` | RV32/64 | 3 | `vmadc.vxm {vd}, {vs2}, {xs1}, v0` | `vs2`, `xs1`, `vd` | `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmadd.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmadd.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmand.mm` | RV32/64 | 3 | `vmand.mm {vd}, {vs2}, {vs1}` | `vs2`, `vs1`, `vd` | `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmandn.mm` | RV32/64 | 3 | `vmandn.mm {vd}, {vs2}, {vs1}` | `vs2`, `vs1`, `vd` | `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmax.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmax.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmaxu.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmaxu.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmerge.vim` | RV32/64 | 3 | `vmerge.vim {vd}, {vs2}, {imm}, v0` | `vs2`, `imm`, `vd` | `vs2`: RV32:5, RV64:5; `imm`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmerge.vvm` | RV32/64 | 3 | `vmerge.vvm {vd}, {vs2}, {vs1}, v0` | `vs2`, `vs1`, `vd` | `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmerge.vxm` | RV32/64 | 3 | `vmerge.vxm {vd}, {vs2}, {xs1}, v0` | `vs2`, `xs1`, `vd` | `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmfeq.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmfeq.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmfge.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmfgt.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmfle.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmfle.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmflt.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmflt.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmfne.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmfne.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmin.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmin.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vminu.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vminu.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmnand.mm` | RV32/64 | 3 | `vmnand.mm {vd}, {vs2}, {vs1}` | `vs2`, `vs1`, `vd` | `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmnor.mm` | RV32/64 | 3 | `vmnor.mm {vd}, {vs2}, {vs1}` | `vs2`, `vs1`, `vd` | `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmor.mm` | RV32/64 | 3 | `vmor.mm {vd}, {vs2}, {vs1}` | `vs2`, `vs1`, `vd` | `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmorn.mm` | RV32/64 | 3 | `vmorn.mm {vd}, {vs2}, {vs1}` | `vs2`, `vs1`, `vd` | `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmsbc.vv` | RV32/64 | 3 | `vmsbc.vv {vd}, {vs2}, {vs1}` | `vs2`, `vs1`, `vd` | `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmsbc.vvm` | RV32/64 | 3 | `vmsbc.vvm {vd}, {vs2}, {vs1}, v0` | `vs2`, `vs1`, `vd` | `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmsbc.vx` | RV32/64 | 3 | `vmsbc.vx {vd}, {vs2}, {xs1}` | `vs2`, `xs1`, `vd` | `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmsbc.vxm` | RV32/64 | 3 | `vmsbc.vxm {vd}, {vs2}, {xs1}, v0` | `vs2`, `xs1`, `vd` | `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmsbf.m` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmseq.vi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `imm`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `imm`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmseq.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmseq.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmsgt.vi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `imm`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `imm`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmsgt.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmsgtu.vi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `imm`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `imm`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmsgtu.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmsif.m` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmsle.vi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `imm`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `imm`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmsle.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmsle.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmsleu.vi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `imm`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `imm`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmsleu.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmsleu.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmslt.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmslt.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmsltu.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmsltu.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmsne.vi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `imm`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `imm`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmsne.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmsne.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmsof.m` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmul.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmul.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmulh.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmulh.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmulhsu.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmulhsu.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmulhu.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmulhu.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmv.s.x` | RV32/64 | 2 | `vmv.s.x {vd}, {xs1}` | `xs1`, `vd` | `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmv.v.i` | RV32/64 | 2 | `vmv.v.i {vd}, {imm}` | `imm`, `vd` | `imm`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmv.v.v` | RV32/64 | 2 | `vmv.v.v {vd}, {vs1}` | `vs1`, `vd` | `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmv.v.x` | RV32/64 | 2 | `vmv.v.x {vd}, {xs1}` | `xs1`, `vd` | `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmv.x.s` | RV32/64 | 2 | `vmv.x.s {xd}, {vs2}` | `vs2`, `xd` | `vs2`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `vmv1r.v` | RV32/64 | 2 | `vmv1r.v {vd}, {vs2}` | `vs2`, `vd` | `vs2`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmv2r.v` | RV32/64 | 2 | `vmv2r.v {vd}, {vs2}` | `vs2`, `vd` | `vs2`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmv4r.v` | RV32/64 | 2 | `vmv4r.v {vd}, {vs2}` | `vs2`, `vd` | `vs2`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmv8r.v` | RV32/64 | 2 | `vmv8r.v {vd}, {vs2}` | `vs2`, `vd` | `vs2`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vmxnor.mm` | RV32/64 | 3 | `vmxnor.mm {vd}, {vs2}, {vs1}` | `vs2`, `vs1`, `vd` | `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vmxor.mm` | RV32/64 | 3 | `vmxor.mm {vd}, {vs2}, {vs1}` | `vs2`, `vs1`, `vd` | `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vnclip.wi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `uimm`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `uimm`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vnclip.wv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vnclip.wx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vnclipu.wi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `uimm`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `uimm`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vnclipu.wv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vnclipu.wx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vnmsac.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vnmsac.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vnmsub.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vnmsub.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vnsra.wi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `uimm`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `uimm`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vnsra.wv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vnsra.wx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vnsrl.wi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `uimm`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `uimm`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vnsrl.wv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vnsrl.wx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vor.vi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `imm`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `imm`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vor.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vor.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vredand.vs` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vredmax.vs` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vredmaxu.vs` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vredmin.vs` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vredminu.vs` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vredor.vs` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vredsum.vs` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vredxor.vs` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vrem.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vrem.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vremu.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vremu.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vrgather.vi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `uimm`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `uimm`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vrgather.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vrgather.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vrgatherei16.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vrsub.vi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `imm`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `imm`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vrsub.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vs1r.v` | RV32/64 | 2 | `vs1r.v {vs3}, 0({xs1})` | `xs1`, `vs3` | `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vs2r.v` | RV32/64 | 2 | `vs2r.v {vs3}, 0({xs1})` | `xs1`, `vs3` | `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vs4r.v` | RV32/64 | 2 | `vs4r.v {vs3}, 0({xs1})` | `xs1`, `vs3` | `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vs8r.v` | RV32/64 | 2 | `vs8r.v {vs3}, 0({xs1})` | `xs1`, `vs3` | `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsadd.vi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `imm`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `imm`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vsadd.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vsadd.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vsaddu.vi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `imm`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `imm`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vsaddu.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vsaddu.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vsbc.vvm` | RV32/64 | 3 | `vsbc.vvm {vd}, {vs2}, {vs1}, v0` | `vs2`, `vs1`, `vd` | `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vsbc.vxm` | RV32/64 | 3 | `vsbc.vxm {vd}, {vs2}, {xs1}, v0` | `vs2`, `xs1`, `vd` | `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vse16.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vse32.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vse64.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vse8.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsetivli` | RV32/64 | 3 | `vsetivli {xd}, {uimm}, {vtypei}` | `vtypei`, `uimm`, `xd` | `vtypei`: RV32:10, RV64:10; `uimm`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `vsetvl` | RV32/64 | 3 | `vsetvl {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `vsetvli` | RV32/64 | 3 | `vsetvli {xd}, {xs1}, {vtypei}` | `vtypei`, `xs1`, `xd` | `vtypei`: RV64:11, RV32:11; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `vsext.vf2` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vsext.vf4` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vsext.vf8` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vslide1down.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vslide1up.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vslidedown.vi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `uimm`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `uimm`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vslidedown.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vslideup.vi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `uimm`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `uimm`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vslideup.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vsll.vi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `uimm`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `uimm`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vsll.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vsll.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vsm.v` | RV32/64 | 2 | `vsm.v {vs3}, ({xs1})` | `xs1`, `vs3` | `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsmul.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vsmul.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vsoxei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsoxei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsoxei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsoxei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsoxseg2ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsoxseg2ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsoxseg2ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsoxseg2ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsoxseg3ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsoxseg3ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsoxseg3ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsoxseg3ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsoxseg4ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsoxseg4ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsoxseg4ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsoxseg4ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsoxseg5ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsoxseg5ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsoxseg5ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsoxseg5ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsoxseg6ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsoxseg6ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsoxseg6ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsoxseg6ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsoxseg7ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsoxseg7ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsoxseg7ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsoxseg7ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsoxseg8ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsoxseg8ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsoxseg8ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsoxseg8ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsra.vi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `uimm`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `uimm`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vsra.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vsra.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vsrl.vi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `uimm`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `uimm`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vsrl.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vsrl.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vsse16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsse32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsse64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsse8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsseg2e16.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsseg2e32.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsseg2e64.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsseg2e8.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsseg3e16.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsseg3e32.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsseg3e64.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsseg3e8.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsseg4e16.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsseg4e32.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsseg4e64.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsseg4e8.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsseg5e16.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsseg5e32.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsseg5e64.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsseg5e8.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsseg6e16.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsseg6e32.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsseg6e64.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsseg6e8.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsseg7e16.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsseg7e32.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsseg7e64.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsseg7e8.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsseg8e16.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsseg8e32.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsseg8e64.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsseg8e8.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vssra.vi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `uimm`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `uimm`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vssra.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vssra.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vssrl.vi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `uimm`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `uimm`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vssrl.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vssrl.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vssseg2e16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vssseg2e32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vssseg2e64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vssseg2e8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vssseg3e16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vssseg3e32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vssseg3e64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vssseg3e8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vssseg4e16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vssseg4e32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vssseg4e64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vssseg4e8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vssseg5e16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vssseg5e32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vssseg5e64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vssseg5e8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vssseg6e16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vssseg6e32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vssseg6e64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vssseg6e8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vssseg7e16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vssseg7e32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vssseg7e64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vssseg7e8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vssseg8e16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vssseg8e32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vssseg8e64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vssseg8e8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `xs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vssub.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vssub.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vssubu.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vssubu.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vsub.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vsub.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vsuxei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsuxei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsuxei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsuxei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsuxseg2ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsuxseg2ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsuxseg2ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsuxseg2ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsuxseg3ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsuxseg3ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsuxseg3ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsuxseg3ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsuxseg4ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsuxseg4ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsuxseg4ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsuxseg4ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsuxseg5ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsuxseg5ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsuxseg5ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsuxseg5ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsuxseg6ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsuxseg6ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsuxseg6ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsuxseg6ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsuxseg7ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsuxseg7ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsuxseg7ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsuxseg7ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsuxseg8ei16.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsuxseg8ei32.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vsuxseg8ei64.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV64:5, RV32:5 | 无 |
| `vsuxseg8ei8.v` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vs3` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vs3`: RV32:5, RV64:5 | 无 |
| `vwadd.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vwadd.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vwadd.wv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vwadd.wx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vwaddu.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vwaddu.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vwaddu.wv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vwaddu.wx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vwmacc.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vwmacc.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vwmaccsu.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vwmaccsu.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vwmaccu.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vwmaccu.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vwmaccus.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vwmul.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vwmul.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vwmulsu.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vwmulsu.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vwmulu.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vwmulu.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vwredsum.vs` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vwredsumu.vs` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vwsub.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vwsub.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vwsub.wv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vwsub.wx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vwsubu.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vwsubu.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vwsubu.wv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vwsubu.wx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vxor.vi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `imm`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `imm`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vxor.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vxor.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vzext.vf2` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vzext.vf4` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vzext.vf8` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |

---

## 🔧 Zaamo 扩展指令

**扩展描述**: 原子内存操作扩展

**指令总数**: 18 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `amoadd.d` | RV64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1; `rl`: RV64:1; `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `amoadd.w` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV32:1, RV64:1; `rl`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `amoand.d` | RV64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1; `rl`: RV64:1; `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `amoand.w` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1, RV32:1; `rl`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `amomax.d` | RV64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1; `rl`: RV64:1; `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `amomax.w` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV32:1, RV64:1; `rl`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `amomaxu.d` | RV64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1; `rl`: RV64:1; `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `amomaxu.w` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1, RV32:1; `rl`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `amomin.d` | RV64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1; `rl`: RV64:1; `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `amomin.w` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1, RV32:1; `rl`: RV64:1, RV32:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `amominu.d` | RV64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1; `rl`: RV64:1; `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `amominu.w` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1, RV32:1; `rl`: RV64:1, RV32:1; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `amoor.d` | RV64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1; `rl`: RV64:1; `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `amoor.w` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1, RV32:1; `rl`: RV64:1, RV32:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `amoswap.d` | RV64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1; `rl`: RV64:1; `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `amoswap.w` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1, RV32:1; `rl`: RV64:1, RV32:1; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `amoxor.d` | RV64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1; `rl`: RV64:1; `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `amoxor.w` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1, RV32:1; `rl`: RV64:1, RV32:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |

---

## 🔧 Zabha 扩展指令

**扩展描述**: 字节和半字原子操作扩展

**指令总数**: 20 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `amoadd.b` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1, RV32:1; `rl`: RV64:1, RV32:1; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `amoadd.h` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV32:1, RV64:1; `rl`: RV64:1, RV32:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `amoand.b` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV32:1, RV64:1; `rl`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `amoand.h` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV32:1, RV64:1; `rl`: RV64:1, RV32:1; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `amocas.b` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1, RV32:1; `rl`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `amocas.h` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1, RV32:1; `rl`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `amomax.b` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1, RV32:1; `rl`: RV64:1, RV32:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `amomax.h` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV32:1, RV64:1; `rl`: RV64:1, RV32:1; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `amomaxu.b` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1, RV32:1; `rl`: RV64:1, RV32:1; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `amomaxu.h` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV32:1, RV64:1; `rl`: RV64:1, RV32:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `amomin.b` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1, RV32:1; `rl`: RV64:1, RV32:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `amomin.h` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1, RV32:1; `rl`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `amominu.b` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV32:1, RV64:1; `rl`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `amominu.h` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV32:1, RV64:1; `rl`: RV64:1, RV32:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `amoor.b` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV32:1, RV64:1; `rl`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `amoor.h` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV32:1, RV64:1; `rl`: RV64:1, RV32:1; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `amoswap.b` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV32:1, RV64:1; `rl`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `amoswap.h` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1, RV32:1; `rl`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `amoxor.b` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV32:1, RV64:1; `rl`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `amoxor.h` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV32:1, RV64:1; `rl`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |

---

## 🔧 Zacas 扩展指令

**扩展描述**: 比较交换原子操作扩展

**指令总数**: 3 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `amocas.d` | RV32/64 | 5 | `Rust代码: 略` | `xs2`, `xs1`, `xd`, `aq`, `rl` | `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5; `aq`: RV32:1, RV64:1; `rl`: RV64:1, RV32:1 | `xs2`: 2的倍数; `xd`: 2的倍数 |
| `amocas.q` | RV64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1; `rl`: RV64:1; `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | `xs2`: 2的倍数; `xd`: 2的倍数 |
| `amocas.w` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV32:1, RV64:1; `rl`: RV32:1, RV64:1; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |

---

## 🔧 Zalasr 扩展指令

**扩展描述**: 加载保留/存储条件扩展

**指令总数**: 8 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `lb.aq` | RV32/64 | 2 | `lb.aq {xd}, ({xs1})` | `xs1`, `xd` | `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `ld.aq` | RV32/64 | 2 | `ld.aq {xd}, ({xs1})` | `xs1`, `xd` | `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `lh.aq` | RV32/64 | 2 | `lh.aq {xd}, ({xs1})` | `xs1`, `xd` | `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `lw.aq` | RV32/64 | 2 | `lw.aq {xd}, ({xs1})` | `xs1`, `xd` | `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `sb.rl` | RV32/64 | 2 | `sb.rl {xs2}, ({xs1})` | `xs2`, `xs1` | `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5 | 无 |
| `sd.rl` | RV32/64 | 2 | `sd.rl {xs2}, ({xs1})` | `xs2`, `xs1` | `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5 | 无 |
| `sh.rl` | RV32/64 | 2 | `sh.rl {xs2}, ({xs1})` | `xs2`, `xs1` | `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5 | 无 |
| `sw.rl` | RV32/64 | 2 | `sw.rl {xs2}, ({xs1})` | `xs2`, `xs1` | `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5 | 无 |

---

## 🔧 Zalrsc 扩展指令

**扩展描述**: LR/SC原子操作扩展

**指令总数**: 4 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `lr.d` | RV64 | 4 | `Rust代码: 略` | `aq`, `rl`, `xs1`, `xd` | `aq`: RV64:1; `rl`: RV64:1; `xs1`: RV64:5; `xd`: RV64:5 | `aq`: 2的倍数; `rl`: 2的倍数 |
| `lr.w` | RV32/64 | 4 | `Rust代码: 略` | `aq`, `rl`, `xs1`, `xd` | `aq`: RV32:1, RV64:1; `rl`: RV32:1, RV64:1; `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | `aq`: 2的倍数; `rl`: 2的倍数 |
| `sc.d` | RV64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1; `rl`: RV64:1; `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `sc.w` | RV32/64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1, RV32:1; `rl`: RV32:1, RV64:1; `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | `aq`: 2的倍数; `rl`: 2的倍数 |

---

## 🔧 Zawrs 扩展指令

**扩展描述**: 等待保留集扩展

**指令总数**: 2 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `wrs.nto` | RV32/64 | 0 | `wrs.nto` | 无 | 无 | 无 |
| `wrs.sto` | RV32/64 | 0 | `wrs.sto` | 无 | 无 | 无 |

---

## 🔧 Zba 扩展指令

**扩展描述**: 地址生成位操作扩展

**指令总数**: 8 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `add.uw` | RV64 | 3 | `add.uw {xd}, {xs1}, {xs2}` | `xd`, `xs1`, `xs2` | `xd`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xs2`: RV32:5, RV64:5 | 无 |
| `sh1add` | RV32/64 | 3 | `sh1add {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `sh1add.uw` | RV64 | 3 | `sh1add.uw {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `sh2add` | RV32/64 | 3 | `sh2add {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `sh2add.uw` | RV64 | 3 | `sh2add.uw {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `sh3add` | RV32/64 | 3 | `sh3add {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `sh3add.uw` | RV64 | 3 | `sh3add.uw {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `slli.uw` | RV64 | 3 | `slli.uw {xd}, {xs1}, {shamt}` | `shamt`, `xs1`, `xd` | `shamt`: RV64:6; `xs1`: RV64:5; `xd`: RV64:5 | 无 |

---

## 🔧 Zbb 扩展指令

**扩展描述**: 基本位操作扩展

**指令总数**: 14 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `clz` | RV32/64 | 2 | `clz {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `clzw` | RV64 | 2 | `clzw {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `cpop` | RV32/64 | 2 | `cpop {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `cpopw` | RV64 | 2 | `cpopw {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `ctz` | RV32/64 | 2 | `ctz {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `ctzw` | RV64 | 2 | `ctzw {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `max` | RV32/64 | 3 | `max {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `maxu` | RV32/64 | 3 | `maxu {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `min` | RV32/64 | 3 | `min {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `minu` | RV32/64 | 3 | `minu {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `orc.b` | RV32/64 | 2 | `orc.b {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `sext.b` | RV32/64 | 2 | `sext.b {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `sext.h` | RV32/64 | 2 | `sext.h {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `zext.h` | RV32/64 | 2 | `zext.h {xd}, {xs1}` | `xd`, `xs1` | `xd`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5 | 无 |

---

## 🔧 Zbc 扩展指令

**扩展描述**: 进位位操作扩展

**指令总数**: 1 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `clmulr` | RV32/64 | 3 | `clmulr {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |

---

## 🔧 Zbkb 扩展指令

**扩展描述**: 位操作加密扩展(基本)

**指令总数**: 6 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `brev8` | RV32/64 | 2 | `brev8 {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `pack` | RV32/64 | 3 | `pack {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `packh` | RV32/64 | 3 | `packh {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `packw` | RV64 | 3 | `packw {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `unzip` | RV32 | 2 | `unzip {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV32:5; `xd`: RV32:5 | 无 |
| `zip` | RV32 | 2 | `zip {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV32:5; `xd`: RV32:5 | 无 |

---

## 🔧 Zbkx 扩展指令

**扩展描述**: 位操作加密扩展(交叉)

**指令总数**: 2 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `xperm4` | RV32/64 | 3 | `xperm4 {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `xperm8` | RV32/64 | 3 | `xperm8 {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |

---

## 🔧 Zbs 扩展指令

**扩展描述**: 单位位操作扩展

**指令总数**: 8 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `bclr` | RV32/64 | 3 | `bclr {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `bclri` | RV32/64 | 3 | `bclri {xd}, {xs1}, {shamt}` | `xd`, `shamt`, `xs1` | `xd`: RV32:5, RV64:5; `shamt`: RV32:5, RV64:6; `xs1`: RV64:5, RV32:5 | 无 |
| `bext` | RV32/64 | 3 | `bext {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `bexti` | RV32/64 | 3 | `bexti {xd}, {xs1}, {shamt}` | `xd`, `shamt`, `xs1` | `xd`: RV32:5, RV64:5; `shamt`: RV32:5, RV64:6; `xs1`: RV32:5, RV64:5 | 无 |
| `binv` | RV32/64 | 3 | `binv {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `binvi` | RV32/64 | 3 | `binvi {xd}, {xs1}, {shamt}` | `shamt`, `xs1`, `xd` | `shamt`: RV64:6, RV32:5; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `bset` | RV32/64 | 3 | `bset {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `bseti` | RV32/64 | 3 | `bseti {xd}, {xs1}, {shamt}` | `xs1`, `xd`, `shamt` | `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5; `shamt`: RV32:5, RV64:6 | 无 |

---

## 🔧 Zcb 扩展指令

**扩展描述**: 压缩基本扩展

**指令总数**: 12 条

### 📦 压缩指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `c.lbu` | RV32/64 | 3 | `c.lbu {xd}, {uimm}({xs1})` | `uimm`, `xd`, `xs1` | `uimm`: RV32:2, RV64:2; `xd`: RV32:3, RV64:3; `xs1`: RV64:3, RV32:3 | 无 |
| `c.lh` | RV32/64 | 3 | `Rust代码: 略` | `uimm`, `xd`, `xs1` | `uimm`: RV64:1, RV32:1; `xd`: RV64:3, RV32:3; `xs1`: RV64:3, RV32:3 | `uimm`: 2的倍数 |
| `c.lhu` | RV32/64 | 3 | `Rust代码: 略` | `uimm`, `xd`, `xs1` | `uimm`: RV64:1, RV32:1; `xd`: RV32:3, RV64:3; `xs1`: RV64:3, RV32:3 | `uimm`: 2的倍数 |
| `c.mul` | RV32/64 | 2 | `c.mul {xd}, {xs2}` | `xd`, `xs2` | `xd`: RV32:3, RV64:3; `xs2`: RV32:3, RV64:3 | 无 |
| `c.not` | RV32/64 | 1 | `c.not {xd}` | `xd` | `xd`: RV32:3, RV64:3 | 无 |
| `c.sb` | RV32/64 | 3 | `c.sb {xs2}, {uimm}({xs1})` | `uimm`, `xs2`, `xs1` | `uimm`: RV32:2, RV64:2; `xs2`: RV64:3, RV32:3; `xs1`: RV64:3, RV32:3 | 无 |
| `c.sext.b` | RV32/64 | 1 | `c.sext.b {xd}` | `xd` | `xd`: RV32:3, RV64:3 | 无 |
| `c.sext.h` | RV32/64 | 1 | `c.sext.h {xd}` | `xd` | `xd`: RV32:3, RV64:3 | 无 |
| `c.sh` | RV32/64 | 3 | `Rust代码: 略` | `uimm`, `xs2`, `xs1` | `uimm`: RV32:1, RV64:1; `xs2`: RV32:3, RV64:3; `xs1`: RV64:3, RV32:3 | `uimm`: 2的倍数 |
| `c.zext.b` | RV32/64 | 1 | `c.zext.b {xd}` | `xd` | `xd`: RV32:3, RV64:3 | 无 |
| `c.zext.h` | RV32/64 | 1 | `c.zext.h {xd}` | `xd` | `xd`: RV32:3, RV64:3 | 无 |
| `c.zext.w` | RV64 | 1 | `c.zext.w {xd}` | `xd` | `xd`: RV64:3 | 无 |

---

## 🔧 Zcd 扩展指令

**扩展描述**: 压缩双精度浮点扩展

**指令总数**: 4 条

### 📦 压缩指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `c.fld` | RV32/64 | 3 | `c.fld {fd}, {uimm}({xs1})` | `uimm`, `fd`, `xs1` | `uimm`: RV32:5, RV64:5; `fd`: RV64:3, RV32:3; `xs1`: RV64:3, RV32:3 | `uimm`: 8的倍数 |
| `c.fldsp` | RV32/64 | 2 | `c.fldsp {fd}, {uimm}(sp)` | `uimm`, `fd` | `uimm`: RV32:6, RV64:6; `fd`: RV32:5, RV64:5 | `uimm`: 8的倍数 |
| `c.fsd` | RV32/64 | 3 | `c.fsd {fs2}, {uimm}({xs1})` | `uimm`, `fs2`, `xs1` | `uimm`: RV32:5, RV64:5; `fs2`: RV32:3, RV64:3; `xs1`: RV64:3, RV32:3 | `uimm`: 8的倍数 |
| `c.fsdsp` | RV32/64 | 2 | `c.fsdsp {fs2}, {uimm}(sp)` | `uimm`, `fs2` | `uimm`: RV32:6, RV64:6; `fs2`: RV64:5, RV32:5 | `uimm`: 8的倍数 |

---

## 🔧 Zcf 扩展指令

**扩展描述**: 压缩单精度浮点扩展

**指令总数**: 4 条

### 📦 压缩指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `c.flw` | RV32 | 3 | `c.flw {fd}, {uimm}({xs1})` | `uimm`, `fd`, `xs1` | `uimm`: RV32:5; `fd`: RV32:3; `xs1`: RV32:3 | `uimm`: 4的倍数 |
| `c.flwsp` | RV32 | 2 | `c.flwsp {fd}, {uimm}(sp)` | `uimm`, `fd` | `uimm`: RV32:6; `fd`: RV32:5 | `uimm`: 4的倍数 |
| `c.fsw` | RV32 | 3 | `c.fsw {fs2}, {uimm}({xs1})` | `uimm`, `fs2`, `xs1` | `uimm`: RV32:5; `fs2`: RV32:3; `xs1`: RV32:3 | `uimm`: 4的倍数 |
| `c.fswsp` | RV32 | 2 | `c.fswsp {fs2}, {uimm}(sp)` | `uimm`, `fs2` | `uimm`: RV32:6; `fs2`: RV32:5 | `uimm`: 4的倍数 |

---

## 🔧 Zcmop 扩展指令

**扩展描述**: 压缩可能操作扩展

**指令总数**: 1 条

### 📦 压缩指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `c.mop.n` | RV32/64 | 1 | `Rust代码: 略` | `n` | `n`: RV32:3, RV64:3 | 无 |

---

## 🔧 Zcmp 扩展指令

**扩展描述**: 压缩指针操作扩展

**指令总数**: 6 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `cm.mva01s` | RV32/64 | 2 | `cm.mva01s {r1s}, {r2s}` | `r1s`, `r2s` | `r1s`: RV32:3, RV64:3; `r2s`: RV64:3, RV32:3 | 无 |
| `cm.mvsa01` | RV32/64 | 1 | `cm.mvsa01 {ne_r1s_r2s}` | `ne_r1s_r2s` | `ne_r1s_r2s`: RV32:6, RV64:6 | 无 |
| `cm.pop` | RV32/64 | 1 | `Rust代码: 略` | `saved_reg_list_with_stack_adj` | `saved_reg_list_with_stack_adj`: RV64:6, RV32:6 | 无 |
| `cm.popret` | RV32/64 | 1 | `Rust代码: 略` | `saved_reg_list_with_stack_adj` | `saved_reg_list_with_stack_adj`: RV64:6, RV32:6 | 无 |
| `cm.popretz` | RV32/64 | 1 | `Rust代码: 略` | `saved_reg_list_with_stack_adj` | `saved_reg_list_with_stack_adj`: RV64:6, RV32:6 | 无 |
| `cm.push` | RV32/64 | 1 | `Rust代码: 略` | `saved_reg_list_with_stack_adj` | `saved_reg_list_with_stack_adj`: RV64:6, RV32:6 | 无 |

---

## 🔧 Zfbfmin 扩展指令

**扩展描述**: 标量BF16转换扩展

**指令总数**: 2 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `fcvt.bf16.s` | RV32/64 | 3 | `fcvt.bf16.s {fd}, {fs1}, {rm}` | `fs1`, `rm`, `fd` | `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `fd`: RV64:5, RV32:5 | 无 |
| `fcvt.s.bf16` | RV32/64 | 2 | `fcvt.s.bf16 {fd}, {fs1}` | `fs1`, `fd` | `fs1`: RV32:5, RV64:5; `fd`: RV32:5, RV64:5 | 无 |

---

## 🔧 Zfh 扩展指令

**扩展描述**: 半精度浮点扩展

**指令总数**: 41 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `fadd.h` | RV32/64 | 4 | `fadd.h {fd}, {fs1}, {fs2}, {rm}` | `fs2`, `fs1`, `rm`, `fd` | `fs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `rm`: RV64:3, RV32:3; `fd`: RV64:5, RV32:5 | 无 |
| `fclass.h` | RV32/64 | 2 | `fclass.h {xd}, {fs1}` | `fs1`, `xd` | `fs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `fcvt.d.h` | RV32/64 | 2 | `fcvt.d.h {fd}, {fs1}` | `fs1`, `fd` | `fs1`: RV64:5, RV32:5; `fd`: RV32:5, RV64:5 | 无 |
| `fcvt.h.d` | RV32/64 | 3 | `fcvt.h.d {fd}, {fs1}, {rm}` | `fs1`, `rm`, `fd` | `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `fd`: RV64:5, RV32:5 | 无 |
| `fcvt.h.l` | RV64 | 3 | `fcvt.h.l {fd}, {xs1}, {rm}` | `xs1`, `rm`, `fd` | `xs1`: RV64:5; `rm`: RV64:3; `fd`: RV64:5 | 无 |
| `fcvt.h.lu` | RV64 | 3 | `fcvt.h.lu {fd}, {xs1}, {rm}` | `xs1`, `rm`, `fd` | `xs1`: RV64:5; `rm`: RV64:3; `fd`: RV64:5 | 无 |
| `fcvt.h.s` | RV32/64 | 3 | `fcvt.h.s {fd}, {fs1}, {rm}` | `fs1`, `rm`, `fd` | `fs1`: RV64:5, RV32:5; `rm`: RV64:3, RV32:3; `fd`: RV64:5, RV32:5 | 无 |
| `fcvt.h.w` | RV32/64 | 3 | `fcvt.h.w {fd}, {xs1}, {rm}` | `xs1`, `rm`, `fd` | `xs1`: RV64:5, RV32:5; `rm`: RV32:3, RV64:3; `fd`: RV64:5, RV32:5 | 无 |
| `fcvt.h.wu` | RV32/64 | 3 | `fcvt.h.wu {fd}, {xs1}, {rm}` | `xs1`, `rm`, `fd` | `xs1`: RV32:5, RV64:5; `rm`: RV64:3, RV32:3; `fd`: RV32:5, RV64:5 | 无 |
| `fcvt.l.h` | RV64 | 3 | `fcvt.l.h {xd}, {fs1}, {rm}` | `fs1`, `rm`, `xd` | `fs1`: RV64:5; `rm`: RV64:3; `xd`: RV64:5 | 无 |
| `fcvt.lu.h` | RV64 | 3 | `fcvt.lu.h {xd}, {fs1}, {rm}` | `fs1`, `rm`, `xd` | `fs1`: RV64:5; `rm`: RV64:3; `xd`: RV64:5 | 无 |
| `fcvt.s.h` | RV32/64 | 2 | `fcvt.s.h {fd}, {fs1}` | `fs1`, `fd` | `fs1`: RV32:5, RV64:5; `fd`: RV32:5, RV64:5 | 无 |
| `fcvt.w.h` | RV32/64 | 3 | `fcvt.w.h {xd}, {fs1}, {rm}` | `fs1`, `rm`, `xd` | `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `xd`: RV64:5, RV32:5 | 无 |
| `fcvt.wu.h` | RV32/64 | 3 | `fcvt.wu.h {xd}, {fs1}, {rm}` | `fs1`, `rm`, `xd` | `fs1`: RV64:5, RV32:5; `rm`: RV32:3, RV64:3; `xd`: RV32:5, RV64:5 | 无 |
| `fdiv.h` | RV32/64 | 4 | `fdiv.h {fd}, {fs1}, {fs2}, {rm}` | `fs2`, `fs1`, `rm`, `fd` | `fs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `rm`: RV64:3, RV32:3; `fd`: RV32:5, RV64:5 | 无 |
| `feq.h` | RV32/64 | 3 | `feq.h {xd}, {fs1}, {fs2}` | `fs2`, `fs1`, `xd` | `fs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `fle.h` | RV32/64 | 3 | `fle.h {xd}, {fs1}, {fs2}` | `fs2`, `fs1`, `xd` | `fs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `fleq.h` | RV32/64 | 3 | `fleq.h {xd}, {fs1}, {fs2}` | `fs2`, `fs1`, `xd` | `fs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `flh` | RV32/64 | 3 | `flh {fd}, {imm}({xs1})` | `imm`, `xs1`, `fd` | `imm`: RV64:12, RV32:12; `xs1`: RV64:5, RV32:5; `fd`: RV32:5, RV64:5 | 无 |
| `fli.h` | RV32/64 | 2 | `fli.h {fd}, {uimm}` | `uimm`, `fd` | `uimm`: RV64:5, RV32:5; `fd`: RV64:5, RV32:5 | 无 |
| `flt.h` | RV32/64 | 3 | `flt.h {xd}, {fs1}, {fs2}` | `fs2`, `fs1`, `xd` | `fs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `fltq.h` | RV32/64 | 3 | `fltq.h {xd}, {fs1}, {fs2}` | `fs2`, `fs1`, `xd` | `fs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `fmadd.h` | RV32/64 | 5 | `fmadd.h {fd}, {fs1}, {fs2}, {fs3}, {rm}` | `fs3`, `fs2`, `fs1`, `rm`, `fd` | `fs3`: RV32:5, RV64:5; `fs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `rm`: RV32:3, RV64:3; `fd`: RV32:5, RV64:5 | 无 |
| `fmax.h` | RV32/64 | 3 | `fmax.h {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `fd`: RV32:5, RV64:5 | 无 |
| `fmaxm.h` | RV32/64 | 3 | `fmaxm.h {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `fd`: RV32:5, RV64:5 | 无 |
| `fmin.h` | RV32/64 | 3 | `fmin.h {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `fd`: RV64:5, RV32:5 | 无 |
| `fminm.h` | RV32/64 | 3 | `fminm.h {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV32:5, RV64:5; `fs1`: RV64:5, RV32:5; `fd`: RV32:5, RV64:5 | 无 |
| `fmsub.h` | RV32/64 | 5 | `fmsub.h {fd}, {fs1}, {fs2}, {fs3}, {rm}` | `fs3`, `fs2`, `fs1`, `rm`, `fd` | `fs3`: RV32:5, RV64:5; `fs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `rm`: RV64:3, RV32:3; `fd`: RV64:5, RV32:5 | 无 |
| `fmul.h` | RV32/64 | 4 | `fmul.h {fd}, {fs1}, {fs2}, {rm}` | `fs2`, `fs1`, `rm`, `fd` | `fs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `rm`: RV64:3, RV32:3; `fd`: RV32:5, RV64:5 | 无 |
| `fmv.h.x` | RV32/64 | 2 | `fmv.h.x {fd}, {xs1}` | `xs1`, `fd` | `xs1`: RV64:5, RV32:5; `fd`: RV64:5, RV32:5 | 无 |
| `fmv.x.h` | RV32/64 | 2 | `fmv.x.h {xd}, {fs1}` | `fs1`, `xd` | `fs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `fnmadd.h` | RV32/64 | 5 | `fnmadd.h {fd}, {fs1}, {fs2}, {fs3}, {rm}` | `fs3`, `fs2`, `fs1`, `rm`, `fd` | `fs3`: RV32:5, RV64:5; `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `rm`: RV64:3, RV32:3; `fd`: RV32:5, RV64:5 | 无 |
| `fnmsub.h` | RV32/64 | 5 | `fnmsub.h {fd}, {fs1}, {fs2}, {fs3}, {rm}` | `fs3`, `fs2`, `fs1`, `rm`, `fd` | `fs3`: RV32:5, RV64:5; `fs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `fd`: RV64:5, RV32:5 | 无 |
| `fround.h` | RV32/64 | 3 | `fround.h {fd}, {fs1}, {rm}` | `fs1`, `rm`, `fd` | `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `fd`: RV32:5, RV64:5 | 无 |
| `froundnx.h` | RV32/64 | 3 | `froundnx.h {fd}, {fs1}, {rm}` | `fs1`, `rm`, `fd` | `fs1`: RV64:5, RV32:5; `rm`: RV32:3, RV64:3; `fd`: RV32:5, RV64:5 | 无 |
| `fsgnj.h` | RV32/64 | 3 | `fsgnj.h {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `fd`: RV64:5, RV32:5 | 无 |
| `fsgnjn.h` | RV32/64 | 3 | `fsgnjn.h {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV64:5, RV32:5; `fs1`: RV64:5, RV32:5; `fd`: RV32:5, RV64:5 | 无 |
| `fsgnjx.h` | RV32/64 | 3 | `fsgnjx.h {fd}, {fs1}, {fs2}` | `fs2`, `fs1`, `fd` | `fs2`: RV64:5, RV32:5; `fs1`: RV32:5, RV64:5; `fd`: RV32:5, RV64:5 | 无 |
| `fsh` | RV32/64 | 3 | `fsh {fs2}, {imm}({xs1})` | `imm`, `xs1`, `fs2` | `imm`: RV64:12, RV32:12; `xs1`: RV32:5, RV64:5; `fs2`: RV64:5, RV32:5 | 无 |
| `fsqrt.h` | RV32/64 | 3 | `fsqrt.h {fd}, {fs1}, {rm}` | `fs1`, `rm`, `fd` | `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `fd`: RV64:5, RV32:5 | 无 |
| `fsub.h` | RV32/64 | 4 | `fsub.h {fd}, {fs1}, {fs2}, {rm}` | `fs2`, `fs1`, `rm`, `fd` | `fs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `rm`: RV32:3, RV64:3; `fd`: RV64:5, RV32:5 | 无 |

---

## 🔧 Zicbom 扩展指令

**扩展描述**: 缓存块管理扩展

**指令总数**: 3 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `cbo.clean` | RV32/64 | 1 | `cbo.clean ({xs1})` | `xs1` | `xs1`: RV64:5, RV32:5 | 无 |
| `cbo.flush` | RV32/64 | 1 | `cbo.flush ({xs1})` | `xs1` | `xs1`: RV64:5, RV32:5 | 无 |
| `cbo.inval` | RV32/64 | 1 | `cbo.inval ({xs1})` | `xs1` | `xs1`: RV32:5, RV64:5 | 无 |

---

## 🔧 Zicboz 扩展指令

**扩展描述**: 缓存块清零扩展

**指令总数**: 1 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `cbo.zero` | RV32/64 | 1 | `cbo.zero ({xs1})` | `xs1` | `xs1`: RV64:5, RV32:5 | 无 |

---

## 🔧 Zicfilp 扩展指令

**扩展描述**: 控制流完整性扩展

**指令总数**: 1 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `lpad` | RV32/64 | 1 | `lpad {uimm}` | `uimm` | `uimm`: RV64:20, RV32:20 | `uimm`: 4096的倍数 |

---

## 🔧 Zicfiss 扩展指令

**扩展描述**: 影子栈扩展

**指令总数**: 7 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `ssamoswap.d` | RV64 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV64:1; `rl`: RV64:1; `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `ssamoswap.w` | RV32 | 5 | `Rust代码: 略` | `aq`, `rl`, `xs2`, `xs1`, `xd` | `aq`: RV32:1; `rl`: RV32:1; `xs2`: RV32:5; `xs1`: RV32:5; `xd`: RV32:5 | 无 |
| `sspopchk.x1` | RV32/64 | 0 | `sspopchk x1` | 无 | 无 | 无 |
| `sspopchk.x5` | RV32/64 | 0 | `sspopchk x5` | 无 | 无 | 无 |
| `sspush.x1` | RV32/64 | 0 | `sspush x1` | 无 | 无 | 无 |
| `sspush.x5` | RV32/64 | 0 | `sspush x5` | 无 | 无 | 无 |
| `ssrdp` | RV32/64 | 1 | `ssrdp {xd}` | `xd` | `xd`: RV32:5, RV64:5 | `xd`: 禁止:0 |

---

## 🔧 Zicond 扩展指令

**扩展描述**: 条件操作扩展

**指令总数**: 2 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `czero.eqz` | RV32/64 | 3 | `czero.eqz {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `czero.nez` | RV32/64 | 3 | `czero.nez {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |

---

## 🔧 Zicsr 扩展指令

**扩展描述**: 控制状态寄存器扩展

**指令总数**: 6 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `csrrc` | RV32/64 | 3 | `csrrc {xd}, {csr}, {xs1}` | `csr`, `xs1`, `xd` | `csr`: RV32:12, RV64:12; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `csrrci` | RV32/64 | 3 | `csrrci {xd}, {csr}, {uimm}` | `csr`, `uimm`, `xd` | `csr`: RV32:12, RV64:12; `uimm`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |
| `csrrs` | RV32/64 | 3 | `csrrs {xd}, {csr}, {xs1}` | `csr`, `xs1`, `xd` | `csr`: RV64:12, RV32:12; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `csrrsi` | RV32/64 | 3 | `csrrsi {xd}, {csr}, {uimm}` | `csr`, `uimm`, `xd` | `csr`: RV32:12, RV64:12; `uimm`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `csrrw` | RV32/64 | 3 | `csrrw {xd}, {csr}, {xs1}` | `csr`, `xs1`, `xd` | `csr`: RV64:12, RV32:12; `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `csrrwi` | RV32/64 | 3 | `csrrwi {xd}, {csr}, {uimm}` | `csr`, `uimm`, `xd` | `csr`: RV32:12, RV64:12; `uimm`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |

---

## 🔧 Zifencei 扩展指令

**扩展描述**: 指令同步扩展

**指令总数**: 1 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `fence.i` | RV32/64 | 3 | `fence.i` | `imm`, `xs1`, `xd` | `imm`: RV64:12, RV32:12; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |

---

## 🔧 Zilsd 扩展指令

**扩展描述**: 负载存储成对扩展

**指令总数**: 2 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `ld` | RV32/64 | 3 | `ld {rd}, {offset}({rs1})` | `rd`, `rs1`, `offset` | `rd`: RV32:5, RV64:5; `rs1`: RV64:5, RV32:5; `offset`: RV32:12, RV64:12 | `rd`: 2的倍数 |
| `sd` | RV32/64 | 3 | `sd {xs2}, {offset}({xs1})` | `xs1`, `xs2`, `offset` | `xs1`: RV64:5, RV32:5; `xs2`: RV64:5, RV32:5; `offset`: RV64:12, RV32:12 | `xs2`: 2的倍数 |

---

## 🔧 Zimop 扩展指令

**扩展描述**: 可能操作扩展

**指令总数**: 2 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `mop.r.n` | RV32/64 | 3 | `Rust代码: 略` | `n`, `xs1`, `xd` | `n`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `mop.rr.n` | RV32/64 | 4 | `Rust代码: 略` | `n`, `xs2`, `xs1`, `xd` | `n`: RV64:3, RV32:3; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xd`: RV32:5, RV64:5 | 无 |

---

## 🔧 Zkn 扩展指令

**扩展描述**: 加密NIST算法扩展

**指令总数**: 2 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `aes64ks1i` | RV64 | 3 | `aes64ks1i {xd}, {xs1}, {rnum}` | `rnum`, `xs1`, `xd` | `rnum`: RV64:4; `xs1`: RV64:5; `xd`: RV64:5 | `rnum`: 范围[0,10] |
| `aes64ks2` | RV64 | 3 | `aes64ks2 {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |

---

## 🔧 Zknd 扩展指令

**扩展描述**: NIST AES解密扩展

**指令总数**: 5 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `aes32dsi` | RV32 | 4 | `aes32dsi {xd}, {xs1}, {xs2}, {bs}` | `bs`, `xs2`, `xs1`, `xd` | `bs`: RV32:2; `xs2`: RV32:5; `xs1`: RV32:5; `xd`: RV32:5 | 无 |
| `aes32dsmi` | RV32 | 4 | `aes32dsmi {xd}, {xs1}, {xs2}, {bs}` | `bs`, `xs2`, `xs1`, `xd` | `bs`: RV32:2; `xs2`: RV32:5; `xs1`: RV32:5; `xd`: RV32:5 | 无 |
| `aes64ds` | RV64 | 3 | `aes64ds {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `aes64dsm` | RV64 | 3 | `aes64dsm {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `aes64im` | RV64 | 2 | `aes64im {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV64:5; `xd`: RV64:5 | 无 |

---

## 🔧 Zkne 扩展指令

**扩展描述**: NIST AES加密扩展

**指令总数**: 4 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `aes32esi` | RV32 | 4 | `aes32esi {xd}, {xs1}, {xs2}, {bs}` | `bs`, `xs2`, `xs1`, `xd` | `bs`: RV32:2; `xs2`: RV32:5; `xs1`: RV32:5; `xd`: RV32:5 | 无 |
| `aes32esmi` | RV32 | 4 | `aes32esmi {xd}, {xs1}, {xs2}, {bs}` | `bs`, `xs2`, `xs1`, `xd` | `bs`: RV32:2; `xs2`: RV32:5; `xs1`: RV32:5; `xd`: RV32:5 | 无 |
| `aes64es` | RV64 | 3 | `aes64es {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `aes64esm` | RV64 | 3 | `aes64esm {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV64:5; `xs1`: RV64:5; `xd`: RV64:5 | 无 |

---

## 🔧 Zknh 扩展指令

**扩展描述**: NIST SHA哈希扩展

**指令总数**: 14 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `sha256sig0` | RV32/64 | 2 | `sha256sig0 {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `sha256sig1` | RV32/64 | 2 | `sha256sig1 {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `sha256sum0` | RV32/64 | 2 | `sha256sum0 {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `sha256sum1` | RV32/64 | 2 | `sha256sum1 {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV64:5, RV32:5; `xd`: RV64:5, RV32:5 | 无 |
| `sha512sig0` | RV64 | 2 | `sha512sig0 {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `sha512sig0h` | RV32 | 3 | `sha512sig0h {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5; `xs1`: RV32:5; `xd`: RV32:5 | 无 |
| `sha512sig0l` | RV32 | 3 | `sha512sig0l {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5; `xs1`: RV32:5; `xd`: RV32:5 | 无 |
| `sha512sig1` | RV64 | 2 | `sha512sig1 {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `sha512sig1h` | RV32 | 3 | `sha512sig1h {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5; `xs1`: RV32:5; `xd`: RV32:5 | 无 |
| `sha512sig1l` | RV32 | 3 | `sha512sig1l {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5; `xs1`: RV32:5; `xd`: RV32:5 | 无 |
| `sha512sum0` | RV64 | 2 | `sha512sum0 {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `sha512sum0r` | RV32 | 3 | `sha512sum0r {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5; `xs1`: RV32:5; `xd`: RV32:5 | 无 |
| `sha512sum1` | RV64 | 2 | `sha512sum1 {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV64:5; `xd`: RV64:5 | 无 |
| `sha512sum1r` | RV32 | 3 | `sha512sum1r {xd}, {xs1}, {xs2}` | `xs2`, `xs1`, `xd` | `xs2`: RV32:5; `xs1`: RV32:5; `xd`: RV32:5 | 无 |

---

## 🔧 Zks 扩展指令

**扩展描述**: 加密ShangMi算法扩展

**指令总数**: 4 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `sm3p0` | RV32/64 | 2 | `sm3p0 {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `sm3p1` | RV32/64 | 2 | `sm3p1 {xd}, {xs1}` | `xs1`, `xd` | `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |
| `sm4ed` | RV32/64 | 4 | `sm4ed {xd}, {xs1}, {xs2}, {bs}` | `bs`, `xs2`, `xs1`, `xd` | `bs`: RV32:2, RV64:2; `xs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `xd`: RV64:5, RV32:5 | 无 |
| `sm4ks` | RV32/64 | 4 | `sm4ks {xd}, {xs1}, {xs2}, {bs}` | `bs`, `xs2`, `xs1`, `xd` | `bs`: RV64:2, RV32:2; `xs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `xd`: RV32:5, RV64:5 | 无 |

---

## 🔧 Zvbb 扩展指令

**扩展描述**: 向量基本位操作扩展

**指令总数**: 16 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `vandn.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vandn.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vbrev.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vbrev8.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vclz.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vcpop.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vctz.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vrev8.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vrol.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vrol.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vror.vi` | RV32/64 | 4 | `Rust代码: 略` | `uimm`, `vm`, `vs2`, `vd` | `uimm`: RV32:6, RV64:6; `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vror.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vror.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vwsll.vi` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `uimm`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `uimm`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vwsll.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vwsll.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |

---

## 🔧 Zvbc 扩展指令

**扩展描述**: 向量进位位操作扩展

**指令总数**: 4 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `vclmul.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vclmul.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vclmulh.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vclmulh.vx` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `xs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV64:5, RV32:5; `xs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |

---

## 🔧 Zvfbfmin 扩展指令

**扩展描述**: 向量BF16转换扩展

**指令总数**: 2 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `vfncvtbf16.f.f.w` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vfwcvtbf16.f.f.v` | RV32/64 | 3 | `Rust代码: 略` | `vm`, `vs2`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |

---

## 🔧 Zvfbfwma 扩展指令

**扩展描述**: 向量BF16乘加扩展

**指令总数**: 2 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `vfwmaccbf16.vf` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `fs1`, `vd` | `vm`: RV64:1, RV32:1; `vs2`: RV32:5, RV64:5; `fs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vfwmaccbf16.vv` | RV32/64 | 4 | `Rust代码: 略` | `vm`, `vs2`, `vs1`, `vd` | `vm`: RV32:1, RV64:1; `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |

---

## 🔧 Zvkg 扩展指令

**扩展描述**: 向量GCM/GMAC扩展

**指令总数**: 2 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `vghsh.vv` | RV32/64 | 3 | `vghsh.vv {vd}, {vs2}, {vs1}` | `vs2`, `vs1`, `vd` | `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vgmul.vv` | RV32/64 | 2 | `vgmul.vv {vd}, {vs2}` | `vs2`, `vd` | `vs2`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |

---

## 🔧 Zvkned 扩展指令

**扩展描述**: 向量NIST AES扩展

**指令总数**: 11 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `vaesdf.vs` | RV32/64 | 2 | `vaesdf.vs {vd}, {vs2}` | `vs2`, `vd` | `vs2`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vaesdf.vv` | RV32/64 | 2 | `vaesdf.vv {vd}, {vs2}` | `vs2`, `vd` | `vs2`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vaesdm.vs` | RV32/64 | 2 | `vaesdm.vs {vd}, {vs2}` | `vs2`, `vd` | `vs2`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vaesdm.vv` | RV32/64 | 2 | `vaesdm.vv {vd}, {vs2}` | `vs2`, `vd` | `vs2`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vaesef.vs` | RV32/64 | 2 | `vaesef.vs {vd}, {vs2}` | `vs2`, `vd` | `vs2`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vaesef.vv` | RV32/64 | 2 | `vaesef.vv {vd}, {vs2}` | `vs2`, `vd` | `vs2`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vaesem.vs` | RV32/64 | 2 | `vaesem.vs {vd}, {vs2}` | `vs2`, `vd` | `vs2`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vaesem.vv` | RV32/64 | 2 | `vaesem.vv {vd}, {vs2}` | `vs2`, `vd` | `vs2`: RV32:5, RV64:5; `vd`: RV32:5, RV64:5 | 无 |
| `vaeskf1.vi` | RV32/64 | 3 | `vaeskf1.vi {vd}, {vs2}, {uimm}` | `vs2`, `uimm`, `vd` | `vs2`: RV64:5, RV32:5; `uimm`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vaeskf2.vi` | RV32/64 | 3 | `vaeskf2.vi {vd}, {vs2}, {uimm}` | `vs2`, `uimm`, `vd` | `vs2`: RV64:5, RV32:5; `uimm`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vaesz.vs` | RV32/64 | 2 | `vaesz.vs {vd}, {vs2}` | `vs2`, `vd` | `vs2`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |

---

## 🔧 Zvknha 扩展指令

**扩展描述**: 向量NIST SHA-2扩展

**指令总数**: 3 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `vsha2ch.vv` | RV32/64 | 3 | `vsha2ch.vv {vd}, {vs2}, {vs1}` | `vs2`, `vs1`, `vd` | `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vsha2cl.vv` | RV32/64 | 3 | `vsha2cl.vv {vd}, {vs2}, {vs1}` | `vs2`, `vs1`, `vd` | `vs2`: RV64:5, RV32:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vsha2ms.vv` | RV32/64 | 3 | `vsha2ms.vv {vd}, {vs2}, {vs1}` | `vs2`, `vs1`, `vd` | `vs2`: RV32:5, RV64:5; `vs1`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |

---

## 🔧 Zvks 扩展指令

**扩展描述**: 向量ShangMi扩展

**指令总数**: 5 条

### 📝 标准指令

| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |
|----------|---------|-----------|----------|------------|-------------------|----------|
| `vsm3c.vi` | RV32/64 | 3 | `vsm3c.vi {vd}, {vs2}, {uimm}` | `vs2`, `uimm`, `vd` | `vs2`: RV32:5, RV64:5; `uimm`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |
| `vsm3me.vv` | RV32/64 | 3 | `vsm3me.vv {vd}, {vs2}, {vs1}` | `vs2`, `vs1`, `vd` | `vs2`: RV32:5, RV64:5; `vs1`: RV64:5, RV32:5; `vd`: RV32:5, RV64:5 | 无 |
| `vsm4k.vi` | RV32/64 | 3 | `vsm4k.vi {vd}, {vs2}, {uimm}` | `vs2`, `uimm`, `vd` | `vs2`: RV32:5, RV64:5; `uimm`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vsm4r.vs` | RV32/64 | 2 | `vsm4r.vs {vd}, {vs2}` | `vs2`, `vd` | `vs2`: RV32:5, RV64:5; `vd`: RV64:5, RV32:5 | 无 |
| `vsm4r.vv` | RV32/64 | 2 | `vsm4r.vv {vd}, {vs2}` | `vs2`, `vd` | `vs2`: RV64:5, RV32:5; `vd`: RV64:5, RV32:5 | 无 |

---

## 🏗️ ISA基础架构兼容性

| ISA基础 | 指令数量 | 相关扩展 |
|---------|----------|----------|
| 仅RV32 | 20 | C, D, Zbkb, ... (共8个) |
| 仅RV64 | 84 | B, C, D, ... (共21个) |
| RV32和RV64 | 1047 | B, C, D, ... (共51个) |

## 📋 操作数使用统计

### 🏷️ 操作数详细统计

| 操作数名称 | 使用次数 | 操作数类型 | 出现在扩展 | 长度分布(RV32/RV64) | 限制条件 |
|------------|----------|------------|------------|-------------------|----------|
| `xs1` | 669 | Integer Register | B, C, D, ... (共41个) | RV32:3,5, RV64:3,5 | 禁止:0 |
| `vm` | 586 | Unsigned Integer | V, Zvbb, Zvbc, ... (共5个) | RV32:1, RV64:1 | 无 |
| `vd` | 532 | Vector Register | V, Zvbb, Zvbc, ... (共9个) | RV32:5, RV64:5 | 无 |
| `vs2` | 480 | Vector Register | V, Zvbb, Zvbc, ... (共9个) | RV32:5, RV64:5 | 无 |
| `xd` | 289 | Integer Register | B, C, D, ... (共32个) | RV32:3,5, RV64:3,5 | 2的倍数; 禁止:0; 禁止:0,2 |
| `xs2` | 230 | Integer Register | B, C, D, ... (共31个) | RV32:3,5, RV64:3,5 | 2的倍数; 禁止:0 |
| `fs1` | 172 | Floating Point Register | D, F, Q, ... (共7个) | RV32:5, RV64:5 | 无 |
| `vs1` | 136 | Vector Register | V, Zvbb, Zvbc, ... (共7个) | RV32:5, RV64:5 | 无 |
| `vs3` | 133 | Vector Register | V | RV32:5, RV64:5 | 无 |
| `fd` | 120 | Floating Point Register | D, F, Q, ... (共8个) | RV32:3,5, RV64:3,5 | 无 |
| `fs2` | 88 | Floating Point Register | D, F, Q, ... (共6个) | RV32:3,5, RV64:3,5 | 无 |
| `rm` | 79 | Round Mode | D, F, Q, ... (共5个) | RV32:3, RV64:3 | 无 |
| `imm` | 63 | Signed Integer | C, D, F, ... (共8个) | RV32:5,6,8,11,12,20, RV64:5,6,8,11,12,20 | 16的倍数, 禁止:0; 2的倍数; 禁止:0; 范围[-32,31], 禁止:0 |
| `uimm` | 51 | Unsigned Integer, FLI Constant | C, D, F, ... (共15个) | RV32:1,2,5,6,8,20, RV64:1,2,5,6,8,20 | 2的倍数; 4096的倍数; 4的倍数; 4的倍数, 禁止:0; 8的倍数 |
| `aq` | 47 | Unsigned Integer | Zaamo, Zabha, Zacas, ... (共5个) | RV32:1, RV64:1 | 2的倍数 |
| `rl` | 47 | Unsigned Integer | Zaamo, Zabha, Zacas, ... (共5个) | RV32:1, RV64:1 | 2的倍数 |
| `shamt` | 16 | Unsigned Integer | B, C, I, ... (共5个) | RV32:5, RV64:5,6 | 禁止:0 |
| `fs3` | 16 | Floating Point Register | D, F, Q, ... (共4个) | RV32:5, RV64:5 | 无 |
| `bs` | 6 | Unsigned Integer | Zknd, Zkne, Zks | RV32:2, RV64:2 | 无 |
| `csr` | 6 | CSR Address | Zicsr | RV32:12, RV64:12 | 无 |
| `saved_reg_list_with_stack_adj` | 4 | Saved Reg List With Stack Adj | Zcmp | RV32:6, RV64:6 | 无 |
| `n` | 3 | Unsigned Integer | Zcmop, Zimop | RV32:3,5, RV64:3,5 | 无 |
| `vtypei` | 2 | Unsigned Integer | V | RV32:10,11, RV64:10,11 | 无 |
| `offset` | 2 | Signed Integer | Zilsd | RV32:12, RV64:12 | 无 |
| `pred` | 1 | Fence Mode | I | RV32:4, RV64:4 | 无 |
| `rnum` | 1 | Unsigned Integer | Zkn | RV32:无, RV64:4 | 范围[0,10] |
| `ne_r1s_r2s` | 1 | Not Equal Compressed Saved Integer Register Pair | Zcmp | RV32:6, RV64:6 | 无 |
| `r2s` | 1 | Saved Integer Register | Zcmp | RV32:3, RV64:3 | 无 |
| `succ` | 1 | Fence Mode | I | RV32:4, RV64:4 | 无 |
| `fm` | 1 | Unsigned Integer | I | RV32:4, RV64:4 | 无 |
| `rs1` | 1 | Integer Register | Zilsd | RV32:5, RV64:5 | 无 |
| `rd` | 1 | Integer Register | Zilsd | RV32:5, RV64:5 | 2的倍数 |
| `r1s` | 1 | Saved Integer Register | Zcmp | RV32:3, RV64:3 | 无 |

### 📐 操作数长度分布统计

| ISA基础 | 位长度 | 使用次数 | 占比 |
|---------|--------|----------|------|
| RV32 | 1 | 657 | 9.0% |
| RV32 | 2 | 8 | 0.1% |
| RV32 | 3 | 110 | 1.5% |
| RV32 | 4 | 3 | 0.0% |
| RV32 | 5 | 2689 | 37.0% |
| RV32 | 6 | 17 | 0.2% |
| RV32 | 8 | 3 | 0.0% |
| RV32 | 10 | 1 | 0.0% |
| RV32 | 11 | 3 | 0.0% |
| RV32 | 12 | 38 | 0.5% |
| RV32 | 20 | 4 | 0.1% |
| RV64 | 1 | 681 | 9.4% |
| RV64 | 2 | 4 | 0.1% |
| RV64 | 3 | 131 | 1.8% |
| RV64 | 4 | 4 | 0.1% |
| RV64 | 5 | 2825 | 38.9% |
| RV64 | 6 | 30 | 0.4% |
| RV64 | 8 | 3 | 0.0% |
| RV64 | 10 | 1 | 0.0% |
| RV64 | 11 | 2 | 0.0% |
| RV64 | 12 | 42 | 0.6% |
| RV64 | 20 | 4 | 0.1% |

### 🚫 操作数限制条件统计

| 限制类型 | 使用次数 | 占受限操作数比例 |
|----------|----------|------------------|
| 范围限制 | 2 | 3.0% |
| 禁止值限制 | 20 | 29.9% |
| 倍数限制 | 47 | 70.1% |

**总操作数**: 3786 个，**受限操作数**: 67 个 (1.8%)

## 📏 按操作数数量分组统计

| 操作数数量 | 指令数量 | 示例指令 |
|-----------|----------|----------|
| 0 | 18 | `sctrclr`, `sret`, `sspush.x1`, `sspopchk.x5`, `sspush.x5`, ... (共18条) |
| 1 | 23 | `c.mop.n`, `cm.pop`, `cm.push`, `cm.mvsa01`, `cm.popretz`, ... (共23条) |
| 2 | 160 | `sha512sum1`, `sha256sig0`, `sha512sig1`, `sha512sum0`, `sha256sum1`, ... (共160条) |
| 3 | 419 | `sha512sig1h`, `sha512sig0l`, `sha512sum1r`, `sha512sig1l`, `sha512sum0r`, ... (共419条) |
| 4 | 469 | `fsub.q`, `fmul.q`, `fdiv.q`, `fadd.q`, `vclmulh.vx`, ... (共469条) |
| 5 | 62 | `fnmadd.q`, `fmsub.q`, `fnmsub.q`, `fmadd.q`, `fmadd.s`, ... (共62条) |

