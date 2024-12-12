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

        println("[Price][$file] ${lines.findPrice()}")
//        println("[Loops  ][$file] ${lines.part2()}")
    }
}

private fun List<List<Char>>.findPrice(): Int {
    val groups = mutableListOf<Group>()

    val allCoords = withIndex().flatMap { (y, line) -> line.indices.map { x -> (x to y) } }

    allCoords
        .asSequence()
        .filter { coord -> groups.none { coord in it.contained } }
        .forEach { coord -> groups += createGroup(coord) }

    return groups.sumOf { (contained, barriers) -> contained.size * barriers.size }
}

private fun List<List<Char>>.part2(): Int {
    return 0
}

private fun List<List<Char>>.createGroup(coord: Coordinate): Group {
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

    return Group(contained, barriers)
}

private fun List<List<Char>>.getAt(position: Coordinate) =
    getOrNull(position.y())?.getOrNull(position.x())

enum class Direction(val diff: Coordinate) {
    N(0 to -1),
    E(1 to 0),
    S(0 to 1),
    W(-1 to 0);
}

private data class Group(val contained: Set<Coordinate>, val barriers: Set<Barrier>)

private class Barrier(from: Coordinate, to: Direction) {
    companion object {
        private val COMPARATOR = Comparator
            .comparingInt(Coordinate::first)
            .thenComparingInt(Coordinate::second)
    }

    val first = minOf(from, from + to.diff, COMPARATOR)
    val second = maxOf(from, from + to.diff, COMPARATOR)

    override fun equals(other: Any?): Boolean {
        if (other !is Barrier) return false
        return first == other.first && second == other.second
    }

    override fun hashCode() = first.hashCode() + second.hashCode()

    override fun toString() = "[$first ~ $second]"
}