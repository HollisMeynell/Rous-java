import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.StandardCopyOption
import kotlin.io.path.name
import kotlin.jvm.optionals.getOrNull

plugins {
    kotlin("jvm") version "2.0.0"
    id("com.github.johnrengelman.shadow") version "8.1.1"
    `maven-publish`
}

group = "rosu.pp.jni"
version = "0.1.2"

repositories {
    mavenCentral()
}

dependencies {
    implementation(kotlin("stdlib"))
    testImplementation(kotlin("test"))
    // svg 操作库, 支持导出图
    implementation("org.apache.xmlgraphics:batik-all:1.17")
    // 浏览器操作库
    // https://playwright.dev/java/docs/api/class-page#page-wait-for-url
    implementation("com.microsoft.playwright:playwright:1.45.1")
}

tasks.test {
    useJUnitPlatform()
}

val outPath by lazy {
    val outPath = layout.projectDirectory.dir("src").asFile.toPath()
    Files.find(outPath, 10, { path, _ ->
        path.fileName.endsWith("resources")
    }).findAny().getOrNull() ?: throw Error("Can't find resources directory")
}

fun Path.write(data: ByteArray) {
    Files.write(this, data)
}

task("buildRust") {
    val rosuDir = layout.projectDirectory.dir("rosu").asFile
    // test cargo
    try {
        val cmd = ProcessBuilder("cargo", "build", "--release")
            .directory(rosuDir)
            .start()
        cmd.inputStream.bufferedReader().forEachLine {
            println(it)
        }
        if (cmd.waitFor() != 0) {
            throw Error()
        }
    } catch (e: Exception) {
        throw Error("build rust error", e)
    }
    val outDir = outPath.resolve("lib")
    Files.find(rosuDir.resolve("target").toPath(), 2, {path,_ ->
        path.name.endsWith(".so") || path.name.endsWith(".dll") || path.name.endsWith(".dylib")
    }).forEach {
        Files.copy(it, outDir.resolve(it.fileName), StandardCopyOption.REPLACE_EXISTING)
    }
}

task("git") {
    fun String.execute(): String {
        return ProcessBuilder(this.split(" "))
            .start()
            .inputStream.bufferedReader().readText()
    }
    doLast {
        val id = "git rev-parse --short HEAD".execute().trim()
        outPath.resolve("git.version").write(
            id.toByteArray(Charsets.UTF_8)
        )
    }
}

tasks.named<ProcessResources>("processResources") {
    dependsOn("buildRust", "git")
}

tasks.register<JavaExec>("playwright") {
    classpath = sourceSets["test"].runtimeClasspath
    mainClass.set("com.microsoft.playwright.CLI")
}

tasks.register<JavaExec>("run") {
    project.ext.set("PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD", "1")
    classpath = sourceSets["main"].runtimeClasspath
    mainClass.set("rosu.MainKt")
}

kotlin {
    jvmToolchain(21)
}
