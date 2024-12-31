package br.com.gabryel.adventofcode.util

import kotlin.time.measureTimedValue

fun <T> timed(name: String, exec: () -> T): T {
    println("-------------------")
    println("-------------------")
    println(name)
    println("-------------------")
    println("-------------------")

    val (result, timeTaken) = measureTimedValue(exec)

    println()
    println("[$name] Result: $result")
    println("[$name] Took $timeTaken")

    return result
}