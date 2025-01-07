package br.com.gabryel.adventofcode.y2023.d17

import br.com.gabryel.adventofcode.util.*
import br.com.gabryel.adventofcode.util.Direction.*
import java.util.*

fun getMinimumHeatLoss(groups: List<String>, min: Int, max: Int): Int {
    val map = groups.map { lines -> lines.map { it.digitToInt() }.toIntArray() }.toTypedArray()
    return MinimumHeatLoss(map, min, max).minimumHeatLoss
}

private class MinimumHeatLoss(
    private val map: IntArray2D,
    private val minConsecutive: Int, 
    private val maxConsecutive: Int
) {
    private val end = map[0].lastIndex to map.lastIndex

    val minimumHeatLoss by lazy { calculateMinimumHeatLoss() }

    private fun calculateMinimumHeatLoss(): Int {
        val visited = mutableSetOf<String>()

        val queue = PriorityQueue(compareBy(Path::heatLoss)).apply {
            addPath(0 to 0, RIGHT)
            addPath(0 to 0, DOWN)
        }

        while (queue.isNotEmpty()) {
            val current = queue.remove()

            val position = current.position
            val direction = current.direction

            if (position == end)
                return current.heatLoss

            val key = "$position$direction${current.consecutive}"
            if (key in visited) continue
            visited += key

            if (current.consecutive < maxConsecutive)
                queue.addPath(position, direction, current)

            if (current.consecutive >= minConsecutive) {
                queue.addPath(position, direction.clockwise(), current)
                queue.addPath(position, direction.counterClockwise(), current)
            }
        }

        return -1
    }

    private fun PriorityQueue<Path>.addPath(position: Coordinate, direction: Direction, current: Path? = null) {
        val newPosition = position + direction
        val tileHeatLoss = map.getOrNull(newPosition) ?: return

        add(Path(newPosition, tileHeatLoss, direction, current))
    }
}

private data class Path(
    val position: Coordinate,
    val tileHeatLoss: Int,
    val direction: Direction,
    val previous: Path? = null
) {
    val heatLoss: Int = (previous?.heatLoss ?: 0) + tileHeatLoss
    val consecutive: Int = if (previous?.direction == direction) previous.consecutive + 1 else 1
}
