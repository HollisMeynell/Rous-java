import com.github.jengelman.gradle.plugins.shadow.tasks.ShadowJar
import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.StandardCopyOption
import kotlin.io.path.name
import kotlin.jvm.optionals.getOrNull

plugins {
    kotlin("jvm") version "2.0.0"
    id("com.gradleup.shadow") version "8.3.0"
    `maven-publish`
}

group = "rosu.pp.jni"
version = "0.1.5"

repositories {
    mavenCentral()
}

dependencies {
    implementation(kotlin("stdlib"))
    testImplementation(kotlin("test"))
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
    val outDir = outPath.resolve("lib")
    var passable = false
    // if it has any lib file, build is passable
    if (Files.isDirectory(outDir)) {
        Files.newDirectoryStream(outDir).use {
            passable = it.iterator().hasNext()
        }
    }
    // test cargo
    val testResult = try {
        val testCmd = ProcessBuilder("cargo", "--version")
            .directory(rosuDir)
            .start()
        if (testCmd.waitFor() != 0) throw Exception()
        true
    } catch (e: Exception) {
        if (!passable) {
            throw Exception("rust environment not find, can not build.")
        }
        false
    }

    // build lib
    if (testResult) try {
        val cmd = ProcessBuilder("cargo", "build", "--release")
            .directory(rosuDir)
            .start()
        if (cmd.waitFor() != 0) throw Exception()
    } catch (e: Exception) {
        if (!passable) {
            throw Exception("build rust error.", e)
        }
    }

    Files.find(rosuDir.resolve("target").toPath(), 2, { path, _ ->
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
    jvmToolchain(17)
}

publishing {
    publications {
        create<MavenPublication>("maven") {
            groupId = project.group.toString()
            artifactId = "rosu-pp-jni"
            version = project.version.toString()
            from(components["java"])
        }
    }
}

tasks.withType<ShadowJar> {
    minimize()
}