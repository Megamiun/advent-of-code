package br.com.gabryel.adventofcode.y2023.d05

import br.com.gabryel.adventofcode.y2023.readLines

fun main() {
    listOf("sample", "input").forEach { file ->
        val lines = readLines(5, file, keepBlanks = true)

        println("[Minimal Location       ][$file] ${generateSourceToDestination(lines)}")
        println("[Minimal Location Ranged][$file] ${generateSourceToDestinationRanged(lines)}")
    }
}
