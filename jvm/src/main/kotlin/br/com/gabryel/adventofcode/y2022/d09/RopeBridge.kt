package br.com.gabryel.adventofcode.y2022.d09

import br.com.gabryel.adventofcode.util.*
import kotlin.math.absoluteValue

private val moveCache = mutableMapOf<Pair<Coordinate, Coordinate>, Coordinate>()

fun getMotionsForKnots(lines: List<String>, size: Int) =
    lines.getCommands()
        .flatMap { (direction, steps) -> (0 until steps).map { direction } }
        .runWithSize(size)

private fun List<Direction>.runWithSize(size: Int): Int {
    val rope = fold(Rope.withSize(size)) { current, direction -> current + direction.vector }
    return rope.getLast().visited.size
}

private fun List<String>.getCommands() = map {
    val (direction, steps) = it.split(" ")
    Direction.findByInitial(direction) to steps.toInt()
}

private sealed class Rope {
    abstract fun follow(previous: Coordinate): Rope

    companion object {
        fun withSize(size: Int): KnotRope =
            if (size <= 1) KnotRope(tail = EmptyRope)
            else KnotRope(tail = withSize(size - 1))
    }
}

private data object EmptyRope : Rope() {
    override fun follow(previous: Coordinate) = this
}

private data class KnotRope(
    val head: Coordinate = ZERO,
    val tail: Rope,
    val visited: Set<Coordinate> = setOf(ZERO)
) : Rope() {

    override fun follow(previous: Coordinate) = this + vectorTo(previous)

    fun getLast(): KnotRope = if (tail is KnotRope) tail.getLast() else this

    operator fun plus(vector: Coordinate): KnotRope {
        if (vector == ZERO) return this

        val newPosition = head + vector
        return copy(head = newPosition, tail = tail.follow(newPosition), visited = visited + newPosition)
    }

    private fun vectorTo(previous: Coordinate) = moveCache.computeIfAbsent(previous to head) {
        val distance = previous distanceTo head
        distance.squaresToMove()
    }

    private fun Coordinate.squaresToMove() =
        if (x().absoluteValue > 1 || y().absoluteValue > 1)
            x().squaresToMove() to y().squaresToMove()
        else ZERO

    private fun Int.squaresToMove() = when {
        this == 0 -> 0
        this > 0 -> 1
        else -> -1
    }
}
