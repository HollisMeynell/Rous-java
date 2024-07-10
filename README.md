# rosu-jni
---
一个基于 [`rosu-pp`](https://github.com/MaxOhn/rosu-pp) 计算星级以及 pp 的JNI库

**注意, 对 windows/linux 的 x64 平台已提供 rust 编译产物, 其他设备运行需要手动编译,
详见下方编译**

## 使用
> kotlin 代码
```kotlin

// 读取 osu 文件
val f = Files.readAllBytes(Path("F:\\bot\\osufile\\4397861.osu"))
// 直接计算星级
var r = Rosu.calculate(
    f, JniScore(
        mode = Mode.Osu,
        mods = 64,
    )
)
println(r.star)

// 计算 pp
r = Rosu.calculate(
    f, JniScore(combo = 500, accuracy = 0.3)
)
r = Rosu.calculate(
    f, JniScore(combo = 500, n100 = 150, n300 = 60, misses = 0)
)

// 渐进计算
val cal = Rosu.getCalculate(f, JniMapAttr())
// java 请使用 try-with-resources, 或者手动调用 .close() 释放 Calculate, 否则会导致内存泄漏
cal.use { c ->
    for (i in 0 until 1270) {
        c.score.n300 += 1
        c.score.combo += 2
        val result = Rosu.calculate(c)
        println(result.pp)
    }
}
```
> java 代码
```java

// 读取 osu 文件
var f = Files.readAllBytes(Path.of("F:\\bot\\osufile\\4397861.osu"));

// 直接计算星级
var score = new JniScore();
score.setMode(Mode.Osu);
score.setMods(64);
score.setAccuracy(100D);
score.setMisses(0);
var r = Rosu.calculate(f, score);

// 计算pp
score.setCombo(500);
score.setAccuracy(0.97);
score.setMisses(2);
r = Rosu.calculate(f, score);

// 渐进计算
try (var cal = Rosu.getCalculate(f, new JniMapAttr())) {
    for (int i = 0; i < 1270; i++) {
        cal.getScore().setN300(cal.getScore().getN300() + 1);
        cal.getScore().setCombo(cal.getScore().getCombo() + 2);
        var r = Rosu.calculate(cal);
        ystem.out.println(r.getPp());
    }
}
```

## 编译
!! 注意: 由于本人没有 mac 设备, 也没有具体研究交叉编译, 所以 rosu 出现编译问题请自行解决
- 编译环境: 
  - jdk, gradle 以及 kotlin 环境, 版本尽可能新, 理论上 jdk 大于 8 即可(但是我是jdk 21)
  - rust 编译环境, 自行安装
  - 网络畅通, 访问 maven 中央仓库以及 github (rust 依赖需要)
- 编译 rosu :
  - `cd rosu` 切换到 rosu 项目目录下
  - `cargo build --release` 编译项目
  - `cp target/release/{目标文件} ../src/main/resources` 复制编译文件到 kotlin 项目中, 其中目标文件是以 .dll/.so/.dylib 结尾的文件
  - linux 下要手动将目标文件重命名, 移除掉开头的 'lib' 字符, 由 `libxxx.so` -> `xxx.so` 
- 编译 rosu-jni :
  - `gradle build` 编译
  - 依赖 jar 在目录 `build/libs/rosu-java-x.y.z.jar`

使用 `cross` 借助 docker 进行编译:
```shell
cross build --target=x86_64-pc-windows-gnu --release 
cross build --target=x86_64-unknown-linux-gnu  --release 
```


