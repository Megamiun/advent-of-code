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
