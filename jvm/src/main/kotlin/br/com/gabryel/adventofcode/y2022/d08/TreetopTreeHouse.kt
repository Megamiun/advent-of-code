package br.com.gabryel.adventofcode.y2022.d08

import br.com.gabryel.adventofcode.util.takeWhileInclusive

fun getVisibilityFromOutside(map: List<String>) =
    map.getForest().rayTraceFromOutside().sumOf { it.count { it } }

fun getVisibilityFromInside(map: List<String>) =
    map.getForest().rayTraceFromInside()

private fun List<List<Int>>.rayTraceFromOutside(): List<List<Boolean>> {
    val visibleNodes = map { it.map { false }.toMutableList() }

    val verticalSize = visibleNodes.size
    val horizontalSize = visibleNodes.first().size

    val verticalRange = 0 until verticalSize
    val horizontalRange = 0 until horizontalSize

    runThrough(horizontalRange, verticalRange, this::get, visibleNodes::mark)
    runThrough(horizontalRange, verticalRange.reversed(), this::get, visibleNodes::mark)
    runThrough(verticalRange, horizontalRange, this::invertedGet, visibleNodes::invertedMark)
    runThrough(verticalRange, horizontalRange.reversed(), this::invertedGet, visibleNodes::invertedMark)

    return visibleNodes
}

private fun runThrough(
    range1: IntProgression,
    range2: IntProgression,
    get: (Int, Int) -> Int,
    mark: (Int, Int) -> Unit,
) {
    range1.forEach { dimension1 ->
        var larger = -1
        range2.forEach { dimension2 ->
            val current = get(dimension1, dimension2)
            if (current > larger) {
                mark(dimension1, dimension2)
                larger = current
            }
        }
    }
}

private fun List<List<Int>>.rayTraceFromInside(): Int {
    val verticalSize = size
    val horizontalSize = first().size

    return withIndex().maxOf { (y, row) ->
        row.withIndex().maxOf { (x, content) ->
            val south = ((y + 1) until verticalSize).countInside(content) { y -> get(x, y) }
            val north = ((y - 1) downTo 0).countInside(content) { y -> get(x, y) }

            val east = ((x + 1) until horizontalSize).countInside(content) { x -> get(x, y) }
            val west = ((x - 1) downTo 0).countInside(content) { x -> get(x, y) }

            west * east * north * south
        }
    }
}

private fun IntProgression.countInside(initial: Int, get: (Int) -> Int) =
    takeWhileInclusive { position -> get(position) < initial }.count()

fun List<List<Int>>.get(x: Int, y: Int) = this[y][x]
fun List<MutableList<Boolean>>.mark(x: Int, y: Int) {
    this[y][x] = true
}

fun List<List<Int>>.invertedGet(y: Int, x: Int) = get(x, y)
fun List<MutableList<Boolean>>.invertedMark(y: Int, x: Int) = mark(x, y)

private fun List<String>.getForest() = map { it.map { it.digitToInt() } }
