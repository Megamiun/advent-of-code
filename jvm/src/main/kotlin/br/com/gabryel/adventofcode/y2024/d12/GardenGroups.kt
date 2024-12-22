package br.com.gabryel.adventofcode.y2024.d12

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.Coordinate
import br.com.gabryel.adventofcode.util.plus
import br.com.gabryel.adventofcode.util.x
import br.com.gabryel.adventofcode.util.y
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
        .sumOf { region -> region.contained.size * region.findSides().size }

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

enum class Direction(val diff: Coordinate, val parallel: () -> List<Direction>) {
    N(0 to -1, { listOf(W, E) }),
    E(1 to 0, { listOf(N, S) }),
    S(0 to 1, { listOf(W, E) }),
    W(-1 to 0, { listOf(N, S) });
}

private data class Side(val barriers: Set<Barrier>)

private data class Region(val contained: Set<Coordinate>, val barriers: Set<Barrier>) {
    fun findSides(): List<Side> {
        val sides = mutableListOf<Side>()

        barriers
            .asSequence()
            .filter { side -> sides.none { side in it.barriers } }
            .forEach { coord -> sides += captureSides(coord) }
        return sides
    }

    private fun captureSides(barrier: Barrier): Side {
        val inSide = barrier.to.parallel().flatMap { dir ->
            generateSequence(barrier) { it + dir }.takeWhile { it in barriers }
        }.toSet()

        return Side(inSide)
    }
}

private data class Barrier(val first: Coordinate, val to: Direction) {
    operator fun plus(dir: Direction) = Barrier(first + dir.diff, to)
}
