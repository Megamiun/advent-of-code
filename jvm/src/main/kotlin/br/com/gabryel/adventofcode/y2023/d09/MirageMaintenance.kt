package br.com.gabryel.adventofcode.y2023.d09

import br.com.gabryel.adventofcode.readLines

fun main() {
    listOf("sample", "input").forEach { file ->
        val lines = readLines(2023, 9, file)
            .map { it.split(" ").map { it.toInt() } }

        println("[Forward ][$file] ${lines.sumOf(::findNextNumberInNextLayer)}")
        println("[Backward][$file] ${lines.sumOf(::findPreviousNumberInNextLayer)}")
    }
}

private fun findNextNumberInNextLayer(layer: List<Int>) =
    findMissingValueInNextLayer(layer) { last() + it }

private fun findPreviousNumberInNextLayer(layer: List<Int>) =
    findMissingValueInNextLayer(layer) { first() - it }

private fun findMissingValueInNextLayer(layer: List<Int>, applying: List<Int>.(Int) -> Int): Int {
    if (layer.all { it == 0 }) return 0

    val nextLayer = layer.windowed(2).map { (l, r) -> r - l }
    val nextLayerMissing = findPreviousNumberInNextLayer(nextLayer)
    return layer.applying(nextLayerMissing)
}
