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
    testImplementation("org.hamcrest:hamcrest:3.0")

    testImplementation("org.junit.jupiter:junit-jupiter-api:5.11.4")
    testImplementation("org.junit.jupiter:junit-jupiter-params:5.11.4")
    testRuntimeOnly("org.junit.jupiter:junit-jupiter-engine:5.11.4")

}

tasks.test {
    useJUnitPlatform()
}
