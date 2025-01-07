import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    kotlin("jvm") version "2.0.20"
}

group = "br.com.gabryel"

repositories {
    mavenCentral()
}

kotlin {
    compilerOptions.jvmTarget = JvmTarget.JVM_21
}

dependencies {
    testImplementation(platform("org.junit:junit-bom:5.11.4"))

    testImplementation("org.hamcrest:hamcrest:3.0")

    testImplementation("org.junit.jupiter:junit-jupiter-api")
    testImplementation("org.junit.jupiter:junit-jupiter-params")
    testRuntimeOnly("org.junit.jupiter:junit-jupiter-engine")
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
}

tasks.test {
    useJUnitPlatform()
    minHeapSize = "1g"
    maxHeapSize = "4g"
}
