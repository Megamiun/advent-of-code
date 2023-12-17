package br.com.gabryel.adventofcode.y2023.d17

import br.com.gabryel.adventofcode.y2023.d17.Direction.*
import br.com.gabryel.adventofcode.y2023.readLines
import java.util.PriorityQueue

fun main() {
    listOf("sample", "input").forEach { file ->
        val lines = readLines(17, file).map { it.map { it.digitToInt() } }

        println("[Minimum Heat Loss - Basic][$file] ${MinimumHeatLoss(lines, 1, 3).minimumHeatLoss}")
        println("[Minimum Heat Loss - Ultra][$file] ${MinimumHeatLoss(lines, 4, 10).minimumHeatLoss}")
    }
}

class MinimumHeatLoss(
    private val map: List<List<Int>>,
    private val minConsecutive: Int, 
    private val maxConsecutive: Int
) {
    
    private val visited = mutableSetOf<String>()

    private val queue = PriorityQueue(compareBy(Path::heatLoss)).also {
        it.addIfViable(0, 1, RIGHT)
        it.addIfViable(1, 0, DOWN)
    }

    val minimumHeatLoss by lazy { calculateMinimumHeatLoss() }

    private fun calculateMinimumHeatLoss(): Int {
        while (queue.isNotEmpty()) {
            val current = queue.remove()
            val x = current.x
            val y = current.y

            if (x == map.lastIndex && y == map.first().lastIndex)
                return current.heatLoss

            val key = "$x;$y;${current.direction};${current.consecutive}"
            if (key in visited) continue
            visited.add(key)

            queue.addIfViable(x, y + 1, RIGHT, current)
            queue.addIfViable(x, y - 1, LEFT, current)
            queue.addIfViable(x + 1, y, DOWN, current)
            queue.addIfViable(x - 1, y, UP, current)
        }

        return -1
    }

    private fun PriorityQueue<Path>.addIfViable(x: Int, y: Int, direction: Direction, previous: Path? = null) {
        if (direction.inverse == previous?.direction) return
        val tileHeatLoss = map.getOrNull(x)?.getOrNull(y) ?: return

        val path = Path(x, y, tileHeatLoss, direction, previous)
        if (path.consecutive > maxConsecutive) return

        val couldPreviousTurn = previous != null && previous.consecutive < minConsecutive
        if (couldPreviousTurn && previous?.direction != direction) return

        add(path)
    }
}

private data class Path(
    val x: Int,
    val y: Int,
    val tileHeatLoss: Int,
    val direction: Direction,
    val previous: Path? = null
) {
    val heatLoss: Int = (previous?.heatLoss ?: 0) + tileHeatLoss
    val consecutive: Int = if (previous?.direction == direction) previous.consecutive + 1 else 1
}

private enum class Direction {
    UP, DOWN, RIGHT, LEFT;

    val inverse by lazy {
        when (this) {
            UP -> DOWN
            DOWN -> UP
            LEFT -> RIGHT
            RIGHT -> LEFT
        }
    }
}
