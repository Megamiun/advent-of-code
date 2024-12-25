package br.com.gabryel.adventofcode.util

import kotlin.time.measureTime

fun <T> logTimeSection(section: String, exec: () -> T): T {
    var result: T
    val time = measureTime {
        result = exec()
    }

    println("$section | Took $time")

    return result
}