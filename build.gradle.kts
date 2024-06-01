plugins {
    kotlin("jvm") version "1.9.23"
    `maven-publish`
}

group = "rosu.pp.jni"
version = "0.1.1"

repositories {
    mavenCentral()
}

dependencies {
    testImplementation(kotlin("test"))
}

tasks.test {
    useJUnitPlatform()
}
kotlin {
    jvmToolchain(21)
}
publishing {
    publications {
        create<MavenPublication>("mavenJava") {
            from(components["java"])
            groupId = "rosu.pp.jni"
            artifactId = "rosu-pp-jni"
            version = "0.1.1"
        }
    }
}
tasks {
    val customJar by creating(Jar::class) {
        archiveBaseName.set("rosu-pp-jni")
        archiveVersion.set("0.1.1")
        from(sourceSets.main.get().output)
        destinationDirectory.set(layout.buildDirectory.dir("libs"))
    }
    "assemble" {
        dependsOn(customJar)
    }
}