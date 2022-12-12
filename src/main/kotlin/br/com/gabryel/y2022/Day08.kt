package br.com.gabryel.y2022

fun main() {
    val forest = getForest()

    val visibilityFromOutside = forest.rayTraceFromOutside()
    println("Trees Visible: ${visibilityFromOutside.sumOf { it.count { it } }}")

    val visibilityForEachTree = forest.rayTraceFromInside()
    println("Trees With Best Visibility: ${visibilityForEachTree.maxOf { it.maxOf { it.points } }}")
}

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

private fun List<List<Int>>.rayTraceFromInside(): List<List<TreePosition>> {
    val verticalSize = size - 1
    val horizontalSize = first().size - 1

    return mapIndexed { y, xs ->
        xs.mapIndexed { x, content ->
            var result = TreePosition()

            fun updateResult(range: IntProgression, get: (Int) -> Int, update: TreePosition.() -> TreePosition) =
                runThroughInside(content, range.reversed(), get) { result = result.update() }

            val getY = { y: Int -> get(x, y) }
            updateResult(verticalSize downTo (y + 1), getY) { copy(south = south + 1) }
            updateResult(0 until y, getY) { copy(north = north + 1) }

            val getX = { x: Int -> get(x, y) }
            updateResult(horizontalSize downTo (x + 1), getX) { copy(east = east + 1) }
            updateResult(0 until x, getX) { copy(west = west + 1) }

            result
        }
    }
}

private fun runThroughInside(initial: Int, range: IntProgression, get: (Int) -> Int, mark: () -> Unit) {
    range.forEach { index ->
        mark()
        if (get(index) >= initial) return
    }
}

fun List<List<Int>>.get(x: Int, y: Int) = this[y][x]
fun List<MutableList<Boolean>>.mark(x: Int, y: Int) { this[y][x] = true }

fun List<List<Int>>.invertedGet(y: Int, x: Int) = get(x, y)
fun List<MutableList<Boolean>>.invertedMark(y: Int, x: Int) = mark(x, y)

private fun getForest() = getLines {it.windowed(1).map { it.toInt() } }.toList()

private data class TreePosition(val west: Int = 0, val east: Int = 0, val north: Int = 0, val south: Int = 0) {
    val points = west * east * north * south
}
