package br.com.gabryel.adventofcode.y2023.d10

import br.com.gabryel.adventofcode.util.readLines

fun main() {
    listOf("sample", "sample-2", "sample-3", "sample-4", "sample-5", "sample-6", "sample-7", "sample-8", "input").forEach { file ->
        val map = readLines(2023, 10, file).map { it.toCharArray() }.toTypedArray()

        println("[Biggest Distance  ][$file] ${MaximumDistance(map).maxDistance}")
        println("[Internal Positions][$file] ${countInternalPositions(map)}")
    }
}
