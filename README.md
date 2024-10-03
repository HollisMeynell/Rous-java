# rosu-jni
---
一个基于 [`rosu-pp`](https://github.com/MaxOhn/rosu-pp) 计算星级以及 pp 的JNI库

# 注意, 此项目不再进行非 bug 处理的维护, 替代项目即将在其他项目中作为子项目提供

## 下载
 [点击访问](https://disk.365246692.xyz/other/rosu-jni-release)
> !! 不带 `all` 的需要自行解决 kotlin 运行时环境, 带 `all` 的打包了 kotlin 运行时 !!

**注意, 对于 x64 平台已提供 rust 编译产物, 其他设备运行需要手动编译,
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


// 收藏夹文件编辑 对应游戏目录下的 collection.db 文件
// 读取已有的文件
val collectionData = Files.readAllBytes(Path("/home/spring/osu/collection.db"))
var collection = OsuDB.readCollection(collectionData)
collection.use {
    // 遍历收藏夹
    for (item in it) {
        println(item.name)
        for (hash in item) {
            println(hash)
        }
    }       
}

// 或者创建空的收藏夹
collection = OsuDB.createCollection()
collection.use {
    // 添加收藏夹
    val collectionItem = it.addCollection("NM1")
    // 添加收藏夹的谱面, 数据是谱面文件的md5
    collectionItem.appendHash("a2aae690e5f98e78fbd64d2ebf4def5f")
    collectionItem.appendHash("a2aae690e5f98e78fbd64d2ebf4def5f")
    // 删改操作
    collectionItem.name = "NM2"
    collectionItem.removeHash(1)
    collectionItem.setHash(0, "a2aae690e5f98e78fbd64d2ebf4def5f")
    collectionItem.insertHash(0, "a2aae690e5f98e78fbd64d2ebf4def5f")

    // 编辑完后导出
    it.toBytes()
}
```
> java 代码
```java

// 读取 osu 文件
byte[] f = Files.readAllBytes(Path.of("F:\\bot\\osufile\\4397861.osu"));

// 直接计算星级
JniScore score = new JniScore();
score.setMode(Mode.Osu);
score.setMods(64);
score.setAccuracy(100D);
score.setMisses(0);
JniResult r = Rosu.calculate(f, score);

// 计算pp
score.setCombo(500);
score.setAccuracy(0.97);
score.setMisses(2);
r = Rosu.calculate(f, score);

// 具体 pp 构成
// 高版本的增强 switch
switch (r) {
      case OsuResult osu -> {
      System.out.println(osu.getPpAim());
      System.out.println(osu.getPpAcc());
      System.out.println(osu.getPpSpeed());
      System.out.println(osu.getPpFlashlight());
      }
      case TaikoResult taiko -> {
      System.out.println(taiko.getPpDifficulty());
      }
      case null, default -> {

      }
}
// 低版本使用 if instanceof
if (r instanceof OsuResult osu) {
    System.out.println(osu.getPpAim());
    System.out.println(osu.getPpAcc());
    System.out.println(osu.getPpSpeed());
    System.out.println(osu.getPpFlashlight());
}

// 渐进计算
try (JniCalculate cal = Rosu.getCalculate(f, new JniMapAttr())) {
    for (int i = 0; i < 1270; i++) {
        cal.getScore().setN300(cal.getScore().getN300() + 1);
        cal.getScore().setCombo(cal.getScore().getCombo() + 2);
        var r = Rosu.calculate(cal);
        System.out.println(r.getPp());
    }
}

// 收藏夹文件编辑 对应游戏目录下的 collection.db 文件
// 读取已有的文件
byte[] collectionData = Files.readAllBytes(Path.of("/home/spring/osu/collection.db"));
    try (OsuCollection collection = OsuDB.readCollection(collectionData)){
        // 遍历收藏夹
        for (OsuCollection.CollectionItem item : collection) {
        System.out.println(item.getName());
        for (String md5 : item) {
            System.out.println("\t" + md5);
        }
        }
    }
// 或者创建空的收藏夹
try (OsuCollection collection = OsuDB.createCollection()){
    // 添加收藏夹
    OsuCollection.CollectionItem item = collection.addCollection("NM1");
    // 添加收藏夹的谱面, 数据是谱面文件的md5
    item.appendHash("a2aae690e5f98e78fbd64d2ebf4def5f");
    item.appendHash("a2aae690e5f98e78fbd64d2ebf4def5f");

    // 删改操作
    item.setName("NM2");
    item.removeHash("a2aae690e5f98e78fbd64d2ebf4def5f");
    item.removeHash(1);
    item.insertHash(0,"a2aae690e5f98e78fbd64d2ebf4def5f");
    item.setHash(0,"a2aae690e5f98e78fbd64d2ebf4def5f");


    // 编辑完后导出
    byte[] out = collection.toBytes();
}
```

## 编译
!! 对于 x86 架构的 linux/windows/mac 已经在 github release 提供了对应的 lib 产物, 可以自行下载
- 编译环境: 
  - jdk, gradle 以及 kotlin 环境, 版本尽可能新, 理论上 jdk 大于 8 即可(但是我是jdk 21)
    - rust 编译环境, 自行安装
  - 网络畅通, 访问 maven 中央仓库以及 github (rust 依赖需要)
- 编译 rosu :
  - `cd rosu` 切换到 rosu 项目目录下
  - `cargo build --release` 编译项目
  - `cp target/release/{目标文件} ../src/main/resources` 复制编译文件到 kotlin 项目中, 其中目标文件是以 .dll/.so/.dylib 结尾的文件
- 编译 rosu-jni :
  - `gradle build` 编译 (适用于项目中已经有 kotlin-stdlib, 比如kotlin项目或者引入kotlin依赖)
  - `gradle shadowJar` 完整编译, jar包中包含 kotlin-stdlib, 但是会导致 jar 包变大
  - 依赖 jar 在目录 `build/libs/rosu-java-x.y.z.jar`

使用 `cross` 借助 docker 进行编译:
```shell
cross build --target=x86_64-pc-windows-gnu --release 
cross build --target=x86_64-unknown-linux-gnu  --release 
```


