package br.com.gabryel.adventofcode.y2024.d12

import br.com.gabryel.adventofcode.readLines
import br.com.gabryel.adventofcode.y2022.Coordinate
import br.com.gabryel.adventofcode.y2022.plus
import br.com.gabryel.adventofcode.y2022.x
import br.com.gabryel.adventofcode.y2022.y
import java.util.*

fun main() {
    listOf("sample-small", "sample", "input").forEach { file ->
        val lines = readLines(2024, 12, file)
            .map { it.toCharArray().toList() }

        println("[Price per perimeter][$file] ${lines.findPriceByPerimeter()}")
        println("[Price per side     ][$file] ${lines.findPriceBySides()}")
    }
}

private fun List<List<Char>>.findPriceByPerimeter() =
    findRegions()
        .sumOf { (contained, barriers) -> contained.size * barriers.size }

private fun List<List<Char>>.findPriceBySides() =
    findRegions()
        .sumOf { (contained, barriers) -> contained.size * barriers.findSides().size }

private fun List<List<Char>>.findRegions(): List<Region> {
    val regions = mutableListOf<Region>()
    val allCoords = withIndex().flatMap { (y, line) -> line.indices.map { x -> (x to y) } }

    allCoords
        .asSequence()
        .filter { coord -> regions.none { coord in it.contained } }
        .forEach { coord -> regions += captureRegion(coord) }
    return regions
}

private fun List<List<Char>>.captureRegion(coord: Coordinate): Region {
    val contained = mutableSetOf(coord)
    val barriers = mutableSetOf<Barrier>()

    val toVisit = LinkedList(Direction.entries.map { coord to it })
    val first = getAt(coord)

    while (!toVisit.isEmpty()) {
        val (curr, dir) = toVisit.pop()
        val next = curr + dir.diff

        if (first != getAt(next)) {
            barriers += Barrier(curr, dir)
            continue
        }

        if (next in contained) continue

        contained += next
        toVisit += Direction.entries.map { next to it }
    }

    return Region(contained, barriers)
}

private fun List<List<Char>>.getAt(position: Coordinate) =
    getOrNull(position.y())?.getOrNull(position.x())

private fun Set<Barrier>.findSides(): List<Side> {
    val sides = mutableListOf<Side>()

    asSequence()
        .filter { side -> sides.none { side in it.barriers } }
        .forEach { coord -> sides += captureSides(coord) }
    return sides
}

private fun Set<Barrier>.captureSides(barrier: Barrier): Side {
    val inSide = barrier.to.parallel().flatMap { dir ->
        generateSequence(barrier) { it + dir }.takeWhile { it in this }
    }.toSet()

    return Side(inSide)
}

enum class Direction(val diff: Coordinate, val parallel: () -> List<Direction>) {
    N(0 to -1, { listOf(W, E) }),
    E(1 to 0, { listOf(N, S) }),
    S(0 to 1, { listOf(W, E) }),
    W(-1 to 0, { listOf(N, S) });
}

private data class Side(val barriers: Set<Barrier>)

private data class Region(val contained: Set<Coordinate>, val barriers: Set<Barrier>)

private data class Barrier(val first: Coordinate, val to: Direction) {
    operator fun plus(dir: Direction) = Barrier(first + dir.diff, to)
}
