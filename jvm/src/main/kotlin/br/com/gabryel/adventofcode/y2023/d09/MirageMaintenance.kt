package br.com.gabryel.adventofcode.y2023.d09

fun findNextNumberInNextLayer(lines: List<String>) =
    findMissingNumber(lines) { last() + it }

fun findPreviousNumberInNextLayer(lines: List<String>) =
    findMissingNumber(lines) { first() - it }

private fun findMissingNumber(lines: List<String>, getMissing: List<Int>.(Int) -> Int) =
    lines.sumOf { line -> findMissingValueInNextLayer(line.split(" ").map(String::toInt), getMissing) }

private fun findMissingValueInNextLayer(layer: List<Int>, getMissing: List<Int>.(Int) -> Int): Int {
    if (layer.all { it == 0 }) return 0

    val nextLayer = layer.windowed(2).map { (l, r) -> r - l }
    val nextLayerMissing = findMissingValueInNextLayer(nextLayer, getMissing)
    return layer.getMissing(nextLayerMissing)
}
