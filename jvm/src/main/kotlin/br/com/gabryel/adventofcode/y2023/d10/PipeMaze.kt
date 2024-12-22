package br.com.gabryel.adventofcode.y2023.d10

import br.com.gabryel.adventofcode.util.readLines

typealias CoordMap = Array<CharArray>

fun main() {
    listOf("sample", "sample2", "sample3", "sample4", "sample5", "sample6", "sample7", "sample8", "input").forEach { file ->
        val map = readLines(2023, 10, file).map { it.toCharArray() }.toTypedArray()

        println("[Biggest Distance  ][$file] ${MaximumDistance(map).maxDistance}")
        println("[Internal Positions][$file] ${countInternalPositions(map)}")
    }
}
